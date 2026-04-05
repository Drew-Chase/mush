use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

#[derive(Default, Debug)]
pub struct CommandInput{
	buffer: String,
}

impl CommandInput {

}

impl Widget for CommandInput {
	fn render(self, area: Rect, buf: &mut Buffer)
	          where
		          Self: Sized
	{
		todo!()
	}
}