use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::cli::{InteractiveMode, RmConfig};

pub fn remove_path(
    path: &Path,
    config: &RmConfig,
    reader: &mut dyn BufRead,
    writer: &mut dyn Write,
) -> io::Result<()> {
    if !path.exists() {
        if config.force {
            return Ok(());
        }
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "cannot remove '{}': No such file or directory",
                path.display()
            ),
        ));
    }

    if config.preserve_root && is_root_path(path) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "it is dangerous to operate recursively on '/'\n\
             (use --no-preserve-root to override this failsafe)",
        ));
    }

    let is_dir = path.is_dir();

    if is_dir && !config.recursive && !config.dir {
        return Err(io::Error::other(format!(
            "cannot remove '{}': Is a directory",
            path.display()
        )));
    }

    if config.interactive == InteractiveMode::Always {
        let description = if is_dir {
            if config.recursive {
                format!("descend into directory '{}'", path.display())
            } else {
                format!("remove empty directory '{}'", path.display())
            }
        } else {
            format!("remove file '{}'", path.display())
        };
        if !confirm(&description, reader, writer) {
            return Ok(());
        }
    }

    if is_dir {
        if config.recursive {
            remove_dir_recursive(path, config, reader, writer)?;
        } else {
            fs::remove_dir(path)?;
            if config.verbose {
                eprintln!("removed directory '{}'", path.display());
            }
        }
    } else {
        fs::remove_file(path)?;
        if config.verbose {
            eprintln!("removed '{}'", path.display());
        }
    }

    Ok(())
}

fn remove_dir_recursive(
    path: &Path,
    config: &RmConfig,
    reader: &mut dyn BufRead,
    writer: &mut dyn Write,
) -> io::Result<()> {
    if config.verbose || config.interactive == InteractiveMode::Always {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                if config.interactive == InteractiveMode::Always {
                    let desc = format!("descend into directory '{}'", entry_path.display());
                    if !confirm(&desc, reader, writer) {
                        continue;
                    }
                }
                remove_dir_recursive(&entry_path, config, reader, writer)?;
            } else {
                if config.interactive == InteractiveMode::Always {
                    let desc = format!("remove file '{}'", entry_path.display());
                    if !confirm(&desc, reader, writer) {
                        continue;
                    }
                }
                fs::remove_file(&entry_path)?;
                if config.verbose {
                    eprintln!("removed '{}'", entry_path.display());
                }
            }
        }
        if config.interactive == InteractiveMode::Always {
            let desc = format!("remove directory '{}'", path.display());
            if !confirm(&desc, reader, writer) {
                return Ok(());
            }
        }
        fs::remove_dir(path)?;
        if config.verbose {
            eprintln!("removed directory '{}'", path.display());
        }
    } else {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

fn confirm(prompt_msg: &str, reader: &mut dyn BufRead, writer: &mut dyn Write) -> bool {
    let _ = write!(writer, "rm: {}? ", prompt_msg);
    let _ = writer.flush();
    let mut response = String::new();
    if reader.read_line(&mut response).is_err() {
        return false;
    }
    let trimmed = response.trim().to_lowercase();
    trimmed == "y" || trimmed == "yes"
}

fn is_root_path(path: &Path) -> bool {
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };

    #[cfg(unix)]
    {
        canonical == Path::new("/")
    }

    #[cfg(windows)]
    {
        let s = canonical.to_string_lossy();
        s.len() == 3 && s.as_bytes()[1] == b':' && s.ends_with('\\')
    }
}
