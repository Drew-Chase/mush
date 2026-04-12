use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::UnexpandConfig;

/// Convert spaces to tabs in input, writing result to output.
pub fn unexpand(
    input: &mut dyn Read,
    output: &mut dyn Write,
    config: &UnexpandConfig,
) -> io::Result<()> {
    let reader = BufReader::new(input);

    for line_result in reader.lines() {
        let line = line_result?;
        let converted = unexpand_line(&line, config);
        writeln!(output, "{converted}")?;
    }

    Ok(())
}

fn unexpand_line(line: &str, config: &UnexpandConfig) -> String {
    let tab_width = config.tab_width;
    if tab_width == 0 {
        return line.to_string();
    }

    let convert_all = config.all && !config.first_only;

    let mut result = String::with_capacity(line.len());
    let mut col = 0;
    let mut space_count = 0;
    let mut space_start_col = 0;
    let mut in_leading = true;

    for ch in line.chars() {
        if ch == ' ' && (in_leading || convert_all) {
            if space_count == 0 {
                space_start_col = col;
            }
            space_count += 1;
            col += 1;

            // Check if we've reached a tab stop
            if col % tab_width == 0 {
                let spaces_to_tab = col - space_start_col;
                if spaces_to_tab >= 2 {
                    result.push('\t');
                } else {
                    // Just one space to this tab stop, keep as space
                    for _ in 0..spaces_to_tab {
                        result.push(' ');
                    }
                }
                space_count = 0;
            }
        } else {
            // Flush any pending spaces
            for _ in 0..space_count {
                result.push(' ');
            }
            space_count = 0;

            if ch != ' ' && ch != '\t' {
                in_leading = false;
            }

            if ch == '\t' {
                // Tab char: advance to next tab stop
                result.push('\t');
                col = col + (tab_width - col % tab_width);
            } else {
                result.push(ch);
                col += 1;
            }
        }
    }

    // Flush trailing spaces
    for _ in 0..space_count {
        result.push(' ');
    }

    result
}
