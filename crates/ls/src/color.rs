use std::io::IsTerminal;

use crate::cli::{ColorMode, ResolvedConfig};
use crate::entry::{FileEntry, FileType};

pub struct ColorScheme {
    enabled: bool,
}

impl ColorScheme {
    pub fn new(config: &ResolvedConfig) -> Self {
        let enabled = match config.color_mode {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => std::io::stdout().is_terminal(),
        };
        Self { enabled }
    }

    pub fn colorize(&self, name: &str, entry: &FileEntry) -> String {
        if !self.enabled {
            return name.to_string();
        }

        let code = match entry.file_type {
            FileType::Directory => "1;34",
            FileType::Symlink => {
                if entry
                    .symlink_target
                    .as_ref()
                    .is_some_and(|t| t.exists())
                {
                    "1;36"
                } else {
                    "1;31"
                }
            }
            FileType::Pipe => "33",
            FileType::Socket => "1;35",
            FileType::BlockDevice | FileType::CharDevice => "1;33;40",
            FileType::Regular if entry.permissions.executable => "1;32",
            _ => return name.to_string(),
        };

        format!("\x1b[{code}m{name}\x1b[0m")
    }
}

pub fn visible_width(s: &str) -> usize {
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
