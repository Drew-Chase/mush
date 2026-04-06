use std::cmp::Ordering;
use std::time::SystemTime;

use crate::cli::{ResolvedConfig, SortKey, TimeField};
use crate::entry::{FileEntry, FileType};

pub fn sort_entries(entries: &mut [FileEntry], config: &ResolvedConfig) {
    if config.sort_key == SortKey::None {
        if config.sort_reverse {
            entries.reverse();
        }
        if config.group_dirs_first {
            stable_partition_dirs(entries);
        }
        return;
    }

    entries.sort_by(|a, b| {
        let ord = compare(a, b, config);
        if config.sort_reverse { ord.reverse() } else { ord }
    });

    if config.group_dirs_first {
        stable_partition_dirs(entries);
    }
}

fn compare(a: &FileEntry, b: &FileEntry, config: &ResolvedConfig) -> Ordering {
    match config.sort_key {
        SortKey::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        SortKey::Size => b.size.cmp(&a.size),
        SortKey::Time => {
            let ta = get_time(a, config.time_field);
            let tb = get_time(b, config.time_field);
            tb.cmp(&ta)
        }
        SortKey::Extension => {
            let ea = extension(&a.name);
            let eb = extension(&b.name);
            ea.cmp(eb).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }
        SortKey::None => Ordering::Equal,
    }
}

fn get_time(entry: &FileEntry, field: TimeField) -> Option<SystemTime> {
    match field {
        TimeField::Modified => entry.modified,
        TimeField::Accessed => entry.accessed,
        TimeField::Created => entry.created,
    }
}

fn extension(name: &str) -> &str {
    name.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("")
}

fn stable_partition_dirs(entries: &mut [FileEntry]) {
    // Stable sort by is_directory preserves relative order within each group
    entries.sort_by_key(|e| if e.file_type == FileType::Directory { 0 } else { 1 });
}
