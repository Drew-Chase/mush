use std::io::{self, Read, Write};

use crate::cli::StringsConfig;

fn is_printable(b: u8) -> bool {
    (0x20..=0x7E).contains(&b)
}

pub fn strings(
    input: &mut dyn Read,
    output: &mut dyn Write,
    config: &StringsConfig,
) -> io::Result<()> {
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    let mut run_start: Option<usize> = None;
    let mut run = Vec::new();

    for (offset, &byte) in data.iter().enumerate() {
        if is_printable(byte) {
            if run.is_empty() {
                run_start = Some(offset);
            }
            run.push(byte);
        } else {
            if run.len() >= config.min_length {
                let s: String = run.iter().map(|&b| b as char).collect();
                write_string(output, &s, run_start.unwrap_or(0), config)?;
            }
            run.clear();
            run_start = None;
        }
    }

    // Handle run at end of file
    if run.len() >= config.min_length {
        let s: String = run.iter().map(|&b| b as char).collect();
        write_string(output, &s, run_start.unwrap_or(0), config)?;
    }

    Ok(())
}

fn write_string(
    output: &mut dyn Write,
    s: &str,
    offset: usize,
    config: &StringsConfig,
) -> io::Result<()> {
    match config.radix {
        Some('o') => writeln!(output, "{offset:>7o} {s}"),
        Some('x') => writeln!(output, "{offset:>7x} {s}"),
        Some('d') => writeln!(output, "{offset:>7} {s}"),
        _ => writeln!(output, "{s}"),
    }
}
