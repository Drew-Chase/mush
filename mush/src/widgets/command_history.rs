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
    /// Returns the rendered height for this entry's block (no gap).
    /// 4 = 2 borders + 2 padding + content lines
    pub fn rendered_height(&self, width: u16) -> u16 {
        let config = Config::get();
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
        4 + content_lines
    }

    fn render_to_buffer(&self, area: Rect, buf: &mut Buffer) {
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
    /// How many lines scrolled UP from the bottom. 0 = pinned to bottom.
    pub scroll_offset: u16,
}

impl CommandHistory {
    /// Total content height across all entries for a given width,
    /// including 1-line gaps between entries.
    pub fn total_content_height(&self, width: u16) -> u16 {
        if self.entries.is_empty() {
            return 0;
        }
        let entry_heights: u16 = self.entries.iter().map(|e| e.rendered_height(width)).sum();
        let gaps = (self.entries.len() - 1) as u16;
        entry_heights + gaps
    }

    /// Maximum scroll offset (0 when content fits in viewport).
    fn max_scroll(&self, viewport_height: u16, viewport_width: u16) -> u16 {
        self.total_content_height(viewport_width)
            .saturating_sub(viewport_height)
    }

    pub fn scroll_up(&mut self, amount: u16, viewport_height: u16, viewport_width: u16) {
        let max = self.max_scroll(viewport_height, viewport_width);
        self.scroll_offset = (self.scroll_offset + amount).min(max);
    }

    pub fn scroll_down(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    /// Reset scroll to bottom (newest content visible).
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
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

        let total = self.total_content_height(area.width);

        // Clamp scroll offset
        let max_scroll = total.saturating_sub(area.height);
        if self.scroll_offset > max_scroll {
            self.scroll_offset = max_scroll;
        }

        // Build the full virtual canvas in a temp buffer, then copy the
        // visible viewport portion to the real buffer.
        // The virtual canvas has height = total content height.
        if total == 0 {
            return;
        }

        let canvas_area = Rect {
            x: 0,
            y: 0,
            width: area.width,
            height: total,
        };
        let mut canvas = Buffer::empty(canvas_area);

        // Render all entries into the canvas
        let mut y: u16 = 0;
        for (i, entry) in self.entries.iter().enumerate() {
            let h = entry.rendered_height(area.width);
            let entry_area = Rect {
                x: 0,
                y,
                width: area.width,
                height: h,
            };
            entry.render_to_buffer(entry_area, &mut canvas);
            y += h;
            if i < self.entries.len() - 1 {
                y += 1; // gap
            }
        }

        // Determine which portion of the canvas is visible.
        // scroll_offset=0 means bottom is visible.
        // The visible window starts at (total - viewport_height - scroll_offset)
        // from the top of the canvas.
        let viewport_height = area.height.min(total);
        let canvas_start = if total <= area.height {
            0u16
        } else {
            total - area.height - self.scroll_offset
        };

        // When content is shorter than viewport, pin to the bottom of the area
        let dest_y_start = if total < area.height {
            area.y + area.height - total
        } else {
            area.y
        };

        // Copy visible rows from canvas to real buffer
        for row in 0..viewport_height {
            let src_row = canvas_start + row;
            if src_row >= total {
                break;
            }
            for col in 0..area.width {
                if let Some(src) = canvas.cell((col, src_row))
                    && let Some(dst) = buf.cell_mut((area.x + col, dest_y_start + row))
                {
                    *dst = src.clone();
                }
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
