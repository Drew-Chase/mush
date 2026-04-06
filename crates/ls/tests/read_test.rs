use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use ls::cli::{HiddenMode, ResolvedConfig};
use ls::read::read_entries;

fn setup_dir() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("visible.txt"), "hello").unwrap();
    fs::write(dir.path().join(".hidden"), "secret").unwrap();
    fs::write(dir.path().join("backup.txt~"), "old").unwrap();
    fs::create_dir(dir.path().join("subdir")).unwrap();
    dir
}

fn entry_names(dir: &std::path::Path, config: &ResolvedConfig) -> Vec<String> {
    let entries = read_entries(dir, config).unwrap();
    let mut names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    names.sort();
    names
}

#[test]
fn default_filters_hidden() {
    let dir = setup_dir();
    let config = ResolvedConfig {
        show_hidden: HiddenMode::None,
        ..Default::default()
    };
    let names = entry_names(dir.path(), &config);
    assert!(!names.contains(&".hidden".to_string()));
    assert!(names.contains(&"visible.txt".to_string()));
    assert!(names.contains(&"subdir".to_string()));
}

#[test]
fn all_shows_hidden_with_dots() {
    let dir = setup_dir();
    let config = ResolvedConfig {
        show_hidden: HiddenMode::All,
        ..Default::default()
    };
    let names = entry_names(dir.path(), &config);
    assert!(names.contains(&".hidden".to_string()));
    assert!(names.contains(&".".to_string()), "-a should include .");
    assert!(names.contains(&"..".to_string()), "-a should include ..");
}

#[test]
fn almost_all_shows_hidden_without_dots() {
    let dir = setup_dir();
    let config = ResolvedConfig {
        show_hidden: HiddenMode::AlmostAll,
        ..Default::default()
    };
    let names = entry_names(dir.path(), &config);
    assert!(names.contains(&".hidden".to_string()));
    assert!(!names.contains(&".".to_string()), "-A should not include .");
    assert!(
        !names.contains(&"..".to_string()),
        "-A should not include .."
    );
}

#[test]
fn ignore_backups() {
    let dir = setup_dir();
    let config = ResolvedConfig {
        show_hidden: HiddenMode::None,
        ignore_backups: true,
        ..Default::default()
    };
    let names = entry_names(dir.path(), &config);
    assert!(!names.contains(&"backup.txt~".to_string()));
    assert!(names.contains(&"visible.txt".to_string()));
}

#[test]
fn directory_mode_lists_dir_itself() {
    let dir = setup_dir();
    let config = ResolvedConfig {
        directory_mode: true,
        ..Default::default()
    };
    let entries = read_entries(dir.path(), &config).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].file_type,
        ls::entry::FileType::Directory,
    );
}

#[test]
fn empty_directory() {
    let dir = TempDir::new().unwrap();
    let config = ResolvedConfig::default();
    let entries = read_entries(dir.path(), &config).unwrap();
    assert!(entries.is_empty());
}

#[test]
fn nonexistent_path_returns_error() {
    let config = ResolvedConfig::default();
    let result = read_entries(&PathBuf::from("nonexistent_path_12345"), &config);
    assert!(result.is_err());
}
