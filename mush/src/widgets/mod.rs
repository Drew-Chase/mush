use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};
use std::time::Instant;

pub mod autocomplete;
pub mod command_history;
pub mod command_input;

use autocomplete::Autocomplete;
use command_history::{CommandEntry, CommandHistory};
use command_input::CommandInput;

use crate::shell;

struct ExecResult {
    output: Vec<String>,
    exit_code: i32,
    exit_app: bool,
}

#[derive(Debug)]
pub struct App {
    pub history: CommandHistory,
    pub input: CommandInput,
    pub autocomplete: Autocomplete,
    exit: bool,
    last_history_area: Rect,
}

impl Default for App {
    fn default() -> Self {
        let mut history = CommandHistory::default();

        // Add some demo entries so the UI isn't empty on first launch
        #[cfg(debug_assertions)]
        {
            use std::time::Duration;
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

        Self {
            history,
            input: CommandInput::default(),
            autocomplete: Autocomplete::default(),
            exit: false,
            last_history_area: Rect::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        self.history.scroll_to_bottom();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let input_height = CommandInput::required_height();
        let popup_height = self.autocomplete.popup_height();
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
        // chunks[2] is the gap
        let input_area = chunks[3];

        self.last_history_area = history_area;

        // Render command history (scrollable, fills top)
        frame.render_widget(&mut self.history, history_area);

        // Render autocomplete popup (if visible)
        if self.autocomplete.visible {
            frame.render_widget(&self.autocomplete, popup_area);
        }

        // Render command input (pinned to bottom)
        frame.render_widget(&self.input, input_area);
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            match (key.modifiers, key.code) {
                // Quit
                (KeyModifiers::CONTROL, KeyCode::Char('c' | 'q')) => {
                    self.exit = true;
                }

                // Escape closes autocomplete
                (_, KeyCode::Esc) => {
                    self.autocomplete.close();
                }

                // Tab accepts autocomplete suggestion
                (_, KeyCode::Tab) => {
                    if let Some(accepted) = self.autocomplete.accept() {
                        self.input.buffer = accepted;
                        self.input.cursor = self.input.buffer.len();
                        self.validate_input();
                    }
                }

                // Up/Down navigate autocomplete when visible, scroll history otherwise
                (KeyModifiers::NONE, KeyCode::Up) if self.autocomplete.visible => {
                    self.autocomplete.select_up();
                }
                (KeyModifiers::NONE, KeyCode::Down) if self.autocomplete.visible => {
                    self.autocomplete.select_down();
                }
                (KeyModifiers::NONE, KeyCode::Up) => self.history_scroll_up(),
                (KeyModifiers::NONE, KeyCode::Down) => self.history_scroll_down(),

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

                // Character input — only printable characters (filter control/escape fragments)
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) if c >= ' ' => {
                    self.input.insert_char(c);
                    self.on_input_changed();
                }

                // Scroll history
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
        let start = Instant::now();

        match shell::resolve_command(trimmed) {
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
                    self.history.add_entry(CommandEntry {
                        command: command_display,
                        output: all_output,
                        duration,
                        exit_code: 0,
                    });
                }
            }
            other => {
                let result = self.dispatch_resolved(other, trimmed);
                let duration = start.elapsed();
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

        self.input.valid_command = true;
        self.input.update_cwd();
        self.history.scroll_to_bottom();
    }

    /// Execute a single command string (used by alias expansion).
    fn execute_single(&mut self, input: &str) -> ExecResult {
        let resolved = shell::resolve_command(input);
        self.dispatch_resolved(resolved, input)
    }

    /// Dispatch an already-resolved command kind.
    fn dispatch_resolved(&mut self, kind: shell::CommandKind, input: &str) -> ExecResult {
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
                // Nested alias — execute each sub-command
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

    fn on_input_changed(&mut self) {
        self.validate_input();
        self.autocomplete.update(&self.input.buffer);
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
