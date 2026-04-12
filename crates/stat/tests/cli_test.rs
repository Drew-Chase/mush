use clap::Parser;

use stat::cli::StatConfig;

fn parse(args: &[&str]) -> StatConfig {
    let mut full = vec!["stat"];
    full.extend_from_slice(args);
    StatConfig::parse_from(full)
}

#[test]
fn basic_file() {
    let config = parse(&["file.txt"]);
    assert_eq!(config.files, vec!["file.txt"]);
    assert!(!config.dereference);
    assert!(!config.terse);
    assert!(config.format.is_none());
}

#[test]
fn dereference_short() {
    let config = parse(&["-L", "link.txt"]);
    assert!(config.dereference);
}

#[test]
fn dereference_long() {
    let config = parse(&["--dereference", "link.txt"]);
    assert!(config.dereference);
}

#[test]
fn format_short() {
    let config = parse(&["-c", "%n %s", "file.txt"]);
    assert_eq!(config.format.as_deref(), Some("%n %s"));
}

#[test]
fn format_long_eq() {
    let config = parse(&["--format=%n", "file.txt"]);
    assert_eq!(config.format.as_deref(), Some("%n"));
}

#[test]
fn format_long_space() {
    let config = parse(&["--format", "%s", "file.txt"]);
    assert_eq!(config.format.as_deref(), Some("%s"));
}

#[test]
fn terse_short() {
    let config = parse(&["-t", "file.txt"]);
    assert!(config.terse);
}

#[test]
fn terse_long() {
    let config = parse(&["--terse", "file.txt"]);
    assert!(config.terse);
}

#[test]
fn multiple_files() {
    let config = parse(&["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
}

#[test]
fn combined_flags() {
    let config = parse(&["-Lt", "file.txt"]);
    assert!(config.dereference);
    assert!(config.terse);
}

#[test]
fn stat_default_output() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.txt");
    std::fs::write(&file, "hello world").unwrap();

    let config = StatConfig {
        files: vec![file.to_string_lossy().to_string()],
        ..Default::default()
    };
    let output = stat::ops::stat_file(&file, &config).unwrap();
    assert!(output.contains("File:"));
    assert!(output.contains("Size:"));
    assert!(output.contains("11")); // "hello world" is 11 bytes
}

#[test]
fn stat_format_string() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("fmt.txt");
    std::fs::write(&file, "abc").unwrap();

    let config = StatConfig {
        format: Some("%s".to_string()),
        files: vec![file.to_string_lossy().to_string()],
        ..Default::default()
    };
    let output = stat::ops::stat_file(&file, &config).unwrap();
    assert_eq!(output, "3");
}

#[test]
fn stat_terse_output() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("terse.txt");
    std::fs::write(&file, "data").unwrap();

    let config = StatConfig {
        terse: true,
        files: vec![file.to_string_lossy().to_string()],
        ..Default::default()
    };
    let output = stat::ops::stat_file(&file, &config).unwrap();
    // Terse output starts with filename
    assert!(output.contains("terse.txt"));
    // Contains size
    assert!(output.contains("4"));
}

#[test]
fn stat_format_file_type() {
    let dir = tempfile::tempdir().unwrap();
    let config = StatConfig {
        format: Some("%F".to_string()),
        files: vec![dir.path().to_string_lossy().to_string()],
        ..Default::default()
    };
    let output = stat::ops::stat_file(dir.path(), &config).unwrap();
    assert_eq!(output, "directory");
}

#[test]
fn stat_nonexistent_file() {
    let config = StatConfig {
        files: vec!["nonexistent_file_xyz".to_string()],
        ..Default::default()
    };
    let result = stat::ops::stat_file(std::path::Path::new("nonexistent_file_xyz"), &config);
    assert!(result.is_err());
}
