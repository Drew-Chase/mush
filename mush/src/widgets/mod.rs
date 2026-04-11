use ratatui::crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind,
};
use ratatui::crossterm::{execute, event::{EnableMouseCapture, DisableMouseCapture, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags}};
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
pub mod history_navigator;
pub mod history_popover;

use autocomplete::Autocomplete;
use command_history::{CommandEntry, CommandHistory, LiveOutputBuffer, LiveRenderData};
use command_input::CommandInput;
use history_navigator::HistoryNavigator;
use history_popover::HistoryPopover;

use crate::config::Config;
use crate::db::HistoryDb;
use crate::shell;
use crate::shell::help_parser::{self, CommandOption};

struct ExecResult {
    output: Vec<String>,
    exit_code: i32,
}

enum OutputChunk {
    Data(String),
    Error(String),
}

struct RunningPipeline {
    command: String,
    buffer: LiveOutputBuffer,
    rx: Receiver<OutputChunk>,
    /// All child processes in the pipeline (for cleanup). The last child is separate.
    pipeline_children: Vec<Child>,
    last_child: Child,
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

struct BackgroundJob {
    job_id: u32,
    command: String,
    children: Vec<Child>,
    last_child: Child,
    start: Instant,
}

pub struct App {
    pub history: CommandHistory,
    pub input: CommandInput,
    pub autocomplete: Autocomplete,
    pub history_popover: HistoryPopover,
    pub history_nav: HistoryNavigator,
    pub db: &'static HistoryDb,
    exit: bool,
    last_history_area: Rect,
    running_pipeline: Option<RunningPipeline>,
    last_exit_code: i32,
    background_jobs: Vec<BackgroundJob>,
    next_job_id: u32,
    needs_clear: bool,
    help_cache: HashMap<String, Vec<CommandOption>>,
    pending_help_lookups: Vec<PendingHelpLookup>,
    last_help_prefix: Option<String>,
    script_watcher_rx: Receiver<()>,
    config_watcher_rx: Receiver<()>,
    pending_path_scan: Option<Receiver<Vec<String>>>,
    last_path_scan: Option<Instant>,
    interactive_mode: bool,
    pipe_output_cache: Option<(String, Vec<String>)>,
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(mut running) = self.running_pipeline.take() {
            for mut c in running.pipeline_children.drain(..) {
                let _ = c.kill();
                let _ = c.wait();
            }
            let _ = running.last_child.kill();
            let _ = running.last_child.wait();
        }
        for mut job in self.background_jobs.drain(..) {
            for mut c in job.children.drain(..) {
                let _ = c.kill();
                let _ = c.wait();
            }
            let _ = job.last_child.kill();
            let _ = job.last_child.wait();
        }
    }
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let db_path = Config::get().db_path();
        // Initialize global DB (ignore error if already initialized, e.g. in tests)
        let _ = HistoryDb::init_global(&db_path);
        let db = HistoryDb::global();
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

        // Spawn filesystem watcher for the config file
        let (config_tx, config_watcher_rx) = mpsc::channel();
        let config_dir = crate::get_appdata_path();
        std::thread::spawn(move || {
            use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
            let tx = config_tx;
            let mut watcher = match recommended_watcher(move |res: Result<notify::Event, _>| {
                if let Ok(event) = res
                    && matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
                    && event.paths.iter().any(|p| {
                        p.file_name()
                            .is_some_and(|name| name == "config.toml")
                    })
                {
                    let _ = tx.send(());
                }
            }) {
                Ok(w) => w,
                Err(_) => return,
            };
            let _ = watcher.watch(&config_dir, RecursiveMode::NonRecursive);
            loop {
                std::thread::park();
            }
        });

        Ok(Self {
            history,
            input: CommandInput::default(),
            autocomplete: Autocomplete::default(),
            history_popover: HistoryPopover::default(),
            history_nav: HistoryNavigator::default(),
            db,
            exit: false,
            last_history_area: Rect::default(),
            running_pipeline: None,
            last_exit_code: 0,
            background_jobs: Vec::new(),
            next_job_id: 1,
            needs_clear: false,
            help_cache,
            pending_help_lookups: Vec::new(),
            last_help_prefix: None,
            script_watcher_rx,
            config_watcher_rx,
            pending_path_scan: None,
            last_path_scan: None,
            interactive_mode: false,
            pipe_output_cache: None,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        execute!(std::io::stdout(), EnableMouseCapture)?;
        let _ = execute!(
            std::io::stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        );
        self.history.scroll_to_bottom();

        while !self.exit {
            if self.needs_clear {
                terminal.clear()?;
                self.needs_clear = false;
            }

            if self.running_pipeline.is_some() {
                self.drain_running_output();
            }

            if !self.pending_help_lookups.is_empty() {
                self.drain_help_lookups();
            }

            self.drain_script_watcher();
            self.drain_config_watcher();
            self.drain_background_jobs();
            self.drain_path_scan();

            terminal.draw(|frame| self.draw(frame))?;

            let needs_poll = self.running_pipeline.is_some()
                || !self.pending_help_lookups.is_empty()
                || self.pending_path_scan.is_some()
                || !self.background_jobs.is_empty();
            if needs_poll {
                if event::poll(Duration::from_millis(16))? {
                    self.handle_events()?;
                }
            } else {
                self.handle_events()?;
            }
        }

        let _ = execute!(std::io::stdout(), PopKeyboardEnhancementFlags);
        execute!(std::io::stdout(), DisableMouseCapture)?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        self.history.live_entry = self.running_pipeline.as_ref().map(|r| LiveRenderData {
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
        match event::read()? {
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return Ok(());
                }

                if self.running_pipeline.is_some() {
                    match (key.modifiers, key.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                            self.kill_running_command();
                        }
                        (_, KeyCode::PageUp)
                        | (KeyModifiers::SHIFT, KeyCode::Up)
                        | (KeyModifiers::CONTROL, KeyCode::Up) => {
                            self.history_scroll_up();
                        }
                        (_, KeyCode::PageDown)
                        | (KeyModifiers::SHIFT, KeyCode::Down)
                        | (KeyModifiers::CONTROL, KeyCode::Down) => {
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
                        self.history_nav.reset();
                        self.validate_input();
                    }

                    // Ctrl+I toggles interactive mode
                    (KeyModifiers::CONTROL, KeyCode::Char('i')) => {
                        self.interactive_mode = !self.interactive_mode;
                        self.input.interactive_mode = self.interactive_mode;
                        let msg = if self.interactive_mode {
                            "Interactive mode ON"
                        } else {
                            "Interactive mode OFF"
                        };
                        self.input.notify(msg.to_string());
                    }

                    // Ctrl+R toggles history popover
                    (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                        if self.history_popover.visible {
                            self.history_popover.close();
                        } else {
                            self.autocomplete.close();
                            self.history_popover.open(self.db);
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
                            self.on_input_changed();
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

                    // Arrow-key command history navigation
                    (KeyModifiers::NONE, KeyCode::Up) => self.history_nav_up(),
                    (KeyModifiers::NONE, KeyCode::Down) => self.history_nav_down(),

                    // Scroll output area
                    (KeyModifiers::CONTROL, KeyCode::Up) => self.history_scroll_up(),
                    (KeyModifiers::CONTROL, KeyCode::Down) => self.history_scroll_down(),
                    (_, KeyCode::PageUp) => self.history_scroll_up(),
                    (_, KeyCode::PageDown) => self.history_scroll_down(),
                    (KeyModifiers::SHIFT, KeyCode::Up) => self.history_scroll_up(),
                    (KeyModifiers::SHIFT, KeyCode::Down) => self.history_scroll_down(),

                    _ => {}
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.history_scroll_up(),
                MouseEventKind::ScrollDown => self.history_scroll_down(),
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    fn execute_command(&mut self) {
        self.history_nav.invalidate();
        let raw_input = self.input.take_buffer();
        let trimmed = raw_input.trim();
        if trimmed.is_empty() {
            return;
        }

        let command_display = trimmed.to_string();
        let cwd = self.input.cwd.clone();

        // Parse the command line into an AST
        let command_line = match shell::parser::parse(trimmed) {
            Ok(cl) => cl,
            Err(e) => {
                self.history.add_entry(CommandEntry {
                    command: command_display,
                    output: vec![format!("parse error: {e}")],
                    duration: Duration::ZERO,
                    exit_code: 1,
                });
                self.last_exit_code = 1;
                self.input.valid_command = true;
                self.input.update_cwd();
                self.history.scroll_to_bottom();
                return;
            }
        };

        if command_line.chains.is_empty() {
            return;
        }

        // Expand variables ($VAR, ${VAR}, $?), globs, command substitution, process substitution
        let mut env = shell::expand::ShellEnv {
            last_exit_code: self.last_exit_code,
            temp_files: Vec::new(),
        };
        let command_line = match shell::expand::expand(&command_line, &mut env) {
            Ok(cl) => cl,
            Err(e) => {
                self.history.add_entry(CommandEntry {
                    command: command_display,
                    output: vec![format!("{e}")],
                    duration: Duration::ZERO,
                    exit_code: 1,
                });
                self.last_exit_code = 1;
                self.input.valid_command = true;
                self.input.update_cwd();
                self.history.scroll_to_bottom();
                return;
            }
        };

        // Execute each chain in the command line (separated by ;)
        for chain in &command_line.chains {
            self.execute_chain(chain, &command_display, &cwd);
            if self.exit {
                break;
            }
        }

        // Clean up temp files from process substitution
        for path in &env.temp_files {
            let _ = std::fs::remove_file(path);
        }

        self.input.valid_command = true;
        self.input.update_cwd();
        self.history.scroll_to_bottom();
    }

    fn execute_chain(
        &mut self,
        chain: &shell::ast::Chain,
        command_display: &str,
        cwd: &str,
    ) {
        if chain.background {
            self.execute_chain_background(chain, command_display, cwd);
            return;
        }

        self.execute_pipeline_in_chain(&chain.first, command_display, cwd);

        for (op, pipeline) in &chain.rest {
            // If there's a streaming command running, we can't chain yet.
            // For && and || chaining, pipelines must complete synchronously.
            // If the previous pipeline was streaming, wait for it.
            if self.running_pipeline.is_some() {
                self.wait_for_running_pipeline();
            }

            let should_run = match op {
                shell::ast::ChainOp::And => self.last_exit_code == 0,
                shell::ast::ChainOp::Or => self.last_exit_code != 0,
            };
            if should_run {
                self.execute_pipeline_in_chain(pipeline, command_display, cwd);
            }
        }
    }

    fn execute_chain_background(
        &mut self,
        chain: &shell::ast::Chain,
        command_display: &str,
        cwd: &str,
    ) {
        let start = Instant::now();

        // For background execution, use the pipeline executor but capture the
        // streaming spawn instead of streaming to TUI.
        match shell::pipeline::execute_pipeline(&chain.first, false) {
            shell::pipeline::PipelineResult::Streaming(spawn) => {
                let job_id = self.next_job_id;
                self.next_job_id += 1;

                #[cfg(unix)]
                let pid = {
                    use std::os::unix::process::CommandExt;
                    spawn.last_child.id()
                };
                #[cfg(not(unix))]
                let pid = spawn.last_child.id();

                self.history.add_entry(CommandEntry {
                    command: command_display.to_string(),
                    output: vec![format!("[{job_id}] {pid}")],
                    duration: Duration::ZERO,
                    exit_code: 0,
                });

                self.background_jobs.push(BackgroundJob {
                    job_id,
                    command: command_display.to_string(),
                    children: spawn.children,
                    last_child: spawn.last_child,
                    start,
                });

                self.last_exit_code = 0;
            }
            shell::pipeline::PipelineResult::Sync(result) => {
                // If the command ran synchronously (builtin, etc.), just show output
                let duration = start.elapsed();
                self.last_exit_code = result.exit_code;
                let _ = self.db.insert(
                    command_display,
                    result.exit_code,
                    duration.as_millis() as i64,
                    Some(cwd),
                );
                if !result.output.is_empty() || result.exit_code != 0 {
                    self.history.add_entry(CommandEntry {
                        command: command_display.to_string(),
                        output: result.output,
                        duration,
                        exit_code: result.exit_code,
                    });
                }
                if result.exit_app {
                    self.exit = true;
                }
            }
            shell::pipeline::PipelineResult::Interactive { path, args } => {
                // Can't run interactive commands in background
                let result = self.run_interactive(&path, &args);
                self.last_exit_code = result.exit_code;
            }
        }
    }

    fn execute_pipeline_in_chain(
        &mut self,
        pipeline: &shell::ast::Pipeline,
        command_display: &str,
        cwd: &str,
    ) {
        let start = Instant::now();

        // Check for clear builtin as a special case (needs to clear history widget)
        if pipeline.subshell.is_none() && pipeline.commands.len() == 1 {
            let first_word = pipeline.commands[0]
                .words
                .first()
                .map(|w| w.to_plain_string());
            if let Some(ref name) = first_word
                && (name == "clear" || name == "cls")
            {
                self.history.entries.clear();
                self.history.scroll_to_bottom();
                self.last_exit_code = 0;
                return;
            }

            // Job control builtins need App access
            if let Some(ref name) = first_word {
                let cmd_args: Vec<String> = pipeline.commands[0]
                    .words
                    .iter()
                    .skip(1)
                    .map(|w| w.to_plain_string())
                    .collect();
                match name.as_str() {
                    "jobs" => {
                        self.execute_jobs_builtin(&cmd_args, command_display, start);
                        return;
                    }
                    "fg" => {
                        self.execute_fg_builtin(&cmd_args, command_display, start);
                        return;
                    }
                    "bg" => {
                        self.execute_bg_builtin(&cmd_args, command_display, start);
                        return;
                    }
                    "wait" => {
                        self.execute_wait_builtin(&cmd_args, command_display, start);
                        return;
                    }
                    _ => {}
                }
            }
        }

        match shell::pipeline::execute_pipeline(pipeline, self.interactive_mode) {
            shell::pipeline::PipelineResult::Sync(result) => {
                let duration = start.elapsed();
                self.last_exit_code = result.exit_code;
                let _ = self.db.insert(
                    command_display,
                    result.exit_code,
                    duration.as_millis() as i64,
                    Some(cwd),
                );
                if !result.output.is_empty() || result.exit_code != 0 {
                    self.history.add_entry(CommandEntry {
                        command: command_display.to_string(),
                        output: result.output,
                        duration,
                        exit_code: result.exit_code,
                    });
                }
                if result.change_dir {
                    self.input.update_cwd();
                }
                if result.exit_app {
                    self.exit = true;
                }
            }
            shell::pipeline::PipelineResult::Streaming(spawn) => {
                self.spawn_streaming_pipeline(command_display, spawn);
            }
            shell::pipeline::PipelineResult::Interactive { path, args } => {
                let result = self.run_interactive(&path, &args);
                let duration = start.elapsed();
                self.last_exit_code = result.exit_code;
                let _ = self.db.insert(
                    command_display,
                    result.exit_code,
                    duration.as_millis() as i64,
                    Some(cwd),
                );
                if !result.output.is_empty() {
                    self.history.add_entry(CommandEntry {
                        command: command_display.to_string(),
                        output: result.output,
                        duration,
                        exit_code: result.exit_code,
                    });
                }
            }
        }
    }

    /// Wait for a running streaming pipeline to complete (blocking).
    fn execute_jobs_builtin(&mut self, args: &[String], command_display: &str, start: Instant) {
        let show_pid = args.iter().any(|a| a == "-l" || a == "--long");
        let mut output = Vec::new();

        for job in &self.background_jobs {
            if show_pid {
                let pid = job.last_child.id();
                output.push(format!("[{}]  {}  Running  {}", job.job_id, pid, job.command));
            } else {
                output.push(format!("[{}]  Running  {}", job.job_id, job.command));
            }
        }

        if output.is_empty() {
            output.push("No background jobs.".to_string());
        }

        let duration = start.elapsed();
        self.last_exit_code = 0;
        self.history.add_entry(CommandEntry {
            command: command_display.to_string(),
            output,
            duration,
            exit_code: 0,
        });
    }

    fn execute_fg_builtin(&mut self, args: &[String], command_display: &str, start: Instant) {
        // Parse job spec: %N, N, or default (last job)
        let job_id = if let Some(spec) = args.first() {
            let s = spec.strip_prefix('%').unwrap_or(spec);
            s.parse::<u32>().ok()
        } else {
            self.background_jobs.last().map(|j| j.job_id)
        };

        let job_id = match job_id {
            Some(id) => id,
            None => {
                let duration = start.elapsed();
                self.last_exit_code = 1;
                self.history.add_entry(CommandEntry {
                    command: command_display.to_string(),
                    output: vec!["fg: no current job".to_string()],
                    duration,
                    exit_code: 1,
                });
                return;
            }
        };

        let idx = self.background_jobs.iter().position(|j| j.job_id == job_id);
        match idx {
            Some(i) => {
                let job = self.background_jobs.remove(i);
                let spawn = shell::pipeline::StreamingSpawn {
                    children: job.children,
                    last_child: job.last_child,
                    stdin_data: None,
                };
                self.history.add_entry(CommandEntry {
                    command: command_display.to_string(),
                    output: vec![format!("[{}] {} brought to foreground", job.job_id, job.command)],
                    duration: start.elapsed(),
                    exit_code: 0,
                });
                self.spawn_streaming_pipeline(&job.command, spawn);
            }
            None => {
                let duration = start.elapsed();
                self.last_exit_code = 1;
                self.history.add_entry(CommandEntry {
                    command: command_display.to_string(),
                    output: vec![format!("fg: %{job_id}: no such job")],
                    duration,
                    exit_code: 1,
                });
            }
        }
    }

    fn execute_bg_builtin(&mut self, args: &[String], command_display: &str, start: Instant) {
        let job_id = if let Some(spec) = args.first() {
            let s = spec.strip_prefix('%').unwrap_or(spec);
            s.parse::<u32>().ok()
        } else {
            self.background_jobs.last().map(|j| j.job_id)
        };

        let output = match job_id {
            Some(id) => {
                if let Some(job) = self.background_jobs.iter().find(|j| j.job_id == id) {
                    vec![format!("[{}] {} &", job.job_id, job.command)]
                } else {
                    self.last_exit_code = 1;
                    vec![format!("bg: %{id}: no such job")]
                }
            }
            None => {
                self.last_exit_code = 1;
                vec!["bg: no current job".to_string()]
            }
        };

        let duration = start.elapsed();
        self.history.add_entry(CommandEntry {
            command: command_display.to_string(),
            output,
            duration,
            exit_code: self.last_exit_code,
        });
    }

    fn execute_wait_builtin(&mut self, args: &[String], command_display: &str, start: Instant) {
        let mut output = Vec::new();
        let mut last_exit_code = 0;

        if args.is_empty() {
            // Wait for all background jobs
            while let Some(mut job) = self.background_jobs.pop() {
                let exit_code = match job.last_child.wait() {
                    Ok(status) => status.code().unwrap_or(-1),
                    Err(_) => -1,
                };
                for mut c in job.children.drain(..) {
                    let _ = c.wait();
                }
                let status_text = if exit_code == 0 { "Done" } else { "Exit" };
                output.push(format!("[{}] {}  {}", job.job_id, status_text, job.command));
                last_exit_code = exit_code;
            }
        } else {
            // Wait for specific jobs
            for spec in args {
                let s = spec.strip_prefix('%').unwrap_or(spec);
                if let Ok(id) = s.parse::<u32>() {
                    if let Some(idx) = self.background_jobs.iter().position(|j| j.job_id == id) {
                        let mut job = self.background_jobs.remove(idx);
                        let exit_code = match job.last_child.wait() {
                            Ok(status) => status.code().unwrap_or(-1),
                            Err(_) => -1,
                        };
                        for mut c in job.children.drain(..) {
                            let _ = c.wait();
                        }
                        let status_text = if exit_code == 0 { "Done" } else { "Exit" };
                        output.push(format!("[{}] {}  {}", job.job_id, status_text, job.command));
                        last_exit_code = exit_code;
                    } else {
                        output.push(format!("wait: %{id}: no such job"));
                        last_exit_code = 127;
                    }
                }
            }
        }

        if output.is_empty() {
            output.push("No background jobs to wait for.".to_string());
        }

        self.last_exit_code = last_exit_code;
        let duration = start.elapsed();
        self.history.add_entry(CommandEntry {
            command: command_display.to_string(),
            output,
            duration,
            exit_code: last_exit_code,
        });
    }

    fn wait_for_running_pipeline(&mut self) {
        if let Some(mut running) = self.running_pipeline.take() {
            // Drain remaining output
            loop {
                match running.rx.try_recv() {
                    Ok(OutputChunk::Data(text)) => running.buffer.push(&text),
                    Ok(OutputChunk::Error(msg)) => running.buffer.push(&format!("[error: {msg}]")),
                    Err(TryRecvError::Empty) => {
                        std::thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Err(TryRecvError::Disconnected) => break,
                }
            }

            let exit_code = match running.last_child.wait() {
                Ok(status) => status.code().unwrap_or(-1),
                Err(_) => -1,
            };
            for mut c in running.pipeline_children {
                let _ = c.wait();
            }

            let duration = running.start.elapsed();
            let output = running.buffer.into_lines();

            self.last_exit_code = exit_code;
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

    fn spawn_streaming_pipeline(
        &mut self,
        command_display: &str,
        mut spawn: shell::pipeline::StreamingSpawn,
    ) {
        // Write here-string/here-doc data to stdin if present
        if let Some(data) = spawn.stdin_data.take()
            && let Some(mut stdin) = spawn.last_child.stdin.take()
        {
            use std::io::Write;
            let _ = stdin.write_all(&data);
            // Drop stdin to signal EOF
        }

        let (tx, rx) = mpsc::channel::<OutputChunk>();

        if let Some(stdout) = spawn.last_child.stdout.take() {
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

        if let Some(stderr) = spawn.last_child.stderr.take() {
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

        self.running_pipeline = Some(RunningPipeline {
            command: command_display.to_string(),
            buffer: LiveOutputBuffer::new(),
            rx,
            pipeline_children: spawn.children,
            last_child: spawn.last_child,
            start: Instant::now(),
        });
    }

    fn run_interactive(&mut self, path: &std::path::Path, args: &[String]) -> ExecResult {
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
            },
            Err(e) => ExecResult {
                output: vec![format!("error: {e}")],
                exit_code: -1,
            },
        }
    }

    fn drain_running_output(&mut self) {
        let running = match &mut self.running_pipeline {
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
                    self.finalize_running_pipeline();
                    return;
                }
            }
        }
    }

    fn finalize_running_pipeline(&mut self) {
        if let Some(mut running) = self.running_pipeline.take() {
            let exit_code = match running.last_child.wait() {
                Ok(status) => status.code().unwrap_or(-1),
                Err(_) => -1,
            };
            // Wait for all pipeline children too
            for mut c in running.pipeline_children {
                let _ = c.wait();
            }
            let duration = running.start.elapsed();
            let output = running.buffer.into_lines();

            self.last_exit_code = exit_code;
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
        if let Some(mut running) = self.running_pipeline.take() {
            for mut c in running.pipeline_children {
                let _ = c.kill();
                let _ = c.wait();
            }
            let _ = running.last_child.kill();
            let _ = running.last_child.wait();
            let duration = running.start.elapsed();
            let mut output = running.buffer.into_lines();
            output.push("^C".to_string());

            self.last_exit_code = 130;
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
        // Trigger async PATH rescan when user starts typing a new command
        if self.input.buffer.len() == 1 {
            self.spawn_path_scan();
        }

        self.validate_input();

        let input = self.input.buffer.clone();

        // --- Pipe-aware autocomplete ---
        if let Some(pipe_pos) = find_last_pipe_pos(&input) {
            let preceding = input[..pipe_pos].trim().to_string();
            let after_pipe = &input[pipe_pos + 1..];
            let after_trimmed = after_pipe.trim_start();

            if after_trimmed.is_empty() {
                // Nothing after pipe yet — show command name autocomplete
                self.pipe_output_cache = None;
                self.autocomplete.update("");
                self.autocomplete.visible = false;
                return;
            }

            let after_tokens = shell::tokenize(after_trimmed);
            let after_ends_with_space = after_pipe.ends_with(' ');

            if after_tokens.len() <= 1 && !after_ends_with_space {
                // User is still typing the command name after the pipe (e.g., "ps | gr")
                self.pipe_output_cache = None;
                self.autocomplete.update(after_trimmed);
                return;
            }

            // User has a command + is typing arguments — time for pipe output autocomplete
            let filter_partial = if after_ends_with_space {
                String::new()
            } else if after_tokens.len() > 1 {
                after_tokens.last().cloned().unwrap_or_default()
            } else {
                String::new()
            };

            // Build full_prefix: everything up to (but not including) the filter partial
            let full_prefix = if filter_partial.is_empty() {
                input.trim_end().to_string()
            } else {
                input[..input.len() - filter_partial.len()].trim_end().to_string()
            };

            // Check cache or execute the preceding pipeline
            let output = match &self.pipe_output_cache {
                Some((key, cached)) if *key == preceding => cached.clone(),
                _ => {
                    if let Some(lines) = execute_pipeline_for_autocomplete(&preceding) {
                        self.pipe_output_cache = Some((preceding, lines.clone()));
                        lines
                    } else {
                        self.pipe_output_cache = None;
                        self.autocomplete.close();
                        return;
                    }
                }
            };

            self.autocomplete
                .update_with_pipe_output(&filter_partial, &output, &full_prefix);
            return;
        }

        // No pipe — clear pipe cache and proceed with normal autocomplete
        self.pipe_output_cache = None;

        let has_space = input.contains(' ');

        if has_space {
            let tokens = shell::tokenize(&input);
            let ends_with_space = input.ends_with(' ');

            let (prefix_tokens, partial) = if ends_with_space {
                (tokens.as_slice(), String::new())
            } else if tokens.len() > 1 {
                (&tokens[..tokens.len() - 1], tokens[tokens.len() - 1].clone())
            } else {
                self.autocomplete.update(&input);
                return;
            };

            let prefix = prefix_tokens.join(" ");

            if !prefix.is_empty() {
                // Check if the partial token looks like a path
                if autocomplete::is_path_like(&partial) {
                    self.autocomplete.update_with_paths(&partial, &prefix);
                    return;
                }

                let should_spawn = match &self.last_help_prefix {
                    Some(last) => *last != prefix,
                    None => true,
                };

                if should_spawn {
                    self.last_help_prefix = Some(prefix.clone());
                    self.spawn_help_lookup(prefix.clone());
                }

                self.autocomplete
                    .update_with_help(&partial, self.help_cache.get(&prefix), &prefix);
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

        let parts = shell::tokenize(&command_prefix);
        if parts.is_empty() {
            return;
        }

        let base_cmd = &parts[0];

        // Determine how to get help for this command
        enum HelpTarget {
            External {
                path: std::path::PathBuf,
                sub_args: Vec<String>,
            },
            Script {
                bun_path: std::path::PathBuf,
                entry_point: std::path::PathBuf,
                script_dir: std::path::PathBuf,
                sub_args: Vec<String>,
            },
        }

        let target = if base_cmd == "scripts" {
            // Inject subcommands directly for the `scripts` builtin
            self.help_cache.insert(
                command_prefix,
                vec![
                    help_parser::CommandOption {
                        name: "new".into(),
                        description: Some("Create a new script from template".into()),
                        kind: help_parser::OptionKind::Subcommand,
                        args: None,
                        default_value: None,
                        possible_values: None,
                    },
                    help_parser::CommandOption {
                        name: "reload".into(),
                        description: Some("Reload all scripts".into()),
                        kind: help_parser::OptionKind::Subcommand,
                        args: None,
                        default_value: None,
                        possible_values: None,
                    },
                ],
            );
            self.refresh_autocomplete();
            return;
        } else if shell::builtins::lookup(base_cmd).is_some() {
            return;
        } else if let Some(entry) = shell::script_registry::find_script(base_cmd) {
            let bun_path = match shell::path_resolver::find_in_path("bun") {
                Some(p) => p,
                None => return,
            };
            HelpTarget::Script {
                bun_path,
                entry_point: entry.entry_point,
                script_dir: entry.script_dir,
                sub_args: parts[1..].to_vec(),
            }
        } else if let Some(path) = shell::path_resolver::find_in_path(base_cmd) {
            HelpTarget::External {
                path,
                sub_args: parts[1..].to_vec(),
            }
        } else {
            return;
        };

        // Custom help invocations for CLIs that don't use standard --help
        const HELP_OVERRIDES: &[(&str, &[&str])] = &[
            ("ffmpeg", &["-h", "full"]),
            ("ffprobe", &["-h", "full"]),
            ("ffplay", &["-h", "full"]),
        ];

        let base_cmd_lower = parts[0].to_lowercase();
        let override_flags: Option<Vec<String>> = HELP_OVERRIDES
            .iter()
            .find(|(cmd, _)| *cmd == base_cmd_lower)
            .map(|(_, flags)| flags.iter().map(|s| s.to_string()).collect());

        let prefix_clone = command_prefix.clone();
        let (tx, rx) = mpsc::channel::<HelpResult>();

        std::thread::spawn(move || {
            let wait_for_child = |child: &mut Child| -> bool {
                let start = Instant::now();
                loop {
                    match child.try_wait() {
                        Ok(Some(_)) => return true,
                        Ok(None) => {
                            if start.elapsed() > Duration::from_secs(3) {
                                let _ = child.kill();
                                let _ = child.wait();
                                return false;
                            }
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(_) => return false,
                    }
                }
            };

            let read_child_output = |child: &mut Child| -> Option<String> {
                let mut stdout_bytes = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_end(&mut stdout_bytes);
                }
                let mut stderr_bytes = Vec::new();
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_end(&mut stderr_bytes);
                }

                let bytes = if stdout_bytes.len() >= stderr_bytes.len() {
                    stdout_bytes
                } else {
                    stderr_bytes
                };

                let result = decode_output(bytes);
                if result.len() > 20 { Some(result) } else { None }
            };

            let try_help = |flags: &[&str]| -> Option<String> {
                let mut child = match &target {
                    HelpTarget::External { path, sub_args } => {
                        std::process::Command::new(path)
                            .args(sub_args)
                            .args(flags)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()
                            .ok()?
                    }
                    HelpTarget::Script {
                        bun_path,
                        entry_point,
                        script_dir,
                        sub_args,
                    } => std::process::Command::new(bun_path)
                        .arg("run")
                        .arg(entry_point)
                        .args(sub_args)
                        .args(flags)
                        .current_dir(script_dir)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .ok()?,
                };

                if !wait_for_child(&mut child) {
                    return None;
                }
                read_child_output(&mut child)
            };

            let raw = if let Some(ref flags) = override_flags {
                let flag_refs: Vec<&str> = flags.iter().map(|s| s.as_str()).collect();
                try_help(&flag_refs)
                    .or_else(|| try_help(&["--help"]))
                    .or_else(|| try_help(&["-h"]))
            } else {
                try_help(&["--help"])
                    .or_else(|| try_help(&["-h"]))
                    .or_else(|| try_help(&["-?"]))
                    .or_else(|| try_help(&["?"]))
                    .or_else(|| try_help(&["/?"]))
            };

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

    fn drain_background_jobs(&mut self) {
        let mut i = 0;
        while i < self.background_jobs.len() {
            match self.background_jobs[i].last_child.try_wait() {
                Ok(Some(status)) => {
                    let mut job = self.background_jobs.remove(i);
                    for mut c in job.children.drain(..) {
                        let _ = c.wait();
                    }
                    let exit_code = status.code().unwrap_or(-1);
                    let duration = job.start.elapsed();
                    let _ = self.db.insert(
                        &job.command,
                        exit_code,
                        duration.as_millis() as i64,
                        Some(&self.input.cwd),
                    );
                    let status_text = if exit_code == 0 { "Done" } else { "Exit" };
                    self.history.add_entry(CommandEntry {
                        command: format!("[{}] {} {}", job.job_id, status_text, job.command),
                        output: if exit_code != 0 {
                            vec![format!("exit code: {exit_code}")]
                        } else {
                            Vec::new()
                        },
                        duration,
                        exit_code,
                    });
                    self.history.scroll_to_bottom();
                    // Don't increment i since we removed the element
                }
                Ok(None) => {
                    i += 1; // Still running
                }
                Err(_) => {
                    self.background_jobs.remove(i);
                }
            }
        }
    }

    fn drain_config_watcher(&mut self) {
        let mut changed = false;
        while self.config_watcher_rx.try_recv().is_ok() {
            changed = true;
        }
        if changed {
            match Config::reload() {
                Ok(()) => self.input.notify("Config reloaded".to_string()),
                Err(e) => self.input.notify(format!("Config reload failed: {e}")),
            }
        }
    }

    fn spawn_path_scan(&mut self) {
        if self.pending_path_scan.is_some() {
            return;
        }
        if let Some(last) = self.last_path_scan
            && last.elapsed() < Duration::from_secs(5)
        {
            return;
        }

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let executables = shell::path_resolver::scan_path_executables();
            let _ = tx.send(executables);
        });
        self.pending_path_scan = Some(rx);
    }

    fn drain_path_scan(&mut self) {
        let rx = match &self.pending_path_scan {
            Some(rx) => rx,
            None => return,
        };

        match rx.try_recv() {
            Ok(executables) => {
                shell::path_resolver::replace_executables(executables);
                shell::path_resolver::invalidate_cache();
                self.last_path_scan = Some(Instant::now());
                self.pending_path_scan = None;
                self.validate_input();
            }
            Err(TryRecvError::Disconnected) => {
                self.pending_path_scan = None;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    fn refresh_autocomplete(&mut self) {
        let input = &self.input.buffer;
        if !input.contains(' ') {
            return;
        }

        let tokens = shell::tokenize(input);
        let ends_with_space = input.ends_with(' ');

        let (prefix_tokens, partial) = if ends_with_space {
            (tokens.as_slice(), String::new())
        } else if tokens.len() > 1 {
            (&tokens[..tokens.len() - 1], tokens[tokens.len() - 1].clone())
        } else {
            return;
        };

        let prefix = prefix_tokens.join(" ");
        if !prefix.is_empty() {
            if autocomplete::is_path_like(&partial) {
                self.autocomplete.update_with_paths(&partial, &prefix);
            } else {
                self.autocomplete
                    .update_with_help(&partial, self.help_cache.get(&prefix), &prefix);
            }
        }
    }

    fn validate_input(&mut self) {
        self.input.valid_command = shell::is_valid_command(&self.input.buffer);
    }

    fn history_nav_up(&mut self) {
        if let Some(cmd) = self.history_nav.navigate_up(self.db, &self.input.buffer) {
            self.input.buffer = cmd;
            self.input.cursor = self.input.buffer.len();
            self.validate_input();
        }
    }

    fn history_nav_down(&mut self) {
        use history_navigator::NavigateResult;
        match self.history_nav.navigate_down() {
            NavigateResult::Entry(cmd) => {
                self.input.buffer = cmd;
                self.input.cursor = self.input.buffer.len();
                self.validate_input();
            }
            NavigateResult::Original(saved) => {
                self.input.buffer = saved;
                self.input.cursor = self.input.buffer.len();
                self.validate_input();
            }
            NavigateResult::AtBottom => {}
        }
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

/// Decodes raw bytes from a child process, handling UTF-16 LE output (common on Windows
/// for tools like ffmpeg) and falling back to UTF-8.
fn decode_output(bytes: Vec<u8>) -> String {
    // Check for UTF-16 LE BOM
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let u16s: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        return String::from_utf16_lossy(&u16s);
    }

    // Heuristic: if a significant portion of odd-indexed bytes are null, likely UTF-16 LE
    if bytes.len() > 8 {
        let null_count = bytes.iter().skip(1).step_by(2).filter(|&&b| b == 0).count();
        let total_pairs = bytes.len() / 2;
        if total_pairs > 0 && null_count > total_pairs * 3 / 4 {
            let u16s: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect();
            return String::from_utf16_lossy(&u16s);
        }
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

/// Finds the byte position of the last unquoted `|` that is not part of `||`.
fn find_last_pipe_pos(input: &str) -> Option<usize> {
    let mut last_pipe = None;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut byte_offset = 0;

    while i < chars.len() {
        let ch = chars[i];
        let ch_len = ch.len_utf8();

        match ch {
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            '|' if !in_single_quote && !in_double_quote => {
                if i + 1 < chars.len() && chars[i + 1] == '|' {
                    // Skip || (logical OR)
                    byte_offset += ch_len + chars[i + 1].len_utf8();
                    i += 2;
                    continue;
                }
                last_pipe = Some(byte_offset);
            }
            '\\' if !in_single_quote => {
                // Skip escaped character
                if i + 1 < chars.len() {
                    byte_offset += ch_len + chars[i + 1].len_utf8();
                    i += 2;
                    continue;
                }
            }
            _ => {}
        }

        byte_offset += ch_len;
        i += 1;
    }

    last_pipe
}

/// Executes a pipeline string synchronously for autocomplete, with a timeout.
fn execute_pipeline_for_autocomplete(preceding: &str) -> Option<Vec<String>> {
    let input = preceding.to_string();
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        if let Ok(cl) = shell::parser::parse(&input)
            && let Some(chain) = cl.chains.first()
        {
            let result = shell::pipeline::execute_chain_sync(chain);
            let lines: Vec<String> = result.output.into_iter().take(10_000).collect();
            let _ = tx.send(lines);
        }
    });

    rx.recv_timeout(std::time::Duration::from_secs(2)).ok()
}

/// Configures a Command to produce colored output even when stdout is piped.
/// Mush captures subprocess output through pipes and renders it via an ANSI parser,
/// so it is always safe to receive ANSI escape codes from child processes.
pub fn force_color_env(cmd: &mut std::process::Command) -> &mut std::process::Command {
    cmd.env("FORCE_COLOR", "1")
        .env("CLICOLOR_FORCE", "1")
        .env("CARGO_TERM_COLOR", "always")
}
