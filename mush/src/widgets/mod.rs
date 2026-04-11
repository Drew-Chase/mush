use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::process::{Child, Stdio};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant};

pub mod autocomplete;
pub mod command_history;
pub mod command_input;
pub mod history_popover;

use autocomplete::Autocomplete;
use command_history::{CommandEntry, CommandHistory, LiveOutputBuffer, LiveRenderData};
use command_input::CommandInput;
use history_popover::HistoryPopover;

use crate::config::Config;
use crate::db::HistoryDb;
use crate::shell;
use crate::shell::help_parser::{self, CommandOption};

struct ExecResult {
    output: Vec<String>,
    exit_code: i32,
    exit_app: bool,
}

enum OutputChunk {
    Data(String),
    Error(String),
}

struct RunningCommand {
    command: String,
    buffer: LiveOutputBuffer,
    rx: Receiver<OutputChunk>,
    child: Child,
    start: Instant,
}

struct HelpResult {
    command_prefix: String,
    raw_output: String,
}

struct PendingHelpLookup {
    command_prefix: String,
    rx: Receiver<HelpResult>,
}

const MAX_PENDING_LOOKUPS: usize = 3;

pub struct App {
    pub history: CommandHistory,
    pub input: CommandInput,
    pub autocomplete: Autocomplete,
    pub history_popover: HistoryPopover,
    pub db: HistoryDb,
    exit: bool,
    last_history_area: Rect,
    running_command: Option<RunningCommand>,
    needs_clear: bool,
    help_cache: HashMap<String, Vec<CommandOption>>,
    pending_help_lookups: Vec<PendingHelpLookup>,
    last_help_prefix: Option<String>,
    script_watcher_rx: Receiver<()>,
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(mut running) = self.running_command.take() {
            let _ = running.child.kill();
            let _ = running.child.wait();
        }
    }
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let db_path = Config::get().db_path();
        let db = HistoryDb::open(&db_path)?;
        let help_cache = db.load_all_help().unwrap_or_default();

        let mut history = CommandHistory::default();

        #[cfg(debug_assertions)]
        {
            history.add_entry(CommandEntry {
                command: "cargo.exe --help".to_string(),
                output: vec![
                    "Rust's package manager".to_string(),
                    "".to_string(),
                    "Usage: cargo [+toolchain] [OPTIONS] [COMMAND]".to_string(),
                    "".to_string(),
                    "Options:".to_string(),
                    "  -V, --version  Print version info and exit".to_string(),
                    "  --list         List installed commands".to_string(),
                    "  -h, --help     Print help".to_string(),
                ],
                duration: Duration::from_secs_f64(2.8),
                exit_code: 0,
            });
            history.add_entry(CommandEntry {
                command: "echo hello".to_string(),
                output: vec!["hello".to_string()],
                duration: Duration::from_millis(5),
                exit_code: 0,
            });
        }

        // Spawn filesystem watcher for the scripts directory
        let (script_tx, script_watcher_rx) = mpsc::channel();
        let scripts_dir = crate::get_appdata_path().join("scripts");
        std::thread::spawn(move || {
            use notify::{RecursiveMode, Watcher, recommended_watcher};
            let tx = script_tx;
            let mut watcher = match recommended_watcher(move |res: Result<notify::Event, _>| {
                if res.is_ok() {
                    let _ = tx.send(());
                }
            }) {
                Ok(w) => w,
                Err(_) => return,
            };
            let _ = watcher.watch(&scripts_dir, RecursiveMode::NonRecursive);
            // Block to keep the watcher alive for the lifetime of the app
            loop {
                std::thread::park();
            }
        });

        Ok(Self {
            history,
            input: CommandInput::default(),
            autocomplete: Autocomplete::default(),
            history_popover: HistoryPopover::default(),
            db,
            exit: false,
            last_history_area: Rect::default(),
            running_command: None,
            needs_clear: false,
            help_cache,
            pending_help_lookups: Vec::new(),
            last_help_prefix: None,
            script_watcher_rx,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        self.history.scroll_to_bottom();

        while !self.exit {
            if self.needs_clear {
                terminal.clear()?;
                self.needs_clear = false;
            }

            if self.running_command.is_some() {
                self.drain_running_output();
            }

            if !self.pending_help_lookups.is_empty() {
                self.drain_help_lookups();
            }

            self.drain_script_watcher();

            terminal.draw(|frame| self.draw(frame))?;

            let needs_poll =
                self.running_command.is_some() || !self.pending_help_lookups.is_empty();
            if needs_poll {
                if event::poll(Duration::from_millis(16))? {
                    self.handle_events()?;
                }
            } else {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        self.history.live_entry = self.running_command.as_ref().map(|r| LiveRenderData {
            command: r.command.clone(),
            lines: r.buffer.all_lines().iter().map(|s| s.to_string()).collect(),
            elapsed: r.start.elapsed(),
        });

        let input_height = CommandInput::required_height();
        let popup_height = if self.history_popover.visible {
            self.history_popover.popup_height()
        } else {
            self.autocomplete.popup_height()
        };
        let gap = if popup_height > 0 { 1 } else { 2 };

        let chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(popup_height),
            Constraint::Length(gap),
            Constraint::Length(input_height),
        ])
        .split(area);

        let history_area = chunks[0];
        let popup_area = chunks[1];
        let input_area = chunks[3];

        self.last_history_area = history_area;

        frame.render_widget(&mut self.history, history_area);

        if self.history_popover.visible {
            frame.render_widget(&self.history_popover, popup_area);
        } else if self.autocomplete.visible {
            frame.render_widget(&self.autocomplete, popup_area);
        }

        frame.render_widget(&self.input, input_area);
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            if self.running_command.is_some() {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        self.kill_running_command();
                    }
                    (_, KeyCode::PageUp) | (KeyModifiers::SHIFT, KeyCode::Up) => {
                        self.history_scroll_up();
                    }
                    (_, KeyCode::PageDown) | (KeyModifiers::SHIFT, KeyCode::Down) => {
                        self.history_scroll_down();
                    }
                    _ => {}
                }
                return Ok(());
            }

            match (key.modifiers, key.code) {
                // Quit
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                    self.exit = true;
                }

                // Ctrl+C clears input
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    self.input.clear();
                    self.autocomplete.close();
                    self.history_popover.close();
                    self.validate_input();
                }

                // Ctrl+R toggles history popover
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                    if self.history_popover.visible {
                        self.history_popover.close();
                    } else {
                        self.autocomplete.close();
                        self.history_popover.open(&self.db);
                    }
                }

                // --- History popover active ---
                (_, KeyCode::Esc) if self.history_popover.visible => {
                    self.history_popover.close();
                }
                (KeyModifiers::NONE, KeyCode::Up) if self.history_popover.visible => {
                    self.history_popover.select_up();
                }
                (KeyModifiers::NONE, KeyCode::Down) if self.history_popover.visible => {
                    self.history_popover.select_down();
                }
                (_, KeyCode::Enter) if self.history_popover.visible => {
                    if let Some(cmd) = self.history_popover.accept() {
                        self.input.buffer = cmd;
                        self.input.cursor = self.input.buffer.len();
                        self.validate_input();
                    }
                }
                (_, KeyCode::Tab) if self.history_popover.visible => {
                    if let Some(cmd) = self.history_popover.accept() {
                        self.input.buffer = cmd;
                        self.input.cursor = self.input.buffer.len();
                        self.validate_input();
                    }
                }
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c))
                    if self.history_popover.visible && c >= ' ' =>
                {
                    self.history_popover.insert_char(c);
                }
                (_, KeyCode::Backspace) if self.history_popover.visible => {
                    self.history_popover.backspace();
                }

                // --- Autocomplete active ---
                (_, KeyCode::Esc) => {
                    self.autocomplete.close();
                }
                (_, KeyCode::Tab) => {
                    if let Some(accepted) = self.autocomplete.accept() {
                        self.input.buffer = accepted;
                        self.input.cursor = self.input.buffer.len();
                        self.validate_input();
                    }
                }
                (KeyModifiers::NONE, KeyCode::Up) if self.autocomplete.visible => {
                    self.autocomplete.select_up();
                }
                (KeyModifiers::NONE, KeyCode::Down) if self.autocomplete.visible => {
                    self.autocomplete.select_down();
                }

                // Submit command
                (_, KeyCode::Enter) => {
                    self.autocomplete.close();
                    self.execute_command();
                }

                // Text editing
                (_, KeyCode::Backspace) => {
                    self.input.backspace();
                    self.on_input_changed();
                }
                (_, KeyCode::Delete) => {
                    self.input.delete();
                    self.on_input_changed();
                }
                (_, KeyCode::Left) => self.input.move_left(),
                (_, KeyCode::Right) => self.input.move_right(),
                (_, KeyCode::Home) => self.input.home(),
                (_, KeyCode::End) => self.input.end(),

                // Character input
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) if c >= ' ' => {
                    self.input.insert_char(c);
                    self.on_input_changed();
                }

                // Scroll history
                (KeyModifiers::NONE, KeyCode::Up) => self.history_scroll_up(),
                (KeyModifiers::NONE, KeyCode::Down) => self.history_scroll_down(),
                (_, KeyCode::PageUp) => self.history_scroll_up(),
                (_, KeyCode::PageDown) => self.history_scroll_down(),
                (KeyModifiers::SHIFT, KeyCode::Up) => self.history_scroll_up(),
                (KeyModifiers::SHIFT, KeyCode::Down) => self.history_scroll_down(),

                _ => {}
            }
        }

        Ok(())
    }

    fn execute_command(&mut self) {
        let raw_input = self.input.take_buffer();
        let trimmed = raw_input.trim();
        if trimmed.is_empty() {
            return;
        }

        let command_display = trimmed.to_string();
        let cwd = self.input.cwd.clone();
        let start = Instant::now();

        match shell::resolve_command(trimmed) {
            shell::CommandKind::Builtin(shell::builtins::BuiltinCommand::Clear) => {
                self.history.entries.clear();
                self.history.scroll_to_bottom();
            }
            shell::CommandKind::Alias(commands) => {
                let mut all_output: Vec<String> = Vec::new();
                for cmd in &commands {
                    let result = self.execute_single(cmd);
                    all_output.extend(result.output);
                    if result.exit_app {
                        self.exit = true;
                        break;
                    }
                }
                let duration = start.elapsed();
                if !all_output.is_empty() || !commands.is_empty() {
                    let _ = self.db.insert(
                        &command_display,
                        0,
                        duration.as_millis() as i64,
                        Some(&cwd),
                    );
                    self.history.add_entry(CommandEntry {
                        command: command_display,
                        output: all_output,
                        duration,
                        exit_code: 0,
                    });
                }
            }
            other => {
                if let Some(result) = self.dispatch_resolved(other, &command_display, trimmed) {
                    let duration = start.elapsed();
                    let _ = self.db.insert(
                        &command_display,
                        result.exit_code,
                        duration.as_millis() as i64,
                        Some(&cwd),
                    );
                    self.history.add_entry(CommandEntry {
                        command: command_display,
                        output: result.output,
                        duration,
                        exit_code: result.exit_code,
                    });
                    if result.exit_app {
                        self.exit = true;
                    }
                }
            }
        }

        self.input.valid_command = true;
        self.input.update_cwd();
        self.history.scroll_to_bottom();
    }

    fn execute_single(&mut self, input: &str) -> ExecResult {
        let resolved = shell::resolve_command(input);
        self.dispatch_resolved_sync(resolved, input)
    }

    fn dispatch_resolved(
        &mut self,
        kind: shell::CommandKind,
        command_display: &str,
        input: &str,
    ) -> Option<ExecResult> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let args = if parts.len() > 1 { &parts[1..] } else { &[] };

        match kind {
            shell::CommandKind::External(ref path)
                if !parts.is_empty() && shell::is_interactive(parts[0], args) =>
            {
                Some(self.run_interactive(path, args))
            }
            shell::CommandKind::External(path) => {
                match std::process::Command::new(&path)
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        self.spawn_streaming(command_display, child);
                        None
                    }
                    Err(e) => Some(ExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                    }),
                }
            }
            shell::CommandKind::Script(entry) => {
                let bun_path = match shell::path_resolver::find_in_path("bun") {
                    Some(p) => p,
                    None => {
                        return Some(ExecResult {
                            output: vec![
                                format!(
                                    "error: script '{}' requires Bun but 'bun' was not found on PATH.",
                                    entry.name
                                ),
                                "Install Bun from https://bun.sh or re-run the Mush installer with the Bun option."
                                    .to_string(),
                            ],
                            exit_code: 1,
                            exit_app: false,
                        });
                    }
                };
                match std::process::Command::new(&bun_path)
                    .arg("run")
                    .arg(&entry.entry_point)
                    .args(args)
                    .current_dir(&entry.script_dir)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        self.spawn_streaming(command_display, child);
                        None
                    }
                    Err(e) => Some(ExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                    }),
                }
            }
            other => Some(self.dispatch_resolved_sync(other, input)),
        }
    }

    fn dispatch_resolved_sync(&mut self, kind: shell::CommandKind, input: &str) -> ExecResult {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let args = if parts.len() > 1 { &parts[1..] } else { &[] };

        match kind {
            shell::CommandKind::Builtin(cmd) => {
                let result = shell::builtins::execute(cmd, args);
                if result.change_dir.is_some() {
                    self.input.update_cwd();
                }
                ExecResult {
                    output: result.output,
                    exit_code: 0,
                    exit_app: result.exit_app,
                }
            }
            shell::CommandKind::External(path) => {
                match std::process::Command::new(&path).args(args).output() {
                    Ok(out) => {
                        let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
                            .lines()
                            .map(String::from)
                            .collect();
                        let stderr_lines: Vec<String> = String::from_utf8_lossy(&out.stderr)
                            .lines()
                            .map(String::from)
                            .collect();
                        lines.extend(stderr_lines);
                        ExecResult {
                            output: lines,
                            exit_code: out.status.code().unwrap_or(-1),
                            exit_app: false,
                        }
                    }
                    Err(e) => ExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                    },
                }
            }
            shell::CommandKind::Alias(commands) => {
                let mut all_output = Vec::new();
                let mut exit_app = false;
                for cmd in &commands {
                    let result = self.execute_single(cmd);
                    all_output.extend(result.output);
                    if result.exit_app {
                        exit_app = true;
                        break;
                    }
                }
                ExecResult {
                    output: all_output,
                    exit_code: 0,
                    exit_app,
                }
            }
            shell::CommandKind::Script(entry) => {
                let bun_path = match shell::path_resolver::find_in_path("bun") {
                    Some(p) => p,
                    None => {
                        return ExecResult {
                            output: vec![
                                format!(
                                    "error: script '{}' requires Bun but 'bun' was not found on PATH.",
                                    entry.name
                                ),
                                "Install Bun from https://bun.sh or re-run the Mush installer with the Bun option."
                                    .to_string(),
                            ],
                            exit_code: 1,
                            exit_app: false,
                        };
                    }
                };
                match std::process::Command::new(&bun_path)
                    .arg("run")
                    .arg(&entry.entry_point)
                    .args(args)
                    .current_dir(&entry.script_dir)
                    .output()
                {
                    Ok(out) => {
                        let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
                            .lines()
                            .map(String::from)
                            .collect();
                        let stderr_lines: Vec<String> = String::from_utf8_lossy(&out.stderr)
                            .lines()
                            .map(String::from)
                            .collect();
                        lines.extend(stderr_lines);
                        ExecResult {
                            output: lines,
                            exit_code: out.status.code().unwrap_or(-1),
                            exit_app: false,
                        }
                    }
                    Err(e) => ExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                    },
                }
            }
            shell::CommandKind::NotFound => {
                let name = input.split_whitespace().next().unwrap_or(input);
                ExecResult {
                    output: vec![format!("command not found: {name}")],
                    exit_code: 127,
                    exit_app: false,
                }
            }
        }
    }

    fn spawn_streaming(&mut self, command_display: &str, mut child: Child) {
        let (tx, rx) = mpsc::channel::<OutputChunk>();

        if let Some(stdout) = child.stdout.take() {
            let tx_out = tx.clone();
            std::thread::spawn(move || {
                let mut reader = std::io::BufReader::new(stdout);
                let mut buf = [0u8; 4096];
                loop {
                    match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let text = String::from_utf8_lossy(&buf[..n]).to_string();
                            if tx_out.send(OutputChunk::Data(text)).is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            let _ = tx_out.send(OutputChunk::Error(e.to_string()));
                            break;
                        }
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let tx_err = tx.clone();
            std::thread::spawn(move || {
                let mut reader = std::io::BufReader::new(stderr);
                let mut buf = [0u8; 4096];
                loop {
                    match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let text = String::from_utf8_lossy(&buf[..n]).to_string();
                            if tx_err.send(OutputChunk::Data(text)).is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            let _ = tx_err.send(OutputChunk::Error(e.to_string()));
                            break;
                        }
                    }
                }
            });
        }

        drop(tx);

        self.running_command = Some(RunningCommand {
            command: command_display.to_string(),
            buffer: LiveOutputBuffer::new(),
            rx,
            child,
            start: Instant::now(),
        });
    }

    fn run_interactive(&mut self, path: &std::path::Path, args: &[&str]) -> ExecResult {
        use ratatui::crossterm::ExecutableCommand;
        use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

        let _ = terminal::disable_raw_mode();
        let _ = std::io::stdout().execute(LeaveAlternateScreen);

        let status = std::process::Command::new(path).args(args).status();

        let _ = std::io::stdout().execute(EnterAlternateScreen);
        let _ = terminal::enable_raw_mode();
        self.needs_clear = true;

        match status {
            Ok(s) => ExecResult {
                output: Vec::new(),
                exit_code: s.code().unwrap_or(-1),
                exit_app: false,
            },
            Err(e) => ExecResult {
                output: vec![format!("error: {e}")],
                exit_code: -1,
                exit_app: false,
            },
        }
    }

    fn drain_running_output(&mut self) {
        let running = match &mut self.running_command {
            Some(r) => r,
            None => return,
        };

        loop {
            match running.rx.try_recv() {
                Ok(OutputChunk::Data(text)) => {
                    running.buffer.push(&text);
                }
                Ok(OutputChunk::Error(msg)) => {
                    running.buffer.push(&format!("[error: {msg}]"));
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.finalize_running_command();
                    return;
                }
            }
        }
    }

    fn finalize_running_command(&mut self) {
        if let Some(mut running) = self.running_command.take() {
            let exit_code = match running.child.wait() {
                Ok(status) => status.code().unwrap_or(-1),
                Err(_) => -1,
            };
            let duration = running.start.elapsed();
            let output = running.buffer.into_lines();

            let _ = self.db.insert(
                &running.command,
                exit_code,
                duration.as_millis() as i64,
                Some(&self.input.cwd),
            );

            self.history.add_entry(CommandEntry {
                command: running.command,
                output,
                duration,
                exit_code,
            });

            self.input.update_cwd();
            self.history.scroll_to_bottom();
        }
    }

    fn kill_running_command(&mut self) {
        if let Some(mut running) = self.running_command.take() {
            let _ = running.child.kill();
            let _ = running.child.wait();
            let duration = running.start.elapsed();
            let mut output = running.buffer.into_lines();
            output.push("^C".to_string());

            let _ = self.db.insert(
                &running.command,
                130,
                duration.as_millis() as i64,
                Some(&self.input.cwd),
            );

            self.history.add_entry(CommandEntry {
                command: running.command,
                output,
                duration,
                exit_code: 130,
            });

            self.input.update_cwd();
            self.history.scroll_to_bottom();
        }
    }

    fn on_input_changed(&mut self) {
        self.validate_input();

        let input = self.input.buffer.clone();
        let has_space = input.contains(' ');

        if has_space {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            let ends_with_space = input.ends_with(' ');

            let (prefix_tokens, partial) = if ends_with_space {
                (tokens.as_slice(), "")
            } else if tokens.len() > 1 {
                (&tokens[..tokens.len() - 1], tokens[tokens.len() - 1])
            } else {
                self.autocomplete.update(&input);
                return;
            };

            let prefix = prefix_tokens.join(" ");

            if !prefix.is_empty() {
                let should_spawn = match &self.last_help_prefix {
                    Some(last) => *last != prefix,
                    None => true,
                };

                if should_spawn {
                    self.last_help_prefix = Some(prefix.clone());
                    self.spawn_help_lookup(prefix.clone());
                }

                self.autocomplete
                    .update_with_help(partial, self.help_cache.get(&prefix), &prefix);
            } else {
                self.autocomplete.update(&input);
            }
        } else {
            self.last_help_prefix = None;
            self.autocomplete.update(&input);
        }
    }

    fn spawn_help_lookup(&mut self, command_prefix: String) {
        if self.pending_help_lookups.len() >= MAX_PENDING_LOOKUPS {
            return;
        }

        if self
            .pending_help_lookups
            .iter()
            .any(|p| p.command_prefix == command_prefix)
        {
            return;
        }

        let parts: Vec<&str> = command_prefix.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let base_cmd = parts[0];

        if shell::builtins::lookup(base_cmd).is_some() {
            return;
        }

        let path = match shell::path_resolver::find_in_path(base_cmd) {
            Some(p) => p,
            None => return,
        };

        let sub_args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        let prefix_clone = command_prefix.clone();

        let (tx, rx) = mpsc::channel::<HelpResult>();

        std::thread::spawn(move || {
            let try_help = |flag: &str| -> Option<String> {
                let mut child = std::process::Command::new(&path)
                    .args(&sub_args)
                    .arg(flag)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .ok()?;

                let start = Instant::now();
                loop {
                    match child.try_wait() {
                        Ok(Some(_)) => break,
                        Ok(None) => {
                            if start.elapsed() > Duration::from_secs(3) {
                                let _ = child.kill();
                                let _ = child.wait();
                                return None;
                            }
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(_) => return None,
                    }
                }

                let mut stdout = String::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut stdout);
                }
                let mut stderr = String::new();
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_string(&mut stderr);
                }

                let result = if stdout.len() >= stderr.len() {
                    stdout
                } else {
                    stderr
                };

                if result.len() > 20 {
                    Some(result)
                } else {
                    None
                }
            };

            let raw = try_help("--help")
                .or_else(|| try_help("-h"))
                .or_else(|| try_help("-?"))
                .or_else(|| try_help("?"))
                .or_else(|| try_help("/?"));

            if let Some(raw) = raw {
                let _ = tx.send(HelpResult {
                    raw_output: raw,
                    command_prefix: prefix_clone,
                });
            }
        });

        self.pending_help_lookups
            .push(PendingHelpLookup { command_prefix, rx });
    }

    fn drain_help_lookups(&mut self) {
        let mut completed = Vec::new();

        let mut i = 0;
        while i < self.pending_help_lookups.len() {
            match self.pending_help_lookups[i].rx.try_recv() {
                Ok(result) => {
                    self.pending_help_lookups.remove(i);
                    completed.push(result);
                }
                Err(TryRecvError::Empty) => {
                    i += 1;
                }
                Err(TryRecvError::Disconnected) => {
                    self.pending_help_lookups.remove(i);
                }
            }
        }

        for result in &completed {
            if result.raw_output.is_empty() {
                continue;
            }

            let new_hash = compute_help_hash(&result.raw_output);
            let options = help_parser::parse_help_output(&result.raw_output);

            if options.is_empty() {
                continue;
            }

            let should_write = match self.db.get_help_hash(&result.command_prefix) {
                Ok(Some(existing_hash)) => existing_hash != new_hash,
                _ => true,
            };

            if should_write {
                let _ = self
                    .db
                    .upsert_help(&result.command_prefix, &options, &new_hash);
            }

            self.help_cache
                .insert(result.command_prefix.clone(), options);
        }

        if !completed.is_empty() {
            self.refresh_autocomplete();
        }
    }

    fn drain_script_watcher(&mut self) {
        let mut changed = false;
        while self.script_watcher_rx.try_recv().is_ok() {
            changed = true;
        }
        if changed {
            let scripts_dir = crate::get_appdata_path().join("scripts");
            shell::script_registry::scan_scripts(&scripts_dir);
            self.input.notify("Scripts reloaded".to_string());
        }
    }

    fn refresh_autocomplete(&mut self) {
        let input = &self.input.buffer;
        if !input.contains(' ') {
            return;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let ends_with_space = input.ends_with(' ');

        let (prefix_tokens, partial) = if ends_with_space {
            (tokens.as_slice(), "")
        } else if tokens.len() > 1 {
            (&tokens[..tokens.len() - 1], tokens[tokens.len() - 1])
        } else {
            return;
        };

        let prefix = prefix_tokens.join(" ");
        if !prefix.is_empty() {
            self.autocomplete
                .update_with_help(partial, self.help_cache.get(&prefix), &prefix);
        }
    }

    fn validate_input(&mut self) {
        self.input.valid_command = shell::is_valid_command(&self.input.buffer);
    }

    fn history_scroll_up(&mut self) {
        self.history.scroll_up(
            3,
            self.last_history_area.height,
            self.last_history_area.width,
        );
    }

    fn history_scroll_down(&mut self) {
        self.history.scroll_down(3);
    }
}

fn compute_help_hash(text: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
