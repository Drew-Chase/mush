use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};
use std::time::Instant;

pub mod autocomplete;
pub mod command_history;
pub mod command_input;

use autocomplete::Autocomplete;
use command_history::{CommandEntry, CommandHistory};
use command_input::CommandInput;

use crate::shell;

#[derive(Debug)]
pub struct App {
    pub history: CommandHistory,
    pub input: CommandInput,
    pub autocomplete: Autocomplete,
    exit: bool,
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
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        // Initial scroll to bottom
        let size = terminal.size()?;
        let input_height = CommandInput::required_height();
        let history_height = size.height.saturating_sub(input_height);
        self.history
            .scroll_to_bottom(history_height, size.width);

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
            // Only handle key press events (not release/repeat)
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

                // Up/Down navigate autocomplete when visible
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
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                    self.input.insert_char(c);
                    self.on_input_changed();
                }

                // Scroll history
                (_, KeyCode::PageUp) => self.input_scroll_up(),
                (_, KeyCode::PageDown) => self.input_scroll_down(),
                (KeyModifiers::SHIFT, KeyCode::Up) => self.input_scroll_up(),
                (KeyModifiers::SHIFT, KeyCode::Down) => self.input_scroll_down(),

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
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let args = &parts[1..];

        let start = Instant::now();

        match shell::resolve_command(trimmed) {
            shell::CommandKind::Builtin(cmd) => {
                let result = shell::builtins::execute(cmd, args);
                let duration = start.elapsed();

                if !result.output.is_empty() {
                    self.history.add_entry(CommandEntry {
                        command: command_display,
                        output: result.output,
                        duration,
                        exit_code: 0,
                    });
                }

                if result.change_dir.is_some() {
                    self.input.update_cwd();
                }

                if result.exit_app {
                    self.exit = true;
                }
            }
            shell::CommandKind::External(path) => {
                let output = std::process::Command::new(&path)
                    .args(args)
                    .output();
                let duration = start.elapsed();

                match output {
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

                        self.history.add_entry(CommandEntry {
                            command: command_display,
                            output: lines,
                            duration,
                            exit_code: out.status.code().unwrap_or(-1),
                        });
                    }
                    Err(e) => {
                        self.history.add_entry(CommandEntry {
                            command: command_display,
                            output: vec![format!("error: {e}")],
                            duration,
                            exit_code: -1,
                        });
                    }
                }
            }
            shell::CommandKind::NotFound => {
                self.history.add_entry(CommandEntry {
                    command: command_display.clone(),
                    output: vec![format!("command not found: {command_display}")],
                    duration: start.elapsed(),
                    exit_code: 127,
                });
            }
        }

        self.input.valid_command = true;
        self.input.update_cwd();
    }

    fn on_input_changed(&mut self) {
        self.validate_input();
        self.autocomplete.update(&self.input.buffer);
    }

    fn validate_input(&mut self) {
        self.input.valid_command = shell::is_valid_command(&self.input.buffer);
    }

    fn input_scroll_up(&mut self) {
        self.history.scroll_up(3);
    }

    fn input_scroll_down(&mut self) {
        // We need to estimate viewport size; use a reasonable default
        // The actual size will be corrected on next draw
        self.history.scroll_down(3, 50, 120);
    }
}
