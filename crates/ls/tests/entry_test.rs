use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use ls::entry::{FileEntry, FileType};

#[test]
fn from_path_regular_file() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "hello world").unwrap();

    let entry = FileEntry::from_path(file_path).unwrap();
    assert_eq!(entry.name, "test.txt");
    assert_eq!(entry.file_type, FileType::Regular);
    assert_eq!(entry.size, 11);
    assert!(entry.modified.is_some());
}

#[test]
fn from_path_directory() {
    let dir = TempDir::new().unwrap();
    let sub = dir.path().join("subdir");
    fs::create_dir(&sub).unwrap();

    let entry = FileEntry::from_path(sub).unwrap();
    assert_eq!(entry.name, "subdir");
    assert_eq!(entry.file_type, FileType::Directory);
}

#[test]
fn from_path_nonexistent_returns_error() {
    let result = FileEntry::from_path(PathBuf::from("nonexistent_file_99999"));
    assert!(result.is_err());
}

#[test]
fn from_path_empty_file() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("empty");
    fs::write(&file_path, "").unwrap();

    let entry = FileEntry::from_path(file_path).unwrap();
    assert_eq!(entry.size, 0);
    assert_eq!(entry.file_type, FileType::Regular);
}

#[test]
fn from_dir_entry() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("a.txt"), "data").unwrap();
    fs::create_dir(dir.path().join("b_dir")).unwrap();

    let mut found_file = false;
    let mut found_dir = false;

    for de in fs::read_dir(dir.path()).unwrap() {
        let de = de.unwrap();
        let entry = FileEntry::from_dir_entry(&de).unwrap();
        match entry.name.as_str() {
            "a.txt" => {
                assert_eq!(entry.file_type, FileType::Regular);
                assert_eq!(entry.size, 4);
                found_file = true;
            }
            "b_dir" => {
                assert_eq!(entry.file_type, FileType::Directory);
                found_dir = true;
            }
            _ => {}
        }
    }

    assert!(found_file, "should find a.txt");
    assert!(found_dir, "should find b_dir");
}

#[test]
fn timestamps_are_populated() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("timestamped.txt");
    fs::write(&file_path, "x").unwrap();

    let entry = FileEntry::from_path(file_path).unwrap();
    assert!(entry.modified.is_some());
    assert!(entry.accessed.is_some());
    assert!(entry.created.is_some());
}
