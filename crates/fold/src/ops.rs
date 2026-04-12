use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::FoldConfig;

/// Fold (wrap) long lines in input to the specified width.
pub fn fold(input: &mut dyn Read, output: &mut dyn Write, config: &FoldConfig) -> io::Result<()> {
    let reader = BufReader::new(input);

    for line_result in reader.lines() {
        let line = line_result?;
        fold_line(&line, output, config)?;
    }

    Ok(())
}

fn fold_line(line: &str, output: &mut dyn Write, config: &FoldConfig) -> io::Result<()> {
    if config.width == 0 {
        writeln!(output, "{line}")?;
        return Ok(());
    }

    if config.bytes {
        fold_bytes(line, output, config)
    } else {
        fold_columns(line, output, config)
    }
}

fn fold_bytes(line: &str, output: &mut dyn Write, config: &FoldConfig) -> io::Result<()> {
    let bytes = line.as_bytes();

    if bytes.len() <= config.width {
        writeln!(output, "{line}")?;
        return Ok(());
    }

    let mut pos = 0;
    while pos < bytes.len() {
        let remaining = bytes.len() - pos;
        if remaining <= config.width {
            output.write_all(&bytes[pos..])?;
            writeln!(output)?;
            break;
        }

        let mut end = pos + config.width;

        if config.spaces {
            // Find last space in range
            if let Some(sp) = bytes[pos..end].iter().rposition(|&b| b == b' ')
                && sp > 0
            {
                end = pos + sp + 1;
            }
        }

        output.write_all(&bytes[pos..end])?;
        writeln!(output)?;
        pos = end;
    }

    Ok(())
}

fn fold_columns(line: &str, output: &mut dyn Write, config: &FoldConfig) -> io::Result<()> {
    let chars: Vec<char> = line.chars().collect();

    if chars.len() <= config.width {
        writeln!(output, "{line}")?;
        return Ok(());
    }

    let mut pos = 0;
    while pos < chars.len() {
        let remaining = chars.len() - pos;
        if remaining <= config.width {
            let s: String = chars[pos..].iter().collect();
            writeln!(output, "{s}")?;
            break;
        }

        let mut end = pos + config.width;

        if config.spaces {
            // Find last space in range
            if let Some(sp) = chars[pos..end].iter().rposition(|&c| c == ' ')
                && sp > 0
            {
                end = pos + sp + 1;
            }
        }

        let s: String = chars[pos..end].iter().collect();
        writeln!(output, "{s}")?;
        pos = end;
    }

    Ok(())
}
