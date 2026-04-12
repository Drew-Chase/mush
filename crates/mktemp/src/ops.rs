use std::fs;
use std::io;
use std::path::PathBuf;

use crate::cli::MktempConfig;

/// Generate a random alphanumeric character.
fn random_alphanum() -> char {
    // Simple LCG-based random using thread-local state seeded from time
    use std::cell::Cell;
    use std::time::SystemTime;

    thread_local! {
        static STATE: Cell<u64> = Cell::new(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        );
    }

    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    STATE.with(|s| {
        let val = s.get().wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        s.set(val);
        CHARS[(val >> 33) as usize % CHARS.len()] as char
    })
}

/// Replace each 'X' in the template with a random alphanumeric character.
pub fn expand_template(template: &str) -> String {
    template
        .chars()
        .map(|c| if c == 'X' { random_alphanum() } else { c })
        .collect()
}

/// Create a temporary file or directory based on the config.
pub fn mktemp(config: &MktempConfig) -> io::Result<PathBuf> {
    let template = config
        .template
        .as_deref()
        .unwrap_or("tmp.XXXXXXXXXX");

    // Count trailing X's in template
    let x_count = template.chars().rev().take_while(|&c| c == 'X').count();
    if x_count < 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("too few X's in template '{template}'"),
        ));
    }

    let base_dir = if let Some(ref dir) = config.tmpdir {
        PathBuf::from(dir)
    } else {
        std::env::temp_dir()
    };

    // Try up to 100 times to find a unique name
    for _ in 0..100 {
        let name = expand_template(template);
        let suffix = config.suffix.as_deref().unwrap_or("");
        let full_name = format!("{name}{suffix}");
        let path = base_dir.join(&full_name);

        if path.exists() {
            continue;
        }

        if config.dry_run {
            return Ok(path);
        }

        if config.directory {
            match fs::create_dir(&path) {
                Ok(()) => return Ok(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        } else {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(_) => return Ok(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "failed to create temp file after 100 attempts",
    ))
}

/// Run mktemp and print the resulting path.
pub fn run(config: &MktempConfig) -> io::Result<()> {
    let path = mktemp(config)?;
    println!("{}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_template_replaces_xs() {
        let result = expand_template("tmp.XXXXXX");
        assert_eq!(result.len(), "tmp.XXXXXX".len());
        assert!(result.starts_with("tmp."));
        // No X's should remain
        assert!(!result[4..].contains('X'));
    }

    #[test]
    fn test_expand_template_preserves_non_x() {
        let result = expand_template("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_too_few_xs() {
        let config = MktempConfig {
            template: Some("tmpXX".to_string()),
            ..Default::default()
        };
        assert!(mktemp(&config).is_err());
    }
}
