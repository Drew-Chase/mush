use clap::Parser;

use rev::cli::RevConfig;
use rev::ops::{rev_line, rev_stream};

fn parse(args: &[&str]) -> RevConfig {
    let mut full = vec!["rev"];
    full.extend_from_slice(args);
    RevConfig::parse_from(full)
}

#[test]
fn reverse_simple() {
    assert_eq!(rev_line("hello"), "olleh");
}

#[test]
fn reverse_empty() {
    assert_eq!(rev_line(""), "");
}

#[test]
fn reverse_palindrome() {
    assert_eq!(rev_line("racecar"), "racecar");
}

#[test]
fn reverse_unicode() {
    assert_eq!(rev_line("abc\u{00e9}"), "\u{00e9}cba");
}

#[test]
fn stream_multiple_lines() {
    let input = "hello\nworld\n";
    let mut cursor = std::io::Cursor::new(input.as_bytes());
    let mut output = Vec::new();
    rev_stream(&mut cursor, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "olleh\ndlrow\n");
}

#[test]
fn stream_no_trailing_newline() {
    let input = "abc";
    let mut cursor = std::io::Cursor::new(input.as_bytes());
    let mut output = Vec::new();
    rev_stream(&mut cursor, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "cba\n");
}

#[test]
fn parse_no_args() {
    let config = parse(&[]);
    assert!(config.files.is_empty());
}

#[test]
fn parse_files() {
    let config = parse(&["a.txt", "b.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt"]);
}

#[test]
fn parse_stdin_dash() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn help_returns_err() {
    let result = RevConfig::try_parse_from(["rev", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = RevConfig::try_parse_from(["rev", "--version"]);
    assert!(result.is_err());
}
