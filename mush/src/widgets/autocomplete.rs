use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Widget};

use crate::shell;

const MAX_VISIBLE: usize = 10;

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Autocomplete {
    pub suggestions: Vec<Suggestion>,
    pub selected: usize,
    pub visible: bool,
}

impl Autocomplete {
    /// Update suggestions based on the current input buffer.
    /// Only autocompletes the first token (command name).
    pub fn update(&mut self, input: &str) {
        let query = input.split_whitespace().next().unwrap_or("");

        // Don't show autocomplete if input has spaces (already past command name)
        // or if query is empty
        if query.is_empty() || input.contains(' ') {
            self.visible = false;
            self.suggestions.clear();
            self.selected = 0;
            return;
        }

        let query_lower = query.to_lowercase();
        let mut matches: Vec<(Suggestion, i32)> = shell::all_commands()
            .into_iter()
            .filter_map(|info| {
                let score = fuzzy_score(&query_lower, &info.name.to_lowercase());
                if score > 0 {
                    Some((
                        Suggestion {
                            name: info.name,
                            description: info.description,
                        },
                        score,
                    ))
                } else {
                    None
                }
            })
            .collect();

        // Sort by score descending, then alphabetically
        matches.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));

        self.suggestions = matches.into_iter().map(|(s, _)| s).collect();
        self.selected = 0;
        self.visible = !self.suggestions.is_empty();
    }

    pub fn select_up(&mut self) {
        if !self.suggestions.is_empty() {
            if self.selected == 0 {
                self.selected = self.suggestions.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn select_down(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected = (self.selected + 1) % self.suggestions.len();
        }
    }

    pub fn accept(&mut self) -> Option<String> {
        if self.visible && self.selected < self.suggestions.len() {
            let result = self.suggestions[self.selected].name.clone();
            self.close();
            Some(result)
        } else {
            None
        }
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.suggestions.clear();
        self.selected = 0;
    }

    /// Height needed for the popup (capped at MAX_VISIBLE).
    pub fn popup_height(&self) -> u16 {
        if !self.visible {
            return 0;
        }
        // +2 for borders
        (self.suggestions.len().min(MAX_VISIBLE) as u16) + 2
    }
}

impl Widget for &Autocomplete {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible || self.suggestions.is_empty() {
            return;
        }

        let visible_count = self.suggestions.len().min(MAX_VISIBLE);

        // Determine the scroll window so selected item is always visible
        let scroll_start = if self.selected >= visible_count {
            self.selected - visible_count + 1
        } else {
            0
        };

        // Calculate the inner width for description alignment
        // area.width - 2 for borders - 2 for padding
        let inner_width = area.width.saturating_sub(4) as usize;

        let items: Vec<ListItem> = self.suggestions
            .iter()
            .enumerate()
            .skip(scroll_start)
            .take(visible_count)
            .map(|(i, suggestion)| {
                let is_selected = i == self.selected;
                let bg = if is_selected { Color::DarkGray } else { Color::Reset };

                let line = if let Some(desc) = &suggestion.description {
                    // Truncate description to fit
                    let name_len = suggestion.name.len();
                    let sep = "  ";
                    let available = inner_width.saturating_sub(name_len + sep.len());
                    let truncated = if desc.len() > available && available > 3 {
                        format!("{}...", &desc[..available.saturating_sub(3)])
                    } else if desc.len() > available {
                        desc[..available].to_string()
                    } else {
                        desc.clone()
                    };

                    Line::from(vec![
                        Span::styled(&suggestion.name, Style::default().fg(Color::White).bg(bg)),
                        Span::styled(sep, Style::default().bg(bg)),
                        Span::styled(truncated, Style::default().fg(Color::DarkGray).bg(bg)),
                    ])
                } else {
                    Line::styled(
                        suggestion.name.as_str(),
                        Style::default().fg(Color::White).bg(bg),
                    )
                };

                ListItem::new(line)
            })
            .collect();

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));

        let list = List::new(items).block(block);
        list.render(area, buf);
    }
}

/// Fuzzy scoring: checks if all characters of `query` appear in order in
/// `candidate`. Higher scores for:
/// - Exact match or prefix match (huge bonus)
/// - Consecutive character matches
/// - Shorter candidates (penalize length difference)
fn fuzzy_score(query: &str, candidate: &str) -> i32 {
    if query.is_empty() {
        return 0;
    }

    // Strip extension for matching purposes (e.g. "cargo.exe" -> "cargo")
    let base = candidate.split('.').next().unwrap_or(candidate);

    // Exact match on the base name
    if base == query {
        return 1000;
    }

    // Prefix match on the base name (e.g. "car" matches "cargo")
    let prefix_bonus = if base.starts_with(query) { 500 } else { 0 };

    let query_chars: Vec<char> = query.chars().collect();
    let candidate_chars: Vec<char> = candidate.chars().collect();

    let mut qi = 0;
    let mut score: i32 = 0;
    let mut prev_match_idx: Option<usize> = None;

    for (ci, &cc) in candidate_chars.iter().enumerate() {
        if qi < query_chars.len() && cc == query_chars[qi] {
            // Bonus for matching at the start position
            if ci == qi {
                score += 3;
            } else {
                score += 1;
            }
            // Bonus for consecutive matches
            if let Some(prev) = prev_match_idx
                && ci == prev + 1
            {
                score += 2;
            }
            prev_match_idx = Some(ci);
            qi += 1;
        }
    }

    // All query chars must match
    if qi < query_chars.len() {
        return 0;
    }

    // Penalize longer names — shorter matches are more relevant
    let length_penalty = (candidate.len() as i32 - query.len() as i32).abs();

    score + prefix_bonus - length_penalty
}
