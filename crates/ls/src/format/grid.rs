use std::io::{self, Write};

use crate::cli::ResolvedConfig;
use crate::color::{visible_width, ColorScheme};
use crate::entry::FileEntry;

use super::format_name;

const MIN_COL_GAP: usize = 2;

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
        .map(|e| {
            let mut prefix = String::new();
            if config.show_inode {
                prefix.push_str(&format!("{:>8} ", e.inode.unwrap_or(0)));
            }
            if config.show_blocks {
                prefix.push_str(&format!("{:>4} ", e.blocks.unwrap_or(0)));
            }
            prefix.push_str(&format_name(e, config, colors));
            prefix
        })
        .collect();

    let widths: Vec<usize> = names.iter().map(|n| visible_width(n)).collect();
    let term_width = config.terminal_width as usize;

    let col_count = find_column_count(&widths, term_width);

    if col_count <= 1 {
        for name in &names {
            writeln!(out, "{name}")?;
        }
        return Ok(());
    }

    let row_count = names.len().div_ceil(col_count);

    let mut col_widths = vec![0usize; col_count];
    for (i, w) in widths.iter().enumerate() {
        let col = i / row_count;
        if col < col_count {
            col_widths[col] = col_widths[col].max(*w);
        }
    }

    for row in 0..row_count {
        for (col, col_w) in col_widths.iter().enumerate() {
            let idx = col * row_count + row;
            if idx >= names.len() {
                break;
            }
            let name = &names[idx];
            let vw = widths[idx];

            if col + 1 < col_count {
                let pad = col_w.saturating_sub(vw) + MIN_COL_GAP;
                write!(out, "{name}{}", " ".repeat(pad))?;
            } else {
                write!(out, "{name}")?;
            }
        }
        writeln!(out)?;
    }

    Ok(())
}

fn find_column_count(widths: &[usize], term_width: usize) -> usize {
    let n = widths.len();
    if n == 0 {
        return 1;
    }

    for cols in (2..=n).rev() {
        let rows = n.div_ceil(cols);
        let mut col_widths = vec![0usize; cols];

        for (i, w) in widths.iter().enumerate() {
            let col = i / rows;
            col_widths[col] = col_widths[col].max(*w);
        }

        let total: usize = col_widths.iter().sum::<usize>() + (cols - 1) * MIN_COL_GAP;
        if total <= term_width {
            return cols;
        }
    }

    1
}
