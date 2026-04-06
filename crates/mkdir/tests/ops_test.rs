use tempfile::TempDir;

use mkdir::cli::MkdirConfig;
use mkdir::ops::create_directory;

#[test]
fn create_single_directory() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("new_dir");
    let config = MkdirConfig::default();
    create_directory(&dir, &config).unwrap();
    assert!(dir.is_dir());
}

#[test]
fn create_nested_with_parents() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("a/b/c");
    let config = MkdirConfig {
        parents: true,
        ..Default::default()
    };
    create_directory(&dir, &config).unwrap();
    assert!(dir.is_dir());
    assert!(tmp.path().join("a/b").is_dir());
    assert!(tmp.path().join("a").is_dir());
}

#[test]
fn create_without_parents_missing_parent_fails() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("x/y/z");
    let config = MkdirConfig::default();
    assert!(create_directory(&dir, &config).is_err());
}

#[test]
fn create_existing_without_parents_fails() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("existing");
    std::fs::create_dir(&dir).unwrap();
    let config = MkdirConfig::default();
    assert!(create_directory(&dir, &config).is_err());
}

#[test]
fn create_existing_with_parents_succeeds() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("existing");
    std::fs::create_dir(&dir).unwrap();
    let config = MkdirConfig {
        parents: true,
        ..Default::default()
    };
    assert!(create_directory(&dir, &config).is_ok());
}

#[test]
fn create_multiple_directories() {
    let tmp = TempDir::new().unwrap();
    let config = MkdirConfig::default();
    for name in &["dir_a", "dir_b", "dir_c"] {
        let dir = tmp.path().join(name);
        create_directory(&dir, &config).unwrap();
        assert!(dir.is_dir());
    }
}

#[test]
fn verbose_does_not_affect_creation() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("verbose_dir");
    let config = MkdirConfig {
        verbose: true,
        ..Default::default()
    };
    create_directory(&dir, &config).unwrap();
    assert!(dir.is_dir());
}

#[test]
fn nested_parents_creates_intermediate() {
    let tmp = TempDir::new().unwrap();
    let deep = tmp.path().join("one/two/three/four");
    let config = MkdirConfig {
        parents: true,
        ..Default::default()
    };
    create_directory(&deep, &config).unwrap();
    assert!(tmp.path().join("one").is_dir());
    assert!(tmp.path().join("one/two").is_dir());
    assert!(tmp.path().join("one/two/three").is_dir());
    assert!(deep.is_dir());
}
