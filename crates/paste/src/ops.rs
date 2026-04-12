use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::PasteConfig;

pub fn paste(
    inputs: &mut [Box<dyn Read>],
    output: &mut dyn Write,
    config: &PasteConfig,
) -> io::Result<()> {
    let delim_chars: Vec<char> = parse_delimiters(&config.delimiters);

    if config.serial {
        paste_serial(inputs, output, &delim_chars)
    } else {
        paste_parallel(inputs, output, &delim_chars)
    }
}

fn parse_delimiters(s: &str) -> Vec<char> {
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '0' => result.push('\0'),
                other => result.push(other),
            }
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    if result.is_empty() {
        result.push('\t');
    }
    result
}

fn paste_parallel(
    inputs: &mut [Box<dyn Read>],
    output: &mut dyn Write,
    delim_chars: &[char],
) -> io::Result<()> {
    let mut readers: Vec<_> = inputs.iter_mut().map(BufReader::new).collect();
    let mut lines: Vec<Option<String>> = vec![None; readers.len()];

    loop {
        let mut any_line = false;

        for (idx, reader) in readers.iter_mut().enumerate() {
            let mut line = String::new();
            let n = reader.read_line(&mut line)?;
            if n > 0 {
                // Strip trailing newline
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                lines[idx] = Some(line);
                any_line = true;
            } else {
                lines[idx] = None;
            }
        }

        if !any_line {
            break;
        }

        let mut first = true;
        let mut delim_idx = 0;
        for line_opt in &lines {
            if !first {
                let d = delim_chars[delim_idx % delim_chars.len()];
                if d != '\0' {
                    write!(output, "{d}")?;
                }
                delim_idx += 1;
            }
            first = false;
            if let Some(line) = line_opt {
                write!(output, "{line}")?;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}

fn paste_serial(
    inputs: &mut [Box<dyn Read>],
    output: &mut dyn Write,
    delim_chars: &[char],
) -> io::Result<()> {
    for input in inputs.iter_mut() {
        let reader = BufReader::new(input);
        let mut first = true;
        let mut delim_idx = 0;

        for line_result in reader.lines() {
            let line = line_result?;
            if !first {
                let d = delim_chars[delim_idx % delim_chars.len()];
                if d != '\0' {
                    write!(output, "{d}")?;
                }
                delim_idx += 1;
            }
            first = false;
            write!(output, "{line}")?;
        }
        writeln!(output)?;
    }

    Ok(())
}
