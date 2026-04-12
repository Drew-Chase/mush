use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::cli::{CpConfig, OverwriteMode};

pub fn copy_path(
    source: &Path,
    dest: &Path,
    config: &CpConfig,
    reader: &mut dyn BufRead,
    writer: &mut dyn Write,
) -> io::Result<()> {
    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "cannot stat '{}': No such file or directory",
                source.display()
            ),
        ));
    }

    if source.is_dir() && !config.recursive {
        return Err(io::Error::other(format!(
            "-r not specified; omitting directory '{}'",
            source.display()
        )));
    }

    if dest.exists() {
        match config.overwrite() {
            OverwriteMode::NoClobber => return Ok(()),
            OverwriteMode::Interactive => {
                if !confirm(
                    &format!("overwrite '{}'?", dest.display()),
                    reader,
                    writer,
                ) {
                    return Ok(());
                }
            }
            OverwriteMode::Force => {}
        }

        if config.update && is_dest_newer(source, dest)? {
            return Ok(());
        }
    }

    if source.is_dir() {
        copy_dir_recursive(source, dest)?;
    } else {
        if let Some(parent) = dest.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, dest)?;
    }

    if config.verbose {
        eprintln!("'{}' -> '{}'", source.display(), dest.display());
    }

    Ok(())
}

fn confirm(prompt_msg: &str, reader: &mut dyn BufRead, writer: &mut dyn Write) -> bool {
    let _ = write!(writer, "cp: {prompt_msg} ");
    let _ = writer.flush();
    let mut response = String::new();
    if reader.read_line(&mut response).is_err() {
        return false;
    }
    let trimmed = response.trim().to_lowercase();
    trimmed == "y" || trimmed == "yes"
}

fn is_dest_newer(source: &Path, dest: &Path) -> io::Result<bool> {
    let src_modified = source.metadata()?.modified()?;
    let dest_modified = dest.metadata()?.modified()?;
    Ok(dest_modified >= src_modified)
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
