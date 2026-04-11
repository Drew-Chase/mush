use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn tail_lines(input: &mut dyn Read, n: usize, writer: &mut dyn Write) -> io::Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    let lines: Vec<&str> = buf.lines().collect();
    let start = lines.len().saturating_sub(n);
    for line in &lines[start..] {
        writeln!(writer, "{line}")?;
    }

    Ok(())
}

pub fn tail_bytes(input: &mut dyn Read, n: usize, writer: &mut dyn Write) -> io::Result<()> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    let start = buf.len().saturating_sub(n);
    writer.write_all(&buf[start..])?;

    Ok(())
}

pub fn follow_file(path: &Path, writer: &mut dyn Write) -> io::Result<()> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::End(0))?;

    let mut buf = [0u8; 4096];
    loop {
        let n = file.read(&mut buf)?;
        if n > 0 {
            writer.write_all(&buf[..n])?;
            writer.flush()?;
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
