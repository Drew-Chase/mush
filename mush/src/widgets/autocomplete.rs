use std::path::PathBuf;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Widget};

use crate::shell;
use crate::shell::help_parser::{CommandOption, OptionKind};

const MAX_VISIBLE: usize = 10;

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Autocomplete {
    pub suggestions: Vec<Suggestion>,
    pub selected: usize,
    pub visible: bool,
    current_prefix: Option<String>,
    path_base: Option<String>,
    pipe_prefix: Option<String>,
}

impl Autocomplete {
    pub fn update(&mut self, input: &str) {
        self.current_prefix = None;
        self.path_base = None;

        let query = input.split_whitespace().next().unwrap_or("");

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
                            display_name: None,
                            description: info.description,
                        },
                        score,
                    ))
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));

        self.suggestions = matches.into_iter().map(|(s, _)| s).collect();
        self.selected = 0;
        self.visible = !self.suggestions.is_empty();
    }

    pub fn update_with_help(
        &mut self,
        partial: &str,
        options: Option<&Vec<CommandOption>>,
        prefix: &str,
    ) {
        self.current_prefix = Some(prefix.to_string());
        self.path_base = None;

        let options = match options {
            Some(opts) if !opts.is_empty() => opts,
            _ => {
                self.visible = false;
                self.suggestions.clear();
                self.selected = 0;
                return;
            }
        };

        let query_lower = partial.to_lowercase();

        let mut matches: Vec<(Suggestion, i32)> = options
            .iter()
            .filter_map(|opt| {
                let display_name = opt.args.as_ref().map(|a| format!("{} {a}", opt.name));
                if partial.is_empty() {
                    Some((
                        Suggestion {
                            name: opt.name.clone(),
                            display_name,
                            description: opt.description.clone(),
                        },
                        if opt.kind == OptionKind::Subcommand {
                            100
                        } else {
                            50
                        },
                    ))
                } else {
                    let score = fuzzy_score(&query_lower, &opt.name.to_lowercase());
                    if score > 0 {
                        Some((
                            Suggestion {
                                name: opt.name.clone(),
                                display_name,
                                description: opt.description.clone(),
                            },
                            score,
                        ))
                    } else {
                        None
                    }
                }
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));

        self.suggestions = matches.into_iter().map(|(s, _)| s).collect();
        self.selected = 0;
        self.visible = !self.suggestions.is_empty();
    }

    /// Populates suggestions with filesystem entries matching the given path token.
    pub fn update_with_paths(&mut self, partial_path: &str, command_prefix: &str) {
        self.current_prefix = None;

        let (dir, file_prefix, display_base) = split_path(partial_path);

        // Store base so accept() can reconstruct the full command + path
        self.path_base = Some(format!("{command_prefix} {display_base}"));

        let entries = match std::fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => {
                self.visible = false;
                self.suggestions.clear();
                self.selected = 0;
                return;
            }
        };

        let prefix_lower = file_prefix.to_lowercase();
        let mut dirs: Vec<Suggestion> = Vec::new();
        let mut files: Vec<Suggestion> = Vec::new();

        for entry in entries.flatten().take(500) {
            let name = entry.file_name().to_string_lossy().to_string();
            let name_lower = name.to_lowercase();

            if !prefix_lower.is_empty() && !name_lower.starts_with(&prefix_lower) {
                continue;
            }

            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let display_name = if is_dir {
                format!("{name}/")
            } else {
                name
            };
            let desc = if is_dir {
                Some("<DIR>".to_string())
            } else {
                None
            };

            let suggestion = Suggestion {
                name: display_name,
                display_name: None,
                description: desc,
            };
            if is_dir {
                dirs.push(suggestion);
            } else {
                files.push(suggestion);
            }

            if dirs.len() + files.len() >= 100 {
                break;
            }
        }

        dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        self.suggestions = dirs;
        self.suggestions.extend(files);
        self.selected = 0;
        self.visible = !self.suggestions.is_empty();
    }

    /// Populates suggestions with output lines from a preceding pipeline,
    /// filtered by the partial argument the user is typing.
    pub fn update_with_pipe_output(
        &mut self,
        partial: &str,
        output_lines: &[String],
        full_prefix: &str,
    ) {
        self.current_prefix = None;
        self.path_base = None;
        self.pipe_prefix = Some(full_prefix.to_string());

        let query_lower = partial.to_lowercase();

        let mut matches: Vec<(Suggestion, i32)> = output_lines
            .iter()
            .filter_map(|line| {
                let trimmed = strip_ansi(line.trim());
                if trimmed.is_empty() {
                    return None;
                }
                if partial.is_empty() {
                    Some((
                        Suggestion {
                            name: trimmed,
                            display_name: None,
                            description: None,
                        },
                        0,
                    ))
                } else {
                    let line_lower = trimmed.to_lowercase();
                    if line_lower.contains(&query_lower) {
                        let score = if line_lower.starts_with(&query_lower) {
                            100
                        } else {
                            50
                        };
                        Some((
                            Suggestion {
                                name: trimmed,
                                display_name: None,
                                description: None,
                            },
                            score,
                        ))
                    } else {
                        None
                    }
                }
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));
        matches.dedup_by(|a, b| a.0.name == b.0.name);

        self.suggestions = matches.into_iter().take(100).map(|(s, _)| s).collect();
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
            let name = &self.suggestions[self.selected].name;
            let result = if let Some(base) = &self.pipe_prefix {
                if name.contains(' ') {
                    format!("{base} \"{name}\"")
                } else {
                    format!("{base} {name}")
                }
            } else if let Some(base) = &self.path_base {
                let full = format!("{base}{name}");
                // Find where the command prefix ends (first space) to isolate the path portion
                if let Some(space_idx) = full.find(' ') {
                    let cmd = &full[..space_idx];
                    let path = &full[space_idx + 1..];
                    if path.contains(' ') {
                        format!("{cmd} \"{path}\"")
                    } else {
                        full
                    }
                } else {
                    full
                }
            } else if let Some(prefix) = &self.current_prefix {
                format!("{prefix} {name}")
            } else {
                name.clone()
            };
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
        self.current_prefix = None;
        self.path_base = None;
        self.pipe_prefix = None;
    }

    pub fn popup_height(&self) -> u16 {
        if !self.visible {
            return 0;
        }
        (self.suggestions.len().min(MAX_VISIBLE) as u16) + 2
    }
}

impl Widget for &Autocomplete {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible || self.suggestions.is_empty() {
            return;
        }

        let visible_count = self.suggestions.len().min(MAX_VISIBLE);

        let scroll_start = if self.selected >= visible_count {
            self.selected - visible_count + 1
        } else {
            0
        };

        let inner_width = area.width.saturating_sub(4) as usize;

        let items: Vec<ListItem> = self
            .suggestions
            .iter()
            .enumerate()
            .skip(scroll_start)
            .take(visible_count)
            .map(|(i, suggestion)| {
                let is_selected = i == self.selected;
                let bg = if is_selected {
                    Color::DarkGray
                } else {
                    Color::Reset
                };

                let visible_name = suggestion
                    .display_name
                    .as_deref()
                    .unwrap_or(&suggestion.name);

                let line = if let Some(desc) = &suggestion.description {
                    let name_len = visible_name.len();
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
                        Span::styled(
                            visible_name,
                            Style::default().fg(Color::White).bg(bg),
                        ),
                        Span::styled(sep, Style::default().bg(bg)),
                        Span::styled(truncated, Style::default().fg(if is_selected { Color::Gray } else { Color::DarkGray }).bg(bg)),
                    ])
                } else {
                    Line::styled(
                        visible_name,
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

/// Returns `true` if the token looks like a filesystem path.
pub(crate) fn is_path_like(token: &str) -> bool {
    if token.starts_with("./")
        || token.starts_with(".\\")
        || token == "."
        || token.starts_with("../")
        || token.starts_with("..\\")
        || token == ".."
        || token.starts_with('/')
        || token.starts_with('~')
    {
        return true;
    }
    // Drive letter pattern: e.g. C: C:\ C:/ D:\foo
    let bytes = token.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

/// Splits a path token into (directory to read, filename prefix, display base).
///
/// The display base is the directory portion as typed by the user (preserving `~`, `./`, etc.)
/// and always ends with a separator. The `PathBuf` is the resolved directory for `read_dir`.
fn split_path(token: &str) -> (PathBuf, String, String) {
    // Find the last path separator
    let last_sep = token.rfind(['/', '\\']);

    let (dir_str, file_prefix) = match last_sep {
        Some(idx) => (&token[..=idx], &token[idx + 1..]),
        None => {
            // No separator: e.g. "C:" — treat as "C:\"
            let bytes = token.as_bytes();
            if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
                let drive = &token[..2];
                let prefix = &token[2..];
                let dir_with_sep = format!("{drive}\\");
                let resolved = PathBuf::from(&dir_with_sep);
                return (resolved, prefix.to_string(), dir_with_sep);
            }
            // Bare "." or ".." without separator
            if token == "." || token == ".." {
                let display = format!("{token}/");
                let resolved = PathBuf::from(token);
                return (resolved, String::new(), display);
            }
            // Fallback: treat as relative path in current dir
            return (PathBuf::from("."), token.to_string(), String::new());
        }
    };

    let display_base = dir_str.to_string();

    // Resolve tilde for the actual PathBuf
    let resolved = if dir_str.starts_with('~') {
        match shell::builtins::home_dir() {
            Some(home) => {
                let rest = dir_str
                    .strip_prefix("~/")
                    .or_else(|| dir_str.strip_prefix("~\\"))
                    .unwrap_or("");
                home.join(rest)
            }
            None => PathBuf::from(dir_str),
        }
    } else {
        PathBuf::from(dir_str)
    };

    (resolved, file_prefix.to_string(), display_base)
}

fn fuzzy_score(query: &str, candidate: &str) -> i32 {
    if query.is_empty() {
        return 0;
    }

    let base = candidate.split('.').next().unwrap_or(candidate);

    if base == query {
        return 1000;
    }

    let prefix_bonus = if base.starts_with(query) { 500 } else { 0 };

    let query_chars: Vec<char> = query.chars().collect();
    let candidate_chars: Vec<char> = candidate.chars().collect();

    let mut qi = 0;
    let mut score: i32 = 0;
    let mut prev_match_idx: Option<usize> = None;

    for (ci, &cc) in candidate_chars.iter().enumerate() {
        if qi < query_chars.len() && cc == query_chars[qi] {
            if ci == qi {
                score += 3;
            } else {
                score += 1;
            }
            if let Some(prev) = prev_match_idx
                && ci == prev + 1
            {
                score += 2;
            }
            prev_match_idx = Some(ci);
            qi += 1;
        }
    }

    if qi < query_chars.len() {
        return 0;
    }

    let length_penalty = (candidate.len() as i32 - query.len() as i32).abs();

    score + prefix_bonus - length_penalty
}

/// Strips ANSI escape sequences (e.g. color codes) from a string.
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&ch) = chars.peek() {
                    chars.next();
                    if ('@'..='~').contains(&ch) {
                        break;
                    }
                }
            } else {
                chars.next();
            }
        } else {
            out.push(c);
        }
    }
    out
}
