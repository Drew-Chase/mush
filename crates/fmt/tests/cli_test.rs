use std::io::Cursor;

use clap::Parser;

use fmt::cli::FmtConfig;
use fmt::ops::fmt;

fn parse(args: &[&str]) -> FmtConfig {
    let mut full = vec!["fmt"];
    full.extend_from_slice(args);
    FmtConfig::parse_from(full)
}

fn run(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    fmt(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn default_reflow() {
    let input = "hello world\nthis is a test";
    let result = run(&[], input);
    assert_eq!(result, "hello world this is a test\n");
}

#[test]
fn width_40() {
    let input = "This is a longer sentence that should be reflowed to fit within forty characters when formatted.";
    let result = run(&["-w", "40"], input);
    for line in result.trim_end().split('\n') {
        assert!(line.len() <= 40, "line too long: '{line}' ({})", line.len());
    }
}

#[test]
fn split_only() {
    let input = "short\nThis is a very long line that exceeds the default width and should be split but not joined with adjacent lines";
    let result = run(&["-s", "-w", "40"], input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines[0], "short");
    for line in &lines[1..] {
        assert!(line.len() <= 40, "line too long: '{line}'");
    }
}

#[test]
fn uniform_spacing() {
    let input = "Hello.  This is a test.   More text here";
    let result = run(&["-u"], input);
    assert!(result.contains(".  "), "should have double space after period");
}

#[test]
fn prefix_filter() {
    let input = "> hello world\n> this is a test\nnot prefixed";
    let result = run(&["-p", ">", "-w", "40"], input);
    assert!(result.contains("> "));
    assert!(result.contains("not prefixed"));
}

#[test]
fn paragraph_separation() {
    let input = "para one line one\npara one line two\n\npara two line one";
    let result = run(&[], input);
    assert!(result.contains("\n\n"), "paragraphs should be separated by blank line");
}

#[test]
fn help_returns_err() {
    assert!(FmtConfig::try_parse_from(["fmt", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(FmtConfig::try_parse_from(["fmt", "--version"]).is_err());
}

#[test]
fn long_width_option() {
    let input = "word ".repeat(20);
    let result = run(&["--width=40"], &input);
    for line in result.trim_end().split('\n') {
        assert!(line.len() <= 40, "line too long: '{line}'");
    }
}
