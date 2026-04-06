use std::io::Cursor;

use tempfile::TempDir;

use rm::cli::{InteractiveMode, RmConfig};
use rm::ops::remove_path;

fn default_config() -> RmConfig {
    RmConfig::default()
}

fn noop_io() -> (Cursor<Vec<u8>>, Vec<u8>) {
    (Cursor::new(Vec::new()), Vec::new())
}

#[test]
fn remove_single_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    remove_path(&file, &config, &mut reader, &mut writer).unwrap();
    assert!(!file.exists());
}

#[test]
fn remove_nonexistent_without_force_fails() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("nonexistent");

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    assert!(remove_path(&file, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn remove_nonexistent_with_force_succeeds() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("nonexistent");

    let config = RmConfig {
        force: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    remove_path(&file, &config, &mut reader, &mut writer).unwrap();
}

#[test]
fn remove_empty_directory_with_dir_flag() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("empty_dir");
    std::fs::create_dir(&dir).unwrap();

    let config = RmConfig {
        dir: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    remove_path(&dir, &config, &mut reader, &mut writer).unwrap();
    assert!(!dir.exists());
}

#[test]
fn remove_directory_without_flags_fails() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("a_dir");
    std::fs::create_dir(&dir).unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    assert!(remove_path(&dir, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn remove_directory_recursively() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("parent");
    std::fs::create_dir_all(dir.join("child")).unwrap();
    std::fs::write(dir.join("child/file.txt"), "data").unwrap();
    std::fs::write(dir.join("top.txt"), "top").unwrap();

    let config = RmConfig {
        recursive: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    remove_path(&dir, &config, &mut reader, &mut writer).unwrap();
    assert!(!dir.exists());
}

#[test]
fn remove_non_empty_directory_with_dir_only_fails() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("nonempty");
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("file.txt"), "content").unwrap();

    let config = RmConfig {
        dir: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    assert!(remove_path(&dir, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn remove_multiple_files() {
    let tmp = TempDir::new().unwrap();
    let config = default_config();
    for name in &["a.txt", "b.txt", "c.txt"] {
        let file = tmp.path().join(name);
        std::fs::write(&file, "x").unwrap();
        let (mut reader, mut writer) = noop_io();
        remove_path(&file, &config, &mut reader, &mut writer).unwrap();
        assert!(!file.exists());
    }
}

#[test]
fn verbose_does_not_affect_removal() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("verbose_test.txt");
    std::fs::write(&file, "data").unwrap();

    let config = RmConfig {
        verbose: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    remove_path(&file, &config, &mut reader, &mut writer).unwrap();
    assert!(!file.exists());
}

#[test]
fn interactive_always_decline_keeps_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("keep_me.txt");
    std::fs::write(&file, "data").unwrap();

    let config = RmConfig {
        interactive: InteractiveMode::Always,
        ..default_config()
    };
    let mut reader = Cursor::new(b"n\n".to_vec());
    let mut writer = Vec::new();
    remove_path(&file, &config, &mut reader, &mut writer).unwrap();
    assert!(file.exists());
}

#[test]
fn interactive_always_accept_removes_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("remove_me.txt");
    std::fs::write(&file, "data").unwrap();

    let config = RmConfig {
        interactive: InteractiveMode::Always,
        ..default_config()
    };
    let mut reader = Cursor::new(b"y\n".to_vec());
    let mut writer = Vec::new();
    remove_path(&file, &config, &mut reader, &mut writer).unwrap();
    assert!(!file.exists());
}

#[test]
fn deep_recursive_removal() {
    let tmp = TempDir::new().unwrap();
    let deep = tmp.path().join("one/two/three/four");
    std::fs::create_dir_all(&deep).unwrap();
    std::fs::write(deep.join("leaf.txt"), "leaf").unwrap();

    let base = tmp.path().join("one");
    let config = RmConfig {
        recursive: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    remove_path(&base, &config, &mut reader, &mut writer).unwrap();
    assert!(!base.exists());
}
