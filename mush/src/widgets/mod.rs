use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};

pub mod command_history;
pub mod command_input;

use command_history::{CommandEntry, CommandHistory};
use command_input::CommandInput;

#[derive(Debug)]
pub struct App {
    pub history: CommandHistory,
    pub input: CommandInput,
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
        let chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(2),
            Constraint::Length(input_height),
        ])
        .split(area);

        let history_area = chunks[0];
        // chunks[1] is the 2-line gap
        let input_area = chunks[2];

        // Render command history (scrollable, fills top)
        frame.render_widget(&mut self.history, history_area);

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

                // Submit command
                (_, KeyCode::Enter) => {
                    let _command = self.input.take_buffer();
                    self.input.update_cwd();
                    // TODO: execute the command and add entry to history
                }

                // Text editing
                (_, KeyCode::Backspace) => self.input.backspace(),
                (_, KeyCode::Delete) => self.input.delete(),
                (_, KeyCode::Left) => self.input.move_left(),
                (_, KeyCode::Right) => self.input.move_right(),
                (_, KeyCode::Home) => self.input.home(),
                (_, KeyCode::End) => self.input.end(),

                // Character input
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                    self.input.insert_char(c);
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

    fn input_scroll_up(&mut self) {
        self.history.scroll_up(3);
    }

    fn input_scroll_down(&mut self) {
        // We need to estimate viewport size; use a reasonable default
        // The actual size will be corrected on next draw
        self.history.scroll_down(3, 50, 120);
    }
}
