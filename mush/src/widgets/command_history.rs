use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap};
use std::time::Duration;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct CommandEntry {
    pub command: String,
    pub output: Vec<String>,
    pub duration: Duration,
    pub exit_code: i32,
}

impl CommandEntry {
    /// Returns the number of rendered lines for just this entry's block (no gap).
    /// Includes: 2 for top/bottom border + 2 for top/bottom padding + output lines
    fn rendered_height(&self, width: u16) -> u16 {
        let config = Config::get();
        // 2 for left/right border + 2 for left/right padding
        let inner_width = width.saturating_sub(4) as usize;
        let content_lines: u16 = if config.layout.line_wrap && inner_width > 0 {
            self.output
                .iter()
                .map(|line| {
                    if line.is_empty() {
                        1u16
                    } else {
                        ((line.len() as f64 / inner_width as f64).ceil() as u16).max(1)
                    }
                })
                .sum()
        } else {
            self.output.len() as u16
        };
        // 2 for top/bottom border + 2 for top/bottom padding + content lines
        4 + content_lines
    }

    fn render_entry(&self, area: Rect, buf: &mut Buffer) {
        let config = Config::get();

        let duration_str = format_duration(self.duration);
        let title_bottom = Line::from(vec![
            Span::styled(" took ", Style::default().fg(Color::DarkGray)),
            Span::styled(duration_str, Style::default().fg(Color::Yellow)),
            Span::raw(" "),
        ]);

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::new(1, 1, 1, 1))
            .title(format!(" {} ", self.command))
            .title_bottom(title_bottom);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        // Render output lines
        let truncate_width = config.layout.truncate_command_width as usize;
        let lines: Vec<Line> = self
            .output
            .iter()
            .map(|line| {
                let display = if !config.layout.line_wrap && line.len() > truncate_width {
                    format!("{}…", &line[..truncate_width.saturating_sub(1)])
                } else {
                    line.clone()
                };
                Line::raw(display)
            })
            .collect();

        let mut paragraph = Paragraph::new(lines);
        if config.layout.line_wrap {
            paragraph = paragraph.wrap(Wrap { trim: false });
        }
        paragraph.render(inner, buf);
    }
}

#[derive(Debug, Default)]
pub struct CommandHistory {
    pub entries: Vec<CommandEntry>,
    pub scroll_offset: u16,
}

impl CommandHistory {
    /// Total content height across all entries for a given width,
    /// including 1-line gaps between entries.
    fn total_content_height(&self, width: u16) -> u16 {
        let entry_heights: u16 = self.entries.iter().map(|e| e.rendered_height(width)).sum();
        let gaps = self.entries.len().saturating_sub(1) as u16;
        entry_heights + gaps
    }

    /// Ensure scroll is clamped to valid range and auto-scroll to bottom.
    pub fn scroll_to_bottom(&mut self, viewport_height: u16, viewport_width: u16) {
        let total = self.total_content_height(viewport_width);
        if total > viewport_height {
            self.scroll_offset = total - viewport_height;
        } else {
            self.scroll_offset = 0;
        }
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: u16, viewport_height: u16, viewport_width: u16) {
        let total = self.total_content_height(viewport_width);
        let max_offset = total.saturating_sub(viewport_height);
        self.scroll_offset = (self.scroll_offset + amount).min(max_offset);
    }

    pub fn add_entry(&mut self, entry: CommandEntry) {
        self.entries.push(entry);
    }
}

impl Widget for &mut CommandHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 || self.entries.is_empty() {
            return;
        }

        let total_content = self.total_content_height(area.width);
        let area_bottom = area.y + area.height;

        // Pin content to the bottom: if content is shorter than the area,
        // start rendering from (area_bottom - total_content) so entries
        // are flush with the bottom edge.
        let content_start = if total_content < area.height {
            area_bottom - total_content
        } else {
            area.y
        };

        let mut y_cursor: i32 = content_start as i32 - self.scroll_offset as i32;

        for (i, entry) in self.entries.iter().enumerate() {
            let entry_height = entry.rendered_height(area.width);
            let entry_bottom = y_cursor + entry_height as i32;

            // Skip entries entirely above the viewport
            if entry_bottom <= area.y as i32 {
                y_cursor = entry_bottom + 1; // +1 for gap
                continue;
            }

            // Stop if we're past the viewport
            if y_cursor >= area_bottom as i32 {
                break;
            }

            // Render if the entry starts within the viewport
            if y_cursor >= area.y as i32 {
                let available_height = (area_bottom as i32 - y_cursor) as u16;
                let entry_area = Rect {
                    x: area.x,
                    y: y_cursor as u16,
                    width: area.width,
                    height: entry_height.min(available_height),
                };
                entry.render_entry(entry_area, buf);
            }

            y_cursor = entry_bottom;
            // Add 1-line gap between entries (not after the last one)
            if i < self.entries.len() - 1 {
                y_cursor += 1;
            }
        }
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs_f64();
    if secs < 0.001 {
        format!("{:.0}µs", d.as_micros())
    } else if secs < 1.0 {
        format!("{:.0}ms", d.as_millis())
    } else if secs < 60.0 {
        format!("{:.1}s", secs)
    } else {
        let mins = secs as u64 / 60;
        let remaining = secs as u64 % 60;
        format!("{}m{}s", mins, remaining)
    }
}
