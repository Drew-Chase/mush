use std::fs;
use std::io;
use std::path::Path;

use crate::cli::{HiddenMode, ResolvedConfig};
use crate::entry::FileEntry;
use crate::platform;

pub fn read_entries(path: &Path, config: &ResolvedConfig) -> io::Result<Vec<FileEntry>> {
    let meta = path.symlink_metadata()?;

    if config.directory_mode || !meta.is_dir() {
        return Ok(vec![FileEntry::from_path(path.to_path_buf())?]);
    }

    let mut entries = Vec::new();

    if config.show_hidden == HiddenMode::All {
        if let Ok(mut e) = FileEntry::from_path(path.join(".")) {
            e.name = ".".to_string();
            entries.push(e);
        }
        if let Ok(mut e) = FileEntry::from_path(path.join("..")) {
            e.name = "..".to_string();
            entries.push(e);
        }
    }

    for result in fs::read_dir(path)? {
        let dir_entry = match result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("ls: {e}");
                continue;
            }
        };

        let name = dir_entry.file_name().to_string_lossy().into_owned();

        if !should_show(&name, &dir_entry, config) {
            continue;
        }

        match FileEntry::from_dir_entry(&dir_entry) {
            Ok(entry) => entries.push(entry),
            Err(e) => eprintln!("ls: cannot access '{}': {e}", name),
        }
    }

    Ok(entries)
}

fn should_show(name: &str, dir_entry: &fs::DirEntry, config: &ResolvedConfig) -> bool {
    if config.show_hidden == HiddenMode::None {
        if let Ok(meta) = dir_entry.metadata() {
            if platform::is_hidden(name, &meta) {
                return false;
            }
        } else if name.starts_with('.') {
            return false;
        }
    }

    if config.ignore_backups && name.ends_with('~') {
        return false;
    }

    true
}
