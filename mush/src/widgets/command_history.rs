use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
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

        render_output_lines(&self.output, &config, inner, buf);
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

        render_output_lines(&self.lines, &config, inner, buf);
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
                    let vw = visible_width(line).max(1);
                    ((vw as f64 / inner_width as f64).ceil() as u16).max(1)
                }
            })
            .sum()
    } else {
        output.len() as u16
    }
}

/// Counts visible characters in a string, skipping ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            width += 1;
        }
    }
    width
}

/// Truncates a string to `max_visible` visible characters, preserving ANSI escape sequences.
fn ansi_truncate(s: &str, max_visible: usize) -> String {
    let mut result = String::new();
    let mut visible = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            result.push(c);
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
            result.push(c);
        } else if visible < max_visible {
            result.push(c);
            visible += 1;
        } else {
            break;
        }
    }
    result
}

/// Maps ANSI standard color codes (0-7) to ratatui colors.
fn ansi_color(code: u8) -> Color {
    match code {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Gray,
        _ => Color::Reset,
    }
}

/// Maps ANSI bright color codes (0-7) to ratatui colors.
fn ansi_bright_color(code: u8) -> Color {
    match code {
        0 => Color::DarkGray,
        1 => Color::LightRed,
        2 => Color::LightGreen,
        3 => Color::LightYellow,
        4 => Color::LightBlue,
        5 => Color::LightMagenta,
        6 => Color::LightCyan,
        7 => Color::White,
        _ => Color::Reset,
    }
}

/// Parses a string containing ANSI escape codes into a styled ratatui `Line`.
fn parse_ansi_line(input: &str) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut current_text = String::new();
    let mut current_style = Style::default();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Check for CSI sequence: ESC [
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['

                // Flush accumulated text
                if !current_text.is_empty() {
                    spans.push(Span::styled(
                        std::mem::take(&mut current_text),
                        current_style,
                    ));
                }

                // Collect parameter string until terminator
                let mut param_str = String::new();
                let mut valid = false;
                for ch in chars.by_ref() {
                    if ch == 'm' {
                        valid = true;
                        break;
                    }
                    // Non-SGR CSI sequence (e.g., cursor movement) — collect until
                    // we hit any letter terminator and discard
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                    param_str.push(ch);
                }

                if !valid {
                    continue; // discard non-SGR sequences
                }

                // Parse SGR parameters
                let params: Vec<u16> = if param_str.is_empty() {
                    vec![0] // bare ESC[m is equivalent to ESC[0m (reset)
                } else {
                    param_str
                        .split(';')
                        .filter_map(|p| p.parse::<u16>().ok())
                        .collect()
                };

                let mut i = 0;
                while i < params.len() {
                    match params[i] {
                        0 => current_style = Style::default(),
                        1 => current_style = current_style.add_modifier(Modifier::BOLD),
                        2 => current_style = current_style.add_modifier(Modifier::DIM),
                        3 => current_style = current_style.add_modifier(Modifier::ITALIC),
                        4 => current_style = current_style.add_modifier(Modifier::UNDERLINED),
                        7 => current_style = current_style.add_modifier(Modifier::REVERSED),
                        8 => current_style = current_style.add_modifier(Modifier::HIDDEN),
                        9 => current_style = current_style.add_modifier(Modifier::CROSSED_OUT),
                        22 => {
                            current_style = current_style
                                .remove_modifier(Modifier::BOLD | Modifier::DIM)
                        }
                        23 => {
                            current_style = current_style.remove_modifier(Modifier::ITALIC)
                        }
                        24 => {
                            current_style = current_style.remove_modifier(Modifier::UNDERLINED)
                        }
                        27 => {
                            current_style = current_style.remove_modifier(Modifier::REVERSED)
                        }
                        28 => {
                            current_style = current_style.remove_modifier(Modifier::HIDDEN)
                        }
                        29 => {
                            current_style =
                                current_style.remove_modifier(Modifier::CROSSED_OUT)
                        }
                        // Standard foreground colors
                        c @ 30..=37 => current_style = current_style.fg(ansi_color((c - 30) as u8)),
                        // Extended foreground: 38;5;N or 38;2;R;G;B
                        38 => {
                            if i + 2 < params.len() && params[i + 1] == 5 {
                                current_style =
                                    current_style.fg(Color::Indexed(params[i + 2] as u8));
                                i += 2;
                            } else if i + 4 < params.len()
                                && params[i + 1] == 2
                            {
                                current_style = current_style.fg(Color::Rgb(
                                    params[i + 2] as u8,
                                    params[i + 3] as u8,
                                    params[i + 4] as u8,
                                ));
                                i += 4;
                            }
                        }
                        39 => current_style = current_style.fg(Color::Reset),
                        // Standard background colors
                        c @ 40..=47 => current_style = current_style.bg(ansi_color((c - 40) as u8)),
                        // Extended background: 48;5;N or 48;2;R;G;B
                        48 => {
                            if i + 2 < params.len() && params[i + 1] == 5 {
                                current_style =
                                    current_style.bg(Color::Indexed(params[i + 2] as u8));
                                i += 2;
                            } else if i + 4 < params.len()
                                && params[i + 1] == 2
                            {
                                current_style = current_style.bg(Color::Rgb(
                                    params[i + 2] as u8,
                                    params[i + 3] as u8,
                                    params[i + 4] as u8,
                                ));
                                i += 4;
                            }
                        }
                        49 => current_style = current_style.bg(Color::Reset),
                        // Bright foreground colors
                        c @ 90..=97 => {
                            current_style = current_style.fg(ansi_bright_color((c - 90) as u8))
                        }
                        // Bright background colors
                        c @ 100..=107 => {
                            current_style = current_style.bg(ansi_bright_color((c - 100) as u8))
                        }
                        _ => {} // ignore unknown codes
                    }
                    i += 1;
                }
            } else {
                // Bare ESC not followed by '[' — include as literal
                current_text.push(c);
            }
        } else {
            current_text.push(c);
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        spans.push(Span::styled(current_text, current_style));
    }

    if spans.is_empty() {
        Line::raw("")
    } else {
        Line::from(spans)
    }
}

fn render_output_lines(output: &[String], config: &Config, inner: Rect, buf: &mut Buffer) {
    let truncate_width = config.layout.truncate_command_width as usize;
    let lines: Vec<Line> = output
        .iter()
        .map(|line| {
            let display = if !config.layout.line_wrap && visible_width(line) > truncate_width {
                format!(
                    "{}…",
                    ansi_truncate(line, truncate_width.saturating_sub(1))
                )
            } else {
                line.clone()
            };
            parse_ansi_line(&display)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_plain() {
        assert_eq!(visible_width("hello"), 5);
        assert_eq!(visible_width(""), 0);
    }

    #[test]
    fn visible_width_ansi() {
        assert_eq!(visible_width("\x1b[31mhello\x1b[0m"), 5);
        assert_eq!(visible_width("\x1b[1;34mtext\x1b[0m more"), 9);
    }

    #[test]
    fn truncate_plain() {
        assert_eq!(ansi_truncate("hello world", 5), "hello");
    }

    #[test]
    fn truncate_preserves_escapes() {
        let s = "\x1b[31mhello\x1b[0m world";
        let t = ansi_truncate(s, 5);
        assert_eq!(t, "\x1b[31mhello\x1b[0m");
    }

    #[test]
    fn parse_plain_text() {
        let line = parse_ansi_line("hello world");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "hello world");
        assert_eq!(line.spans[0].style, Style::default());
    }

    #[test]
    fn parse_red_text() {
        let line = parse_ansi_line("\x1b[31mred\x1b[0m normal");
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[0].content, "red");
        assert_eq!(line.spans[0].style, Style::default().fg(Color::Red));
        assert_eq!(line.spans[1].content, " normal");
        assert_eq!(line.spans[1].style, Style::default());
    }

    #[test]
    fn parse_bold_blue() {
        let line = parse_ansi_line("\x1b[1;34mbold blue\x1b[0m");
        assert_eq!(line.spans[0].content, "bold blue");
        assert_eq!(
            line.spans[0].style,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        );
    }

    #[test]
    fn parse_256_color() {
        let line = parse_ansi_line("\x1b[38;5;196mcolor\x1b[0m");
        assert_eq!(line.spans[0].content, "color");
        assert_eq!(
            line.spans[0].style,
            Style::default().fg(Color::Indexed(196))
        );
    }

    #[test]
    fn parse_rgb_color() {
        let line = parse_ansi_line("\x1b[38;2;255;128;0mrgb\x1b[0m");
        assert_eq!(line.spans[0].content, "rgb");
        assert_eq!(
            line.spans[0].style,
            Style::default().fg(Color::Rgb(255, 128, 0))
        );
    }

    #[test]
    fn parse_bright_colors() {
        let line = parse_ansi_line("\x1b[91mbright red\x1b[0m");
        assert_eq!(line.spans[0].content, "bright red");
        assert_eq!(
            line.spans[0].style,
            Style::default().fg(Color::LightRed)
        );
    }

    #[test]
    fn parse_background() {
        let line = parse_ansi_line("\x1b[41mred bg\x1b[0m");
        assert_eq!(line.spans[0].content, "red bg");
        assert_eq!(line.spans[0].style, Style::default().bg(Color::Red));
    }

    #[test]
    fn parse_bare_reset() {
        // ESC[m with no params is equivalent to ESC[0m
        let line = parse_ansi_line("\x1b[31mred\x1b[m normal");
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[1].style, Style::default());
    }

    #[test]
    fn parse_empty() {
        let line = parse_ansi_line("");
        // Empty input produces Line::raw("") which has 0 spans
        assert_eq!(line.spans.len(), 0);
    }
}
