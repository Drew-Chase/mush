use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use digest::Digest;
use sha2::Sha256;

use crate::cli::Sha256sumConfig;

pub fn hash_file(path: &Path) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn hash_reader(reader: &mut dyn Read) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn format_hash(hash: &str, filename: &str, config: &Sha256sumConfig) -> String {
    if config.tag {
        format!("SHA256 ({filename}) = {hash}")
    } else {
        let mode = if config.binary { " *" } else { "  " };
        format!("{hash}{mode}{filename}")
    }
}

pub fn check_file(path: &Path, config: &Sha256sumConfig) -> io::Result<(usize, usize)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut ok_count = 0usize;
    let mut fail_count = 0usize;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (expected_hash, filename) = if let Some(rest) = line.strip_prefix("SHA256 (") {
            // BSD-style: SHA256 (filename) = hash
            if let Some(paren_pos) = rest.find(") = ") {
                let fname = &rest[..paren_pos];
                let hash = &rest[paren_pos + 4..];
                (hash.to_string(), fname.to_string())
            } else {
                if config.warn {
                    eprintln!(
                        "sha256sum: {}: improperly formatted checksum line",
                        path.display()
                    );
                }
                continue;
            }
        } else if let Some((hash, fname)) = line.split_once("  ").or_else(|| line.split_once(" *"))
        {
            (hash.to_string(), fname.to_string())
        } else {
            if config.warn {
                eprintln!(
                    "sha256sum: {}: improperly formatted checksum line",
                    path.display()
                );
            }
            continue;
        };

        let actual = match hash_file(Path::new(&filename)) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("sha256sum: {filename}: {e}");
                fail_count += 1;
                continue;
            }
        };

        if actual == expected_hash {
            ok_count += 1;
            if !config.quiet && !config.status {
                println!("{filename}: OK");
            }
        } else {
            fail_count += 1;
            if !config.status {
                println!("{filename}: FAILED");
            }
        }
    }

    Ok((ok_count, fail_count))
}
