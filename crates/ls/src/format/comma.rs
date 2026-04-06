use std::io::{self, Write};

use crate::cli::ResolvedConfig;
use crate::color::{visible_width, ColorScheme};
use crate::entry::FileEntry;

use super::format_name;

pub fn write(
    entries: &[FileEntry],
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
) -> io::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let names: Vec<String> = entries
        .iter()
        .map(|e| format_name(e, config, colors))
        .collect();

    let term_width = config.terminal_width as usize;
    let mut col = 0;

    for (i, name) in names.iter().enumerate() {
        let vw = visible_width(name);
        let separator = if i + 1 < names.len() { ", " } else { "" };
        let needed = vw + separator.len();

        if col > 0 && col + needed > term_width {
            writeln!(out)?;
            col = 0;
        }

        write!(out, "{name}{separator}")?;
        col += needed;
    }

    if col > 0 {
        writeln!(out)?;
    }

    Ok(())
}
