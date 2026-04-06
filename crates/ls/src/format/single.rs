use std::io::{self, Write};

use crate::cli::ResolvedConfig;
use crate::color::ColorScheme;
use crate::entry::FileEntry;

use super::format_name;

pub fn write(
    entries: &[FileEntry],
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
) -> io::Result<()> {
    for entry in entries {
        let name = format_name(entry, config, colors);
        if config.show_inode {
            write!(out, "{:>8} ", entry.inode.unwrap_or(0))?;
        }
        if config.show_blocks {
            write!(out, "{:>4} ", entry.blocks.unwrap_or(0))?;
        }
        writeln!(out, "{name}")?;
    }
    Ok(())
}
