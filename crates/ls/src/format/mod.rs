mod comma;
mod grid;
mod long;
mod single;

use std::io::{self, Write};

use crate::cli::{ClassifyMode, FormatMode, ResolvedConfig};
use crate::color::ColorScheme;
use crate::entry::{FileEntry, FileType};

pub fn write_output(
    entries: &[FileEntry],
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
) -> io::Result<()> {
    match config.format_mode {
        FormatMode::Long => long::write(entries, config, colors, out),
        FormatMode::Grid => grid::write(entries, config, colors, out),
        FormatMode::SingleColumn => single::write(entries, config, colors, out),
        FormatMode::Commas => comma::write(entries, config, colors, out),
    }
}

pub fn format_name(entry: &FileEntry, config: &ResolvedConfig, colors: &ColorScheme) -> String {
    let name = if config.escape_nongraphic {
        escape_nongraphic(&entry.name)
    } else {
        entry.name.clone()
    };

    let colored = colors.colorize(&name, entry);
    let suffix = classify_suffix(entry, config.classify);

    if let Some(target) = &entry.symlink_target {
        format!("{colored}{suffix} -> {}", target.display())
    } else {
        format!("{colored}{suffix}")
    }
}

fn classify_suffix(entry: &FileEntry, mode: ClassifyMode) -> &'static str {
    match mode {
        ClassifyMode::None => "",
        ClassifyMode::SlashDirs => {
            if entry.file_type == FileType::Directory {
                "/"
            } else {
                ""
            }
        }
        ClassifyMode::All | ClassifyMode::FileTypeOnly => match entry.file_type {
            FileType::Directory => "/",
            FileType::Symlink => "@",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::Regular if mode == ClassifyMode::All && entry.permissions.executable => "*",
            _ => "",
        },
    }
}

fn escape_nongraphic(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_control() || !c.is_ascii_graphic() && !c.is_ascii_whitespace() {
            for byte in c.to_string().bytes() {
                out.push_str(&format!("\\{byte:03o}"));
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn format_size(bytes: u64, config: &ResolvedConfig) -> String {
    if let Some(block) = config.block_size {
        let scaled = bytes.div_ceil(block);
        return format!("{scaled}");
    }

    if config.human_readable {
        const UNITS: &[&str] = &["", "K", "M", "G", "T", "P"];
        let mut size = bytes as f64;
        let mut idx = 0;
        while size >= 1024.0 && idx + 1 < UNITS.len() {
            size /= 1024.0;
            idx += 1;
        }
        if idx == 0 {
            format!("{bytes}")
        } else if size < 10.0 {
            format!("{size:.1}{}", UNITS[idx])
        } else {
            format!("{:.0}{}", size, UNITS[idx])
        }
    } else {
        format!("{bytes}")
    }
}
