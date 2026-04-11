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
            FileType::Regular => match extension_color(&entry.name) {
                Some(code) => code,
                None => return name.to_string(),
            },
            _ => return name.to_string(),
        };

        format!("\x1b[{code}m{name}\x1b[0m")
    }
}

/// Returns an ANSI color code for a file based on its extension, or `None` if
/// the extension is not recognized. Follows Linux's default LS_COLORS conventions.
fn extension_color(name: &str) -> Option<&'static str> {
    let ext = name.rsplit('.').next()?;
    let ext_lower: String = ext.to_lowercase();
    match ext_lower.as_str() {
        // Archives — bold red
        "tar" | "gz" | "tgz" | "zip" | "7z" | "rar" | "bz2" | "xz" | "zst" | "lz" | "lzma"
        | "deb" | "rpm" | "jar" | "war" | "cab" | "iso" | "msi" => Some("1;31"),
        // Images — bold magenta
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif"
        | "psd" | "xcf" | "raw" | "cr2" | "nef" | "heic" | "avif" => Some("1;35"),
        // Video — bold magenta
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
        | "ts" => Some("1;35"),
        // Audio — cyan
        "mp3" | "wav" | "flac" | "ogg" | "aac" | "wma" | "m4a" | "opus" | "mid" | "midi" => {
            Some("0;36")
        }
        // Documents — yellow
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp"
        | "rtf" | "epub" | "csv" => Some("0;33"),
        _ => None,
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
