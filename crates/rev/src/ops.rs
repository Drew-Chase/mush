use std::io::{BufRead, Write};

pub fn rev_line(line: &str) -> String {
    line.chars().rev().collect()
}

pub fn rev_stream(input: &mut dyn BufRead, output: &mut dyn Write) -> std::io::Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = input.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let trimmed = line.strip_suffix('\n').unwrap_or(&line);
        let trimmed = trimmed.strip_suffix('\r').unwrap_or(trimmed);
        let reversed = rev_line(trimmed);
        writeln!(output, "{reversed}")?;
    }
    output.flush()?;
    Ok(())
}
