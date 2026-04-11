use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn tee(input: &mut dyn Read, files: &[String], append: bool) -> io::Result<()> {
    let mut outputs: Vec<File> = Vec::with_capacity(files.len());

    for path in files {
        let file = if append {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?
        } else {
            File::create(path)?
        };
        outputs.push(file);
    }

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut buf = [0u8; 8192];

    loop {
        let n = input.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let chunk = &buf[..n];

        stdout_lock.write_all(chunk)?;
        for file in &mut outputs {
            file.write_all(chunk)?;
        }
    }

    stdout_lock.flush()?;
    for file in &mut outputs {
        file.flush()?;
    }

    Ok(())
}
