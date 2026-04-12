use std::io::{self, BufRead, Read, Write};

use crate::cli::{CutConfig, CutMode, Range};

/// Check if a 1-based index is selected by the range list,
/// then apply complement if needed.
fn is_selected(idx: usize, ranges: &[Range], complement: bool) -> bool {
    let matched = ranges.iter().any(|r| r.contains(idx));
    if complement { !matched } else { matched }
}

pub fn cut_line(line: &str, config: &CutConfig) -> Option<String> {
    match config.mode.as_ref().expect("mode must be resolved before calling cut_line") {
        CutMode::Bytes(ranges) => {
            let bytes = line.as_bytes();
            let selected: Vec<u8> = bytes
                .iter()
                .enumerate()
                .filter(|(i, _)| is_selected(i + 1, ranges, config.complement))
                .map(|(_, &b)| b)
                .collect();
            Some(String::from_utf8_lossy(&selected).into_owned())
        }
        CutMode::Characters(ranges) => {
            let selected: String = line
                .chars()
                .enumerate()
                .filter(|(i, _)| is_selected(i + 1, ranges, config.complement))
                .map(|(_, c)| c)
                .collect();
            Some(selected)
        }
        CutMode::Fields(ranges) => {
            let delim_str: &str = &config.delimiter_char().to_string();

            // If line has no delimiter and -s is set, suppress the line
            if !line.contains(delim_str) {
                if config.only_delimited {
                    return None;
                } else {
                    return Some(line.to_string());
                }
            }

            let fields: Vec<&str> = line.split(delim_str).collect();
            let selected: Vec<&str> = fields
                .iter()
                .enumerate()
                .filter(|(i, _)| is_selected(i + 1, ranges, config.complement))
                .map(|(_, &f)| f)
                .collect();

            let out_delim = config
                .output_delimiter
                .as_deref()
                .unwrap_or(delim_str);

            Some(selected.join(out_delim))
        }
    }
}

pub fn cut(input: &mut dyn Read, output: &mut dyn Write, config: &CutConfig) -> io::Result<()> {
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let line = line?;
        if let Some(result) = cut_line(&line, config) {
            writeln!(output, "{result}")?;
        }
    }

    Ok(())
}
