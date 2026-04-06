use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::cli::{MvConfig, OverwriteMode};

pub fn move_path(
    source: &Path,
    dest: &Path,
    config: &MvConfig,
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

    if dest.exists() {
        match config.overwrite {
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

    match fs::rename(source, dest) {
        Ok(()) => {}
        Err(e) if is_cross_device_error(&e) => {
            cross_device_move(source, dest)?;
        }
        Err(e) => return Err(e),
    }

    if config.verbose {
        eprintln!("renamed '{}' -> '{}'", source.display(), dest.display());
    }

    Ok(())
}

fn confirm(prompt_msg: &str, reader: &mut dyn BufRead, writer: &mut dyn Write) -> bool {
    let _ = write!(writer, "mv: {prompt_msg} ");
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

fn is_cross_device_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::CrossesDevices
}

fn cross_device_move(source: &Path, dest: &Path) -> io::Result<()> {
    if source.is_dir() {
        copy_dir_recursive(source, dest)?;
        fs::remove_dir_all(source)?;
    } else {
        fs::copy(source, dest)?;
        fs::remove_file(source)?;
    }
    Ok(())
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
