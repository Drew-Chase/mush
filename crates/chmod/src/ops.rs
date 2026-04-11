use std::fs;
use std::io;
use std::path::Path;

use crate::cli::ChmodConfig;

/// Parse an octal or symbolic mode string, returning the new mode.
///
/// `current_mode` is used when evaluating symbolic expressions (e.g. `u+x`).
pub fn parse_mode(mode_str: &str, current_mode: u32) -> Result<u32, String> {
    // Try octal first
    if mode_str.chars().all(|c| c.is_ascii_digit()) {
        return u32::from_str_radix(mode_str, 8)
            .map_err(|e| format!("invalid octal mode '{}': {}", mode_str, e));
    }

    // Symbolic mode: comma-separated clauses like u+x,go-w
    let mut mode = current_mode;
    for clause in mode_str.split(',') {
        mode = parse_symbolic_clause(clause.trim(), mode)?;
    }
    Ok(mode)
}

fn parse_symbolic_clause(clause: &str, current: u32) -> Result<u32, String> {
    if clause.is_empty() {
        return Err("empty mode clause".to_string());
    }

    let bytes = clause.as_bytes();
    let mut pos = 0;

    // Parse who: [ugoa]*
    let mut who_user = false;
    let mut who_group = false;
    let mut who_other = false;
    let mut who_all = false;

    while pos < bytes.len() {
        match bytes[pos] {
            b'u' => who_user = true,
            b'g' => who_group = true,
            b'o' => who_other = true,
            b'a' => who_all = true,
            _ => break,
        }
        pos += 1;
    }

    // If no who specified, default to all
    if !who_user && !who_group && !who_other && !who_all {
        who_all = true;
    }
    if who_all {
        who_user = true;
        who_group = true;
        who_other = true;
    }

    if pos >= bytes.len() {
        return Err(format!("invalid mode clause '{clause}'"));
    }

    // Parse operator: [+-=]
    let op = bytes[pos];
    if op != b'+' && op != b'-' && op != b'=' {
        return Err(format!("invalid operator '{}' in clause '{clause}'", op as char));
    }
    pos += 1;

    // Parse permissions: [rwx]*
    let mut perm_bits: u32 = 0;
    while pos < bytes.len() {
        match bytes[pos] {
            b'r' => perm_bits |= 0o4,
            b'w' => perm_bits |= 0o2,
            b'x' => perm_bits |= 0o1,
            _ => return Err(format!("invalid permission character '{}' in clause '{clause}'", bytes[pos] as char)),
        }
        pos += 1;
    }

    // Build the full mask from who + perms
    let mut mask: u32 = 0;
    if who_user {
        mask |= perm_bits << 6;
    }
    if who_group {
        mask |= perm_bits << 3;
    }
    if who_other {
        mask |= perm_bits;
    }

    let result = match op {
        b'+' => current | mask,
        b'-' => current & !mask,
        b'=' => {
            // Clear affected who bits, then set
            let mut clear: u32 = 0;
            if who_user {
                clear |= 0o700;
            }
            if who_group {
                clear |= 0o070;
            }
            if who_other {
                clear |= 0o007;
            }
            (current & !clear) | mask
        }
        _ => unreachable!(),
    };

    Ok(result)
}

pub fn chmod(path: &Path, config: &ChmodConfig) -> io::Result<()> {
    chmod_single(path, config)?;

    if config.recursive && path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            chmod(&entry.path(), config)?;
        }
    }

    Ok(())
}

fn chmod_single(path: &Path, config: &ChmodConfig) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let old_mode = get_mode(&metadata);

    let new_mode = parse_mode(&config.mode, old_mode)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    set_mode(path, new_mode, &metadata)?;

    if config.verbose || (config.changes && old_mode != new_mode) {
        eprintln!(
            "mode of '{}' changed from {:04o} to {:04o}",
            path.display(),
            old_mode,
            new_mode
        );
    }

    Ok(())
}

#[cfg(unix)]
fn get_mode(metadata: &fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o7777
}

#[cfg(not(unix))]
fn get_mode(metadata: &fs::Metadata) -> u32 {
    if metadata.permissions().readonly() {
        0o444
    } else {
        0o666
    }
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32, _metadata: &fs::Metadata) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::Permissions::from_mode(mode);
    fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
fn set_mode(path: &Path, mode: u32, _metadata: &fs::Metadata) -> io::Result<()> {
    // On non-Unix, we can only toggle the readonly flag.
    let readonly = (mode & 0o222) == 0;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(readonly);
    fs::set_permissions(path, perms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_octal_755() {
        assert_eq!(parse_mode("755", 0).unwrap(), 0o755);
    }

    #[test]
    fn test_parse_octal_644() {
        assert_eq!(parse_mode("644", 0).unwrap(), 0o644);
    }

    #[test]
    fn test_parse_symbolic_u_plus_x() {
        assert_eq!(parse_mode("u+x", 0o644).unwrap(), 0o744);
    }

    #[test]
    fn test_parse_symbolic_go_minus_w() {
        assert_eq!(parse_mode("go-w", 0o666).unwrap(), 0o644);
    }

    #[test]
    fn test_parse_symbolic_a_plus_r() {
        assert_eq!(parse_mode("a+r", 0o000).unwrap(), 0o444);
    }

    #[test]
    fn test_parse_symbolic_equals() {
        assert_eq!(parse_mode("u=rwx", 0o000).unwrap(), 0o700);
    }

    #[test]
    fn test_parse_symbolic_combined() {
        assert_eq!(parse_mode("u+x,go-w", 0o666).unwrap(), 0o744);
    }

    #[test]
    fn test_parse_symbolic_default_all() {
        // No who specified means all
        assert_eq!(parse_mode("+x", 0o000).unwrap(), 0o111);
    }

    #[test]
    fn test_parse_invalid_mode() {
        assert!(parse_mode("zzz", 0).is_err());
    }
}
