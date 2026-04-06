use std::io::Cursor;

use tempfile::TempDir;

use cp::cli::{CpConfig, OverwriteMode};
use cp::ops::copy_path;

fn default_config() -> CpConfig {
    CpConfig::default()
}

fn noop_io() -> (Cursor<Vec<u8>>, Vec<u8>) {
    (Cursor::new(Vec::new()), Vec::new())
}

#[test]
fn copy_single_file() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("source.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "hello").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(src.exists(), "source should still exist");
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "hello");
}

#[test]
fn copy_nonexistent_fails() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("nonexistent");
    let dest = tmp.path().join("dest");

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    assert!(copy_path(&src, &dest, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn copy_directory_without_recursive_fails() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("dir");
    let dest = tmp.path().join("dest");
    std::fs::create_dir(&src).unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    assert!(copy_path(&src, &dest, &config, &mut reader, &mut writer).is_err());
}

#[test]
fn copy_directory_recursively() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src_dir");
    let dest = tmp.path().join("dest_dir");
    std::fs::create_dir_all(src.join("sub")).unwrap();
    std::fs::write(src.join("file.txt"), "data").unwrap();
    std::fs::write(src.join("sub/nested.txt"), "nested").unwrap();

    let config = CpConfig {
        recursive: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(src.exists(), "source should still exist");
    assert_eq!(std::fs::read_to_string(dest.join("file.txt")).unwrap(), "data");
    assert_eq!(
        std::fs::read_to_string(dest.join("sub/nested.txt")).unwrap(),
        "nested"
    );
}

#[test]
fn overwrite_force() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new");
}

#[test]
fn overwrite_noclobber_skips() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = CpConfig {
        overwrite: OverwriteMode::NoClobber,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "old");
}

#[test]
fn interactive_accept_copies() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = CpConfig {
        overwrite: OverwriteMode::Interactive,
        ..default_config()
    };
    let mut reader = Cursor::new(b"y\n".to_vec());
    let mut writer = Vec::new();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new");
}

#[test]
fn interactive_decline_skips() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "new").unwrap();
    std::fs::write(&dest, "old").unwrap();

    let config = CpConfig {
        overwrite: OverwriteMode::Interactive,
        ..default_config()
    };
    let mut reader = Cursor::new(b"n\n".to_vec());
    let mut writer = Vec::new();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "old");
}

#[test]
fn verbose_does_not_affect_operation() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "data").unwrap();

    let config = CpConfig {
        verbose: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(dest.exists());
    assert!(src.exists());
}

#[test]
fn update_dest_missing_copies() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dest = tmp.path().join("dest.txt");
    std::fs::write(&src, "data").unwrap();

    let config = CpConfig {
        update: true,
        ..default_config()
    };
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(dest.exists());
}

#[test]
fn copy_preserves_source() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("original.txt");
    let dest = tmp.path().join("copy.txt");
    std::fs::write(&src, "content").unwrap();

    let config = default_config();
    let (mut reader, mut writer) = noop_io();
    copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(src.exists(), "cp must not remove the source");
    assert_eq!(std::fs::read_to_string(&src).unwrap(), "content");
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "content");
}

#[test]
fn deep_recursive_copy() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("one/two/three");
    let dest = tmp.path().join("copy_root");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("leaf.txt"), "leaf").unwrap();

    let config = CpConfig {
        recursive: true,
        ..default_config()
    };
    let base = tmp.path().join("one");
    let (mut reader, mut writer) = noop_io();
    copy_path(&base, &dest, &config, &mut reader, &mut writer).unwrap();
    assert!(base.exists(), "source should still exist");
    assert_eq!(
        std::fs::read_to_string(dest.join("two/three/leaf.txt")).unwrap(),
        "leaf"
    );
}

#[test]
fn copy_multiple_files() {
    let tmp = TempDir::new().unwrap();
    let config = default_config();
    for name in &["a.txt", "b.txt", "c.txt"] {
        let src = tmp.path().join(name);
        let dest = tmp.path().join(format!("copy_{name}"));
        std::fs::write(&src, name).unwrap();
        let (mut reader, mut writer) = noop_io();
        copy_path(&src, &dest, &config, &mut reader, &mut writer).unwrap();
        assert!(src.exists());
        assert_eq!(std::fs::read_to_string(&dest).unwrap(), *name);
    }
}
