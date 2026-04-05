use ratatui::{DefaultTerminal, Frame};
use std::io;
use command_input::CommandInput;

mod command_input;

#[derive(Debug, Default)]
pub struct App {
	cwd: String,
	exit: bool,
}

impl App {
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.draw(frame))?;
			self.exit = true;
		}

		Ok(())
	}

	fn draw(&self, frame: &mut Frame) {
		frame.render_widget(CommandInput::default(), frame.area());
	}
	fn exit(&mut self) {
		self.exit = true;
	}
}