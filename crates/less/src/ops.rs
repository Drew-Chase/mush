use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{
    self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, queue};

use crate::cli::LessConfig;

pub struct Pager {
    lines: Vec<String>,
    offset: usize,
    search: Option<String>,
    config: LessConfig,
}

impl Pager {
    pub fn new(lines: Vec<String>, config: LessConfig) -> Self {
        let offset = config
            .start_line
            .map(|n| n.saturating_sub(1))
            .unwrap_or(0);

        let mut pager = Self {
            lines,
            offset,
            search: None,
            config,
        };

        // Clamp offset
        pager.clamp_offset();

        // Handle start_pattern
        if let Some(ref pattern) = pager.config.start_pattern.clone() {
            pager.search = Some(pattern.clone());
            pager.search_forward(pattern);
        }

        pager
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        if !self.config.no_init {
            execute!(stdout, EnterAlternateScreen)?;
        }
        terminal::enable_raw_mode()?;
        execute!(stdout, Hide)?;

        let result = self.main_loop(&mut stdout);

        execute!(stdout, Show)?;
        terminal::disable_raw_mode()?;
        if !self.config.no_init {
            execute!(stdout, LeaveAlternateScreen)?;
        }

        result
    }

    fn main_loop(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        loop {
            let (width, height) = terminal::size()?;
            self.draw(stdout, width, height)?;

            match event::read()? {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.offset = self.offset.saturating_add(1);
                        self.clamp_offset();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.offset = self.offset.saturating_sub(1);
                    }
                    KeyCode::Char('f') | KeyCode::Char(' ') | KeyCode::PageDown => {
                        let (_, h) = terminal::size()?;
                        let page = h.saturating_sub(1) as usize;
                        self.offset = self.offset.saturating_add(page);
                        self.clamp_offset();
                    }
                    KeyCode::Char('b') | KeyCode::PageUp => {
                        let (_, h) = terminal::size()?;
                        let page = h.saturating_sub(1) as usize;
                        self.offset = self.offset.saturating_sub(page);
                    }
                    KeyCode::Char('g') | KeyCode::Home => {
                        self.offset = 0;
                    }
                    KeyCode::Char('G') | KeyCode::End => {
                        let (_, h) = terminal::size()?;
                        let visible = h.saturating_sub(1) as usize;
                        self.offset = self.lines.len().saturating_sub(visible);
                    }
                    KeyCode::Char('/') => {
                        self.enter_search_mode(stdout)?;
                    }
                    KeyCode::Char('n') => {
                        if let Some(pattern) = self.search.clone() {
                            self.search_forward(&pattern);
                        }
                    }
                    KeyCode::Char('N') => {
                        if let Some(pattern) = self.search.clone() {
                            self.search_backward(&pattern);
                        }
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {
                    // Will redraw on next loop iteration
                }
                _ => {}
            }
        }
    }

    fn draw(&self, stdout: &mut impl Write, width: u16, height: u16) -> io::Result<()> {
        let visible_rows = height.saturating_sub(1) as usize; // reserve 1 for status bar
        let line_num_width = if self.config.line_numbers {
            let max_line = self.lines.len();
            format!("{max_line}").len() + 1 // +1 for the space separator
        } else {
            0
        };

        queue!(stdout, MoveTo(0, 0))?;

        for row in 0..visible_rows {
            let line_idx = self.offset + row;
            queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::CurrentLine))?;

            if line_idx < self.lines.len() {
                let line = &self.lines[line_idx];

                if self.config.line_numbers {
                    let num_str = format!("{:>width$} ", line_idx + 1, width = line_num_width - 1);
                    queue!(stdout, Print(&num_str))?;
                }

                let available_width = (width as usize).saturating_sub(line_num_width);
                if self.config.chop_long_lines && line.len() > available_width {
                    let truncated: String = line.chars().take(available_width).collect();
                    queue!(stdout, Print(truncated))?;
                } else {
                    queue!(stdout, Print(line))?;
                }
            } else {
                queue!(stdout, Print("~"))?;
            }
        }

        // Status bar
        self.draw_status_bar(stdout, width, height)?;

        stdout.flush()
    }

    fn draw_status_bar(&self, stdout: &mut impl Write, width: u16, height: u16) -> io::Result<()> {
        let status_row = height.saturating_sub(1);
        queue!(
            stdout,
            MoveTo(0, status_row),
            Clear(ClearType::CurrentLine),
            SetAttribute(Attribute::Reverse)
        )?;

        let total = self.lines.len();
        let end_line = self.offset + 1;
        let (_, h) = terminal::size()?;
        let visible = h.saturating_sub(1) as usize;
        let last_visible = (self.offset + visible).min(total);

        let mut status = if total == 0 {
            "(empty)".to_string()
        } else {
            format!("lines {end_line}-{last_visible}/{total}")
        };

        if let Some(ref search) = self.search {
            status.push_str(&format!("  /{search}"));
        }

        // Pad or truncate to fill the width
        let status_display: String = if status.len() < width as usize {
            format!("{:<width$}", status, width = width as usize)
        } else {
            status.chars().take(width as usize).collect()
        };

        queue!(
            stdout,
            Print(status_display),
            SetAttribute(Attribute::Reset)
        )?;

        Ok(())
    }

    fn enter_search_mode(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (_, height) = terminal::size()?;
        let status_row = height.saturating_sub(1);

        // Show search prompt
        execute!(
            stdout,
            MoveTo(0, status_row),
            Clear(ClearType::CurrentLine),
            Print("/")
        )?;

        let mut pattern = String::new();

        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => {
                        if !pattern.is_empty() {
                            self.search = Some(pattern.clone());
                            self.search_forward(&pattern);
                        }
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Backspace => {
                        pattern.pop();
                        execute!(
                            stdout,
                            MoveTo(0, status_row),
                            Clear(ClearType::CurrentLine),
                            Print(format!("/{pattern}"))
                        )?;
                    }
                    KeyCode::Char(c) => {
                        pattern.push(c);
                        execute!(
                            stdout,
                            MoveTo(0, status_row),
                            Clear(ClearType::CurrentLine),
                            Print(format!("/{pattern}"))
                        )?;
                    }
                    _ => {}
                }
            }
        }
    }

    fn search_forward(&mut self, pattern: &str) {
        let start = self.offset + 1;
        for i in start..self.lines.len() {
            if self.line_matches(&self.lines[i], pattern) {
                self.offset = i;
                return;
            }
        }
        // Wrap around
        for i in 0..start.min(self.lines.len()) {
            if self.line_matches(&self.lines[i], pattern) {
                self.offset = i;
                return;
            }
        }
    }

    fn search_backward(&mut self, pattern: &str) {
        if self.offset == 0 {
            // Search from end
            for i in (0..self.lines.len()).rev() {
                if self.line_matches(&self.lines[i], pattern) {
                    self.offset = i;
                    return;
                }
            }
            return;
        }
        for i in (0..self.offset).rev() {
            if self.line_matches(&self.lines[i], pattern) {
                self.offset = i;
                return;
            }
        }
        // Wrap around
        for i in (self.offset..self.lines.len()).rev() {
            if self.line_matches(&self.lines[i], pattern) {
                self.offset = i;
                return;
            }
        }
    }

    fn line_matches(&self, line: &str, pattern: &str) -> bool {
        if self.config.ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        }
    }

    fn clamp_offset(&mut self) {
        if self.lines.is_empty() {
            self.offset = 0;
            return;
        }
        let max = self.lines.len().saturating_sub(1);
        if self.offset > max {
            self.offset = max;
        }
    }
}
