use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Widget};

use crate::db::HistoryDb;

const MAX_VISIBLE: usize = 10;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub exit_code: i32,
}

#[derive(Debug, Default)]
pub struct HistoryPopover {
    all_entries: Vec<HistoryEntry>,
    pub entries: Vec<HistoryEntry>,
    pub selected: usize,
    pub visible: bool,
    pub search_buffer: String,
}

impl HistoryPopover {
    pub fn open(&mut self, db: &HistoryDb) {
        self.search_buffer.clear();
        self.selected = 0;
        self.visible = true;
        self.all_entries = match db.search("", 200) {
            Ok(records) => records
                .into_iter()
                .map(|r| HistoryEntry {
                    command: r.command,
                    exit_code: r.exit_code,
                })
                .collect(),
            Err(_) => Vec::new(),
        };
        self.apply_filter();
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.all_entries.clear();
        self.entries.clear();
        self.search_buffer.clear();
        self.selected = 0;
    }

    pub fn select_up(&mut self) {
        if !self.entries.is_empty() {
            if self.selected == 0 {
                self.selected = self.entries.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn select_down(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1) % self.entries.len();
        }
    }

    pub fn accept(&mut self) -> Option<String> {
        if self.visible && self.selected < self.entries.len() {
            let result = self.entries[self.selected].command.clone();
            self.close();
            Some(result)
        } else {
            None
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.search_buffer.push(c);
        self.selected = 0;
        self.apply_filter();
    }

    pub fn backspace(&mut self) {
        self.search_buffer.pop();
        self.selected = 0;
        self.apply_filter();
    }

    pub fn popup_height(&self) -> u16 {
        if !self.visible {
            return 0;
        }
        (self.entries.len().min(MAX_VISIBLE) as u16) + 2
    }

    fn apply_filter(&mut self) {
        if self.search_buffer.is_empty() {
            self.entries = self.all_entries.clone();
        } else {
            let query = self.search_buffer.to_lowercase();
            self.entries = self
                .all_entries
                .iter()
                .filter(|e| e.command.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
    }
}

impl Widget for &HistoryPopover {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        if self.entries.is_empty() {
            let block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(title_line(&self.search_buffer));
            block.render(area, buf);
            return;
        }

        let visible_count = self.entries.len().min(MAX_VISIBLE);
        let scroll_start = if self.selected >= visible_count {
            self.selected - visible_count + 1
        } else {
            0
        };

        let inner_width = area.width.saturating_sub(4) as usize;

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .enumerate()
            .skip(scroll_start)
            .take(visible_count)
            .map(|(i, entry)| {
                let is_selected = i == self.selected;
                let bg = if is_selected {
                    Color::DarkGray
                } else {
                    Color::Reset
                };

                let indicator_color = if entry.exit_code == 0 {
                    Color::Green
                } else {
                    Color::Red
                };

                let cmd_display = if entry.command.len() > inner_width.saturating_sub(3) {
                    format!(
                        "{}...",
                        &entry.command[..inner_width.saturating_sub(6).max(1)]
                    )
                } else {
                    entry.command.clone()
                };

                ListItem::new(Line::from(vec![
                    Span::styled("● ", Style::default().fg(indicator_color).bg(bg)),
                    Span::styled(cmd_display, Style::default().fg(Color::White).bg(bg)),
                ]))
            })
            .collect();

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(title_line(&self.search_buffer));

        let list = List::new(items).block(block);
        list.render(area, buf);
    }
}

fn title_line(search: &str) -> Line<'static> {
    if search.is_empty() {
        Line::from(vec![Span::styled(
            " History (Ctrl+R) ",
            Style::default().fg(Color::DarkGray),
        )])
    } else {
        Line::from(vec![
            Span::styled(" search: ", Style::default().fg(Color::DarkGray)),
            Span::styled(search.to_string(), Style::default().fg(Color::Yellow)),
            Span::styled(" ", Style::default().fg(Color::DarkGray)),
        ])
    }
}
