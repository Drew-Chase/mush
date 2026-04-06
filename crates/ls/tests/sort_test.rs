use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use ls::cli::{ResolvedConfig, SortKey, TimeField};
use ls::entry::{FileEntry, FileType, Permissions};
use ls::sort::sort_entries;

fn make_entry(name: &str, size: u64, file_type: FileType) -> FileEntry {
    FileEntry {
        name: name.to_string(),
        path: PathBuf::from(name),
        file_type,
        size,
        modified: Some(SystemTime::UNIX_EPOCH),
        accessed: Some(SystemTime::UNIX_EPOCH),
        created: Some(SystemTime::UNIX_EPOCH),
        permissions: Permissions {
            mode: None,
            readonly: false,
            executable: false,
        },
        owner: None,
        group: None,
        nlinks: 1,
        inode: None,
        blocks: None,
        symlink_target: None,
    }
}

fn names(entries: &[FileEntry]) -> Vec<&str> {
    entries.iter().map(|e| e.name.as_str()).collect()
}

#[test]
fn sort_by_name_case_insensitive() {
    let mut entries = vec![
        make_entry("Charlie", 0, FileType::Regular),
        make_entry("alpha", 0, FileType::Regular),
        make_entry("Bravo", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Name,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["alpha", "Bravo", "Charlie"]);
}

#[test]
fn sort_by_size_largest_first() {
    let mut entries = vec![
        make_entry("small", 10, FileType::Regular),
        make_entry("big", 1000, FileType::Regular),
        make_entry("medium", 500, FileType::Regular),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Size,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["big", "medium", "small"]);
}

#[test]
fn sort_by_time_newest_first() {
    let now = SystemTime::now();
    let old = now - Duration::from_secs(3600);
    let older = now - Duration::from_secs(7200);

    let mut entries = vec![
        make_entry("old", 0, FileType::Regular),
        make_entry("new", 0, FileType::Regular),
        make_entry("oldest", 0, FileType::Regular),
    ];
    entries[0].modified = Some(old);
    entries[1].modified = Some(now);
    entries[2].modified = Some(older);

    let config = ResolvedConfig {
        sort_key: SortKey::Time,
        time_field: TimeField::Modified,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["new", "old", "oldest"]);
}

#[test]
fn sort_by_extension() {
    let mut entries = vec![
        make_entry("file.rs", 0, FileType::Regular),
        make_entry("file.c", 0, FileType::Regular),
        make_entry("file.txt", 0, FileType::Regular),
        make_entry("noext", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Extension,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["noext", "file.c", "file.rs", "file.txt"]);
}

#[test]
fn sort_reversed() {
    let mut entries = vec![
        make_entry("a", 0, FileType::Regular),
        make_entry("b", 0, FileType::Regular),
        make_entry("c", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Name,
        sort_reverse: true,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["c", "b", "a"]);
}

#[test]
fn sort_none_preserves_order() {
    let mut entries = vec![
        make_entry("c", 0, FileType::Regular),
        make_entry("a", 0, FileType::Regular),
        make_entry("b", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::None,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["c", "a", "b"]);
}

#[test]
fn group_directories_first() {
    let mut entries = vec![
        make_entry("file_a", 0, FileType::Regular),
        make_entry("dir_b", 0, FileType::Directory),
        make_entry("file_c", 0, FileType::Regular),
        make_entry("dir_a", 0, FileType::Directory),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Name,
        group_dirs_first: true,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    assert_eq!(names(&entries), vec!["dir_a", "dir_b", "file_a", "file_c"]);
}

#[test]
fn sort_size_reverse_group_dirs() {
    let mut entries = vec![
        make_entry("big.txt", 1000, FileType::Regular),
        make_entry("docs", 0, FileType::Directory),
        make_entry("small.txt", 10, FileType::Regular),
        make_entry("src", 0, FileType::Directory),
    ];
    let config = ResolvedConfig {
        sort_key: SortKey::Size,
        sort_reverse: true,
        group_dirs_first: true,
        ..Default::default()
    };
    sort_entries(&mut entries, &config);
    // dirs first (reversed by size, but both 0), then files reversed (smallest first)
    let result = names(&entries);
    assert!(result[0] == "docs" || result[0] == "src");
    assert!(result[1] == "docs" || result[1] == "src");
    assert_eq!(result[2], "small.txt");
    assert_eq!(result[3], "big.txt");
}
