use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct CommandInput {
    pub buffer: String,
    pub cursor: usize,
    pub cwd: String,
    pub valid_command: bool,
    pub notification: Option<(String, Instant)>,
}

impl Default for CommandInput {
    fn default() -> Self {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        Self {
            buffer: String::new(),
            cursor: 0,
            cwd,
            valid_command: true,
            notification: None,
        }
    }
}

impl CommandInput {
    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            // Find the previous char boundary
            let prev = self.buffer[..self.cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.buffer.remove(prev);
            self.cursor = prev;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.buffer[..self.cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += self.buffer[self.cursor..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
        }
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.buffer.len();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    pub fn take_buffer(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.buffer)
    }

    pub fn update_cwd(&mut self) {
        self.cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
    }

    pub fn notify(&mut self, message: String) {
        self.notification = Some((message, Instant::now()));
    }

    /// Total height this widget needs: 2 border + 1 content
    pub fn required_height() -> u16 {
        3
    }
}

impl Widget for &CommandInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 2 || area.width == 0 {
            return;
        }

        // Build CWD title with last segment highlighted
        let cwd_title = cwd_to_title(&self.cwd);

        let hints = Line::from(vec![
            Span::styled(" Enter ", Style::default().fg(Color::DarkGray)),
            Span::styled("Send", Style::default().fg(Color::DarkGray)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+R ", Style::default().fg(Color::DarkGray)),
            Span::styled("History", Style::default().fg(Color::DarkGray)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("PgUp/PgDn ", Style::default().fg(Color::DarkGray)),
            Span::styled("Scroll", Style::default().fg(Color::DarkGray)),
            Span::raw(" "),
        ]).alignment(ratatui::layout::Alignment::Right);

        // Build notification title (top-right) if active and not expired
        let notification_title = self
            .notification
            .as_ref()
            .filter(|(_, ts)| ts.elapsed().as_secs() < 15)
            .map(|(msg, _)| {
                Line::from(vec![
                    Span::styled(format!(" {msg} "), Style::default().fg(Color::Green)),
                ])
                .alignment(Alignment::Right)
            });

        let mut block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::horizontal(1))
            .title(cwd_title)
            .title_bottom(hints);

        if let Some(notif) = notification_title {
            block = block.title(notif);
        }

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        if self.buffer.is_empty() {
            let placeholder = Paragraph::new("Command...")
                .style(Style::default().fg(Color::DarkGray));
            placeholder.render(inner, buf);
        } else {
            let text_color = if self.valid_command {
                Color::White
            } else {
                Color::Red
            };

            let before_cursor = &self.buffer[..self.cursor];
            let at_cursor = self.buffer[self.cursor..]
                .chars()
                .next()
                .map(|c| c.to_string())
                .unwrap_or_else(|| " ".to_string());
            let after_cursor = if self.cursor < self.buffer.len() {
                &self.buffer[self.cursor + at_cursor.len()..]
            } else {
                ""
            };

            let line = Line::from(vec![
                Span::styled(before_cursor, Style::default().fg(text_color)),
                Span::styled(
                    at_cursor,
                    Style::default().bg(Color::White).fg(Color::Black),
                ),
                Span::styled(after_cursor, Style::default().fg(text_color)),
            ]);
            Paragraph::new(line).render(inner, buf);
        }
    }
}

fn cwd_to_title(cwd: &str) -> Line<'static> {
    let normalized = cwd.replace('/', "\\");
    let (parent, last) = match normalized.rfind('\\') {
        Some(pos) => (
            normalized[..=pos].to_string(),
            normalized[pos + 1..].to_string(),
        ),
        None => (String::new(), normalized),
    };

    Line::from(vec![
        Span::styled(parent, Style::default().fg(Color::White)),
        Span::styled(last, Style::default().fg(Color::Yellow)),
    ])
}
