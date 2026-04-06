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
    pub fn rendered_height(&self, width: u16) -> u16 {
        compute_content_height(&self.output, width) + 4
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

        render_output_lines(&self.output, config, inner, buf);
    }
}

#[derive(Debug)]
pub struct LiveRenderData {
    pub command: String,
    pub lines: Vec<String>,
    pub elapsed: Duration,
}

impl LiveRenderData {
    fn rendered_height(&self, width: u16) -> u16 {
        compute_content_height(&self.lines, width) + 4
    }

    fn render_to_buffer(&self, area: Rect, buf: &mut Buffer) {
        let config = Config::get();

        let elapsed_str = format_duration(self.elapsed);
        let title_bottom = Line::from(vec![
            Span::styled(" running... ", Style::default().fg(Color::Cyan)),
            Span::styled(elapsed_str, Style::default().fg(Color::Cyan)),
            Span::raw(" "),
        ]);

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::new(1, 1, 1, 1))
            .title(format!(" {} ", self.command))
            .title_bottom(title_bottom);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        render_output_lines(&self.lines, config, inner, buf);
    }
}

fn compute_content_height(output: &[String], width: u16) -> u16 {
    let config = Config::get();
    let inner_width = width.saturating_sub(4) as usize;
    if config.layout.line_wrap && inner_width > 0 {
        output
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
        output.len() as u16
    }
}

fn render_output_lines(output: &[String], config: &Config, inner: Rect, buf: &mut Buffer) {
    let truncate_width = config.layout.truncate_command_width as usize;
    let lines: Vec<Line> = output
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

pub struct LiveOutputBuffer {
    lines: Vec<String>,
    partial: String,
}

impl LiveOutputBuffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            partial: String::new(),
        }
    }

    pub fn push(&mut self, text: &str) {
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                        self.lines.push(std::mem::take(&mut self.partial));
                    } else {
                        self.partial.clear();
                    }
                }
                '\n' => {
                    self.lines.push(std::mem::take(&mut self.partial));
                }
                c => {
                    self.partial.push(c);
                }
            }
        }
    }

    pub fn all_lines(&self) -> Vec<&str> {
        let mut result: Vec<&str> = self.lines.iter().map(|s| s.as_str()).collect();
        if !self.partial.is_empty() {
            result.push(&self.partial);
        }
        result
    }

    pub fn into_lines(mut self) -> Vec<String> {
        if !self.partial.is_empty() {
            self.lines.push(self.partial);
        }
        self.lines
    }
}

#[derive(Debug, Default)]
pub struct CommandHistory {
    pub entries: Vec<CommandEntry>,
    pub scroll_offset: u16,
    pub live_entry: Option<LiveRenderData>,
}

impl CommandHistory {
    pub fn total_content_height(&self, width: u16) -> u16 {
        let mut total: u16 = 0;
        let entry_count = self.entries.len();

        if entry_count > 0 {
            let entry_heights: u16 = self.entries.iter().map(|e| e.rendered_height(width)).sum();
            let gaps = (entry_count - 1) as u16;
            total = entry_heights + gaps;
        }

        if let Some(live) = &self.live_entry {
            if total > 0 {
                total += 1; // gap before live entry
            }
            total += live.rendered_height(width);
        }

        total
    }

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

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn add_entry(&mut self, entry: CommandEntry) {
        self.entries.push(entry);
    }
}

impl Widget for &mut CommandHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let has_content = !self.entries.is_empty() || self.live_entry.is_some();
        if area.height == 0 || area.width == 0 || !has_content {
            return;
        }

        let total = self.total_content_height(area.width);

        let max_scroll = total.saturating_sub(area.height);
        if self.scroll_offset > max_scroll {
            self.scroll_offset = max_scroll;
        }

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
                y += 1;
            }
        }

        if let Some(live) = &self.live_entry {
            if !self.entries.is_empty() {
                y += 1; // gap
            }
            let h = live.rendered_height(area.width);
            let live_area = Rect {
                x: 0,
                y,
                width: area.width,
                height: h,
            };
            live.render_to_buffer(live_area, &mut canvas);
        }

        let viewport_height = area.height.min(total);
        let canvas_start = if total <= area.height {
            0u16
        } else {
            total - area.height - self.scroll_offset
        };

        let dest_y_start = if total < area.height {
            area.y + area.height - total
        } else {
            area.y
        };

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
