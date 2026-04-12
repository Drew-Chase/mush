use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::ExpandConfig;

/// Convert tabs to spaces in input, writing result to output.
pub fn expand(input: &mut dyn Read, output: &mut dyn Write, config: &ExpandConfig) -> io::Result<()> {
    let reader = BufReader::new(input);

    for line_result in reader.lines() {
        let line = line_result?;
        let expanded = expand_line(&line, config.tab_width, config.initial_only);
        writeln!(output, "{expanded}")?;
    }

    Ok(())
}

fn expand_line(line: &str, tab_width: usize, initial_only: bool) -> String {
    let mut result = String::with_capacity(line.len());
    let mut col = 0;
    let mut past_initial = false;

    for ch in line.chars() {
        if ch == '\t' && !(initial_only && past_initial) {
            if tab_width == 0 {
                // Tab width 0: just remove tabs
                continue;
            }
            let spaces = tab_width - (col % tab_width);
            for _ in 0..spaces {
                result.push(' ');
            }
            col += spaces;
        } else {
            if ch != ' ' && ch != '\t' {
                past_initial = true;
            }
            result.push(ch);
            col += 1;
        }
    }

    result
}
