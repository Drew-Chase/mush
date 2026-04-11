use std::fs;
use std::io;
use std::path::Path;

use crate::cli::LnConfig;

pub fn create_link(target: &Path, link: &Path, config: &LnConfig) -> io::Result<()> {
    if link.exists() || link.symlink_metadata().is_ok() {
        if config.force {
            if link.is_dir() && !link.symlink_metadata().map(|m| m.is_symlink()).unwrap_or(false) {
                fs::remove_dir(link)?;
            } else {
                fs::remove_file(link)?;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "cannot create link '{}': File exists",
                    link.display()
                ),
            ));
        }
    }

    if config.symbolic {
        symlink(target, link)?;
    } else {
        fs::hard_link(target, link)?;
    }

    if config.verbose {
        if config.symbolic {
            eprintln!("'{}' -> '{}' (symbolic)", link.display(), target.display());
        } else {
            eprintln!("'{}' -> '{}' (hard)", link.display(), target.display());
        }
    }

    Ok(())
}

#[cfg(unix)]
fn symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink(target: &Path, link: &Path) -> io::Result<()> {
    // Resolve the target relative to the link's parent to check if it's a directory
    let resolved = if target.is_absolute() {
        target.to_path_buf()
    } else {
        link.parent().unwrap_or(Path::new(".")).join(target)
    };

    if resolved.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}
