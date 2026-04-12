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
    pub interactive_mode: bool,
    /// Selection anchor byte offset. The selection range is between this and `cursor`.
    pub selection: Option<usize>,
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
            interactive_mode: false,
            selection: None,
        }
    }
}

impl CommandInput {
    /// Returns (start, end) byte offsets of the selection, or None if no selection.
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection.and_then(|anchor| {
            let start = anchor.min(self.cursor);
            let end = anchor.max(self.cursor);
            if start == end {
                None
            } else {
                Some((start, end))
            }
        })
    }

    /// Set the anchor to current cursor position if not already set.
    pub fn start_selection(&mut self) {
        if self.selection.is_none() {
            self.selection = Some(self.cursor);
        }
    }

    /// Clear the selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Delete the selected text and leave cursor at the start of the deleted range.
    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.selection_range() {
            self.buffer.drain(start..end);
            self.cursor = start;
            self.selection = None;
        }
    }

    /// Select all text.
    pub fn select_all(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        self.selection = Some(0);
        self.cursor = self.buffer.len();
    }

    pub fn insert_char(&mut self, c: char) {
        if self.selection_range().is_some() {
            self.delete_selection();
        }
        self.buffer.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn backspace(&mut self) {
        if self.selection_range().is_some() {
            self.delete_selection();
            return;
        }
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
        if self.selection_range().is_some() {
            self.delete_selection();
            return;
        }
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn move_left(&mut self) {
        self.clear_selection();
        self.move_left_inner();
    }

    pub fn move_right(&mut self) {
        self.clear_selection();
        self.move_right_inner();
    }

    pub fn move_left_select(&mut self) {
        self.start_selection();
        self.move_left_inner();
    }

    pub fn move_right_select(&mut self) {
        self.start_selection();
        self.move_right_inner();
    }

    pub fn home_select(&mut self) {
        self.start_selection();
        self.cursor = 0;
    }

    pub fn end_select(&mut self) {
        self.start_selection();
        self.cursor = self.buffer.len();
    }

    fn move_left_inner(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.buffer[..self.cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    fn move_right_inner(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += self.buffer[self.cursor..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
        }
    }

    pub fn home(&mut self) {
        self.clear_selection();
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.clear_selection();
        self.cursor = self.buffer.len();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
        self.selection = None;
    }

    pub fn take_buffer(&mut self) -> String {
        self.cursor = 0;
        self.selection = None;
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

const SELECTION_BG: Color = Color::Indexed(24); // dark blue

impl Widget for &CommandInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 2 || area.width == 0 {
            return;
        }

        // Build CWD title with last segment highlighted
        let cwd_title = cwd_to_title(&self.cwd);

        let mut hint_spans = Vec::new();
        if self.interactive_mode {
            hint_spans.push(Span::styled(" [INTERACTIVE] ", Style::default().fg(Color::Yellow)));
            hint_spans.push(Span::styled("| ", Style::default().fg(Color::DarkGray)));
        }
        hint_spans.extend([
            Span::styled(" Enter ", Style::default().fg(Color::DarkGray)),
            Span::styled("Send", Style::default().fg(Color::DarkGray)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("\u{2191}/\u{2193} ", Style::default().fg(Color::DarkGray)),
            Span::styled("History", Style::default().fg(Color::DarkGray)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+\u{2191}/\u{2193} ", Style::default().fg(Color::DarkGray)),
            Span::styled("Scroll", Style::default().fg(Color::DarkGray)),
            Span::raw(" "),
        ]);
        let hints = Line::from(hint_spans).alignment(ratatui::layout::Alignment::Right);

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
            let line = Line::from(vec![
                Span::styled(" ", Style::default().bg(Color::White).fg(Color::Black)),
                Span::styled("Command...", Style::default().fg(Color::DarkGray)),
            ]);
            Paragraph::new(line).render(inner, buf);
        } else {
            let text_color = if self.valid_command {
                Color::White
            } else {
                Color::Red
            };

            let sel_style = Style::default().bg(SELECTION_BG).fg(text_color);
            let normal_style = Style::default().fg(text_color);
            let cursor_style = Style::default().bg(Color::White).fg(Color::Black);

            let spans = match self.selection_range() {
                Some((sel_start, sel_end)) => {
                    // Build spans accounting for selection and cursor
                    let cursor = self.cursor;
                    let cursor_char = self.buffer[cursor..]
                        .chars()
                        .next()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| " ".to_string());
                    let cursor_end = if cursor < self.buffer.len() {
                        cursor + cursor_char.len()
                    } else {
                        cursor
                    };

                    let mut spans = Vec::new();

                    // Before selection
                    if sel_start > 0 {
                        spans.push(Span::styled(&self.buffer[..sel_start], normal_style));
                    }

                    if cursor >= sel_start && cursor < sel_end {
                        // Cursor is inside the selection
                        if cursor > sel_start {
                            spans.push(Span::styled(&self.buffer[sel_start..cursor], sel_style));
                        }
                        spans.push(Span::styled(cursor_char, cursor_style));
                        if cursor_end < sel_end {
                            spans.push(Span::styled(&self.buffer[cursor_end..sel_end], sel_style));
                        }
                    } else {
                        // Cursor is outside the selection (at sel_end)
                        spans.push(Span::styled(&self.buffer[sel_start..sel_end], sel_style));
                        spans.push(Span::styled(cursor_char, cursor_style));
                    }

                    // After selection and cursor
                    let after_start = sel_end.max(cursor_end);
                    if after_start < self.buffer.len() {
                        spans.push(Span::styled(&self.buffer[after_start..], normal_style));
                    }

                    spans
                }
                None => {
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

                    vec![
                        Span::styled(before_cursor, normal_style),
                        Span::styled(at_cursor, cursor_style),
                        Span::styled(after_cursor, normal_style),
                    ]
                }
            };

            let line = Line::from(spans);
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
