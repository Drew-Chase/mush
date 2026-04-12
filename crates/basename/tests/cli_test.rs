use clap::Parser;

use basename::cli::BasenameConfig;
use basename::ops::basename;

fn parse(args: &[&str]) -> BasenameConfig {
    let mut full = vec!["basename"];
    full.extend_from_slice(args);
    let mut config = BasenameConfig::parse_from(full);
    config.fixup();
    config
}

#[test]
fn basic_path() {
    assert_eq!(basename("/usr/bin/sort", None), "sort");
}

#[test]
fn with_suffix() {
    assert_eq!(basename("file.txt", Some(".txt")), "file");
}

#[test]
fn suffix_not_matching() {
    assert_eq!(basename("file.txt", Some(".rs")), "file.txt");
}

#[test]
fn no_directory() {
    assert_eq!(basename("hello", None), "hello");
}

#[test]
fn root_path() {
    assert_eq!(basename("/", None), "/");
}

#[test]
fn trailing_slash() {
    assert_eq!(basename("/usr/bin/", None), "bin");
}

#[test]
fn parse_basic() {
    let config = parse(&["/usr/bin/sort"]);
    assert_eq!(config.names, vec!["/usr/bin/sort"]);
    assert!(!config.multiple);
    assert!(!config.zero);
    assert_eq!(config.suffix, None);
}

#[test]
fn parse_with_suffix_positional() {
    let config = parse(&["file.txt", ".txt"]);
    assert_eq!(config.names, vec!["file.txt"]);
    assert_eq!(config.suffix, Some(".txt".to_string()));
}

#[test]
fn parse_multiple_flag() {
    let config = parse(&["-a", "file1.txt", "file2.txt"]);
    assert!(config.multiple);
    assert_eq!(config.names, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn parse_suffix_flag_separate() {
    let config = parse(&["-s", ".txt", "file1.txt", "file2.txt"]);
    assert!(config.multiple);
    assert_eq!(config.suffix, Some(".txt".to_string()));
    assert_eq!(config.names, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn parse_suffix_flag_attached() {
    let config = parse(&["-s.txt", "file1.txt"]);
    assert!(config.multiple);
    assert_eq!(config.suffix, Some(".txt".to_string()));
    assert_eq!(config.names, vec!["file1.txt"]);
}

#[test]
fn parse_suffix_long_equals() {
    let config = parse(&["--suffix=.txt", "file1.txt"]);
    assert!(config.multiple);
    assert_eq!(config.suffix, Some(".txt".to_string()));
}

#[test]
fn parse_suffix_long_separate() {
    let config = parse(&["--suffix", ".txt", "file1.txt"]);
    assert!(config.multiple);
    assert_eq!(config.suffix, Some(".txt".to_string()));
}

#[test]
fn parse_zero_flag() {
    let config = parse(&["-z", "file.txt"]);
    assert!(config.zero);
}

#[test]
fn parse_combined_az() {
    let config = parse(&["-az", "file1.txt", "file2.txt"]);
    assert!(config.multiple);
    assert!(config.zero);
    assert_eq!(config.names, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn help_returns_err() {
    let result = BasenameConfig::try_parse_from(["basename", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = BasenameConfig::try_parse_from(["basename", "--version"]);
    assert!(result.is_err());
}
