use std::io::{self, BufWriter, Write};

pub fn yes_loop(string: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    loop {
        writeln!(out, "{string}")?;
    }
}
