use clap::Parser;

use dirname::cli::DirnameConfig;
use dirname::ops::dirname;

fn parse(args: &[&str]) -> DirnameConfig {
    let mut full = vec!["dirname"];
    full.extend_from_slice(args);
    DirnameConfig::parse_from(full)
}

#[test]
fn basic_path() {
    assert_eq!(dirname("/usr/bin/sort"), "/usr/bin");
}

#[test]
fn file_in_current_dir() {
    assert_eq!(dirname("hello"), ".");
}

#[test]
fn root_path() {
    assert_eq!(dirname("/"), "/");
}

#[test]
fn trailing_slash() {
    assert_eq!(dirname("/usr/bin/"), "/usr");
}

#[test]
fn nested_path() {
    assert_eq!(dirname("/a/b/c/d"), "/a/b/c");
}

#[test]
fn empty_string() {
    assert_eq!(dirname(""), ".");
}

#[test]
fn single_component() {
    assert_eq!(dirname("filename.txt"), ".");
}

#[test]
fn parse_basic() {
    let config = parse(&["/usr/bin/sort"]);
    assert_eq!(config.names, vec!["/usr/bin/sort"]);
    assert!(!config.zero);
}

#[test]
fn parse_zero_flag() {
    let config = parse(&["-z", "/usr/bin/sort"]);
    assert!(config.zero);
    assert_eq!(config.names, vec!["/usr/bin/sort"]);
}

#[test]
fn parse_long_zero() {
    let config = parse(&["--zero", "/usr/bin/sort"]);
    assert!(config.zero);
}

#[test]
fn parse_multiple_names() {
    let config = parse(&["/usr/bin/sort", "/usr/lib/foo"]);
    assert_eq!(config.names, vec!["/usr/bin/sort", "/usr/lib/foo"]);
}

#[test]
fn help_returns_err() {
    let result = DirnameConfig::try_parse_from(["dirname", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = DirnameConfig::try_parse_from(["dirname", "--version"]);
    assert!(result.is_err());
}
