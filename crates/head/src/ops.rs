use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::HeadConfig;

pub fn head(input: &mut dyn Read, config: &HeadConfig, writer: &mut dyn Write) -> io::Result<()> {
    if let Some(num_bytes) = config.bytes {
        let mut buf = vec![0u8; num_bytes];
        let mut total = 0;
        while total < num_bytes {
            let n = input.read(&mut buf[total..])?;
            if n == 0 {
                break;
            }
            total += n;
        }
        writer.write_all(&buf[..total])?;
    } else {
        let reader = BufReader::new(input);
        for (i, line) in reader.lines().enumerate() {
            if i >= config.lines {
                break;
            }
            let line = line?;
            writeln!(writer, "{line}")?;
        }
    }
    writer.flush()?;
    Ok(())
}
