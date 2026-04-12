use std::fs;
use std::io;
use std::path::Path;

use crate::cli::InstallConfig;

/// Create directories (install -d mode).
pub fn create_directories(dirs: &[String], config: &InstallConfig) -> io::Result<()> {
    for dir in dirs {
        let path = Path::new(dir);
        fs::create_dir_all(path)?;
        if config.verbose {
            eprintln!("install: creating directory '{dir}'");
        }
        if let Some(ref mode) = config.mode {
            set_mode(path, mode)?;
        }
    }
    Ok(())
}

/// Install (copy) files to a destination.
pub fn install_files(config: &InstallConfig) -> io::Result<()> {
    if config.directory_mode {
        return create_directories(&config.files, config);
    }

    if let Some(ref target_dir) = config.target_dir {
        // Copy all files into target_dir
        let target = Path::new(target_dir);
        if !target.is_dir() {
            fs::create_dir_all(target)?;
        }
        for src in &config.files {
            let src_path = Path::new(src);
            let file_name = src_path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no file name"))?;
            let dest_path = target.join(file_name);
            install_single(src_path, &dest_path, config)?;
        }
        return Ok(());
    }

    // Standard mode: last arg is dest, rest are sources
    if config.files.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing destination operand",
        ));
    }

    let dest = &config.files[config.files.len() - 1];
    let sources = &config.files[..config.files.len() - 1];
    let dest_path = Path::new(dest);

    if sources.len() > 1 {
        // Multiple sources: dest must be a directory
        if !dest_path.is_dir() {
            if config.create_leading {
                fs::create_dir_all(dest_path)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("target '{dest}' is not a directory"),
                ));
            }
        }
        for src in sources {
            let src_path = Path::new(src);
            let file_name = src_path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no file name"))?;
            let final_dest = dest_path.join(file_name);
            install_single(src_path, &final_dest, config)?;
        }
    } else {
        // Single source to dest
        if config.create_leading
            && let Some(parent) = dest_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }
        let src_path = Path::new(&sources[0]);
        if dest_path.is_dir() {
            let file_name = src_path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no file name"))?;
            install_single(src_path, &dest_path.join(file_name), config)?;
        } else {
            install_single(src_path, dest_path, config)?;
        }
    }

    Ok(())
}

/// Install a single file from src to dest.
fn install_single(src: &Path, dest: &Path, config: &InstallConfig) -> io::Result<()> {
    if config.compare && dest.exists() && files_identical(src, dest)? {
        if config.verbose {
            eprintln!(
                "install: '{}' and '{}' are identical, skipping",
                src.display(),
                dest.display()
            );
        }
        return Ok(());
    }

    fs::copy(src, dest)?;

    if let Some(ref mode) = config.mode {
        set_mode(dest, mode)?;
    }

    if config.verbose {
        eprintln!(
            "install: '{}' -> '{}'",
            src.display(),
            dest.display()
        );
    }

    Ok(())
}

/// Check if two files have identical contents.
fn files_identical(a: &Path, b: &Path) -> io::Result<bool> {
    let meta_a = fs::metadata(a)?;
    let meta_b = fs::metadata(b)?;

    if meta_a.len() != meta_b.len() {
        return Ok(false);
    }

    let content_a = fs::read(a)?;
    let content_b = fs::read(b)?;
    Ok(content_a == content_b)
}

/// Set file permissions from mode string.
#[cfg(unix)]
fn set_mode(path: &Path, mode_str: &str) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mode = u32::from_str_radix(mode_str, 8)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("invalid mode: {e}")))?;
    let perms = fs::Permissions::from_mode(mode);
    fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
fn set_mode(path: &Path, mode_str: &str) -> io::Result<()> {
    let mode = u32::from_str_radix(mode_str, 8)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("invalid mode: {e}")))?;
    let readonly = (mode & 0o222) == 0;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(readonly);
    fs::set_permissions(path, perms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files_identical_same_content() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "hello").unwrap();
        fs::write(&b, "hello").unwrap();
        assert!(files_identical(&a, &b).unwrap());
    }

    #[test]
    fn test_files_identical_different_content() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "hello").unwrap();
        fs::write(&b, "world").unwrap();
        assert!(!files_identical(&a, &b).unwrap());
    }

    #[test]
    fn test_files_identical_different_size() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "hello").unwrap();
        fs::write(&b, "hi").unwrap();
        assert!(!files_identical(&a, &b).unwrap());
    }
}
