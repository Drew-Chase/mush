use std::io::Cursor;

use tempfile::TempDir;

use mv::cli::{MvConfig, OverwriteMode};
use mv::ops::move_path;

fn default_config() -> MvConfig {
    MvConfig::default()
}

fn noop_io() -> (Cursor<Vec<u8>>, Vec<u8>) {
    (Cursor::new(Vec::new()), Vec::new())
}

#[test]
fn move_single_file() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("source.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "hello").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "hello");
}

#[test]
fn move_nonexistent_fails() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("nonexistent");
    let dest = tmp.path().join("dest");

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    assert!(move_path(&src, &dest, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn overwrite_force() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = MvConfig {
        overwrite: OverwriteMode::Force,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new");
}

#[test]
fn overwrite_noclobber_skips() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = MvConfig {
        overwrite: OverwriteMode::NoClobber,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(src.exists(), "source should still exist");
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "old");
}

#[test]
fn interactive_accept_moves() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = MvConfig {
        overwrite: OverwriteMode::Interactive,
        ..default_config()
    };
    let mut reader = Cursor::new(b"y\n".to_vec());
    let mut writer = Vec::new();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new");
}

#[test]
fn interactive_decline_skips() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = MvConfig {
        overwrite: OverwriteMode::Interactive,
        ..default_config()
    };
    let mut reader = Cursor::new(b"n\n".to_vec());
    let mut writer = Vec::new();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "old");
}

#[test]
fn move_directory() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src_dir");
    let dest = tmp.path().join("dest_dir");
    std::fs::create_dir(&src).unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert!(dest.is_dir());
}

#[test]
fn move_directory_with_contents() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src_dir");
    let dest = tmp.path().join("dest_dir");
    std::fs::create_dir_all(src.join("sub")).unwrap();
    std::fs::write(src.join("file.txt"), "data").unwrap();
    std::fs::write(src.join("sub/nested.txt"), "nested").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(dest.join("file.txt")).unwrap(), "data");
    assert_eq!(
        std::fs::read_to_string(dest.join("sub/nested.txt")).unwrap(),
        "nested"
    );
}

#[test]
fn verbose_does_not_affect_operation() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "data").unwrap();

    let config = MvConfig {
        verbose: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert!(dest.exists());
}

#[test]
fn update_dest_missing_moves() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "data").unwrap();

    let config = MvConfig {
        update: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert!(dest.exists());
}

#[test]
fn move_to_new_name() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("original.txt");
    let dest = tmp.path().join("renamed.txt");
    std::fs::write(&src, "content").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "content");
}

#[test]
fn move_overwrites_existing_by_default() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new content").unwrap();
    std::fs::write(&dest, "old content").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    move_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(!src.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new content");
}
