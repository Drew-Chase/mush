use std::io::Cursor;

use fold::cli::FoldConfig;
use fold::ops::fold;

fn parse(args: &[&str]) -> FoldConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    FoldConfig::from_args(&owned).expect("should not be --help/--version")
}

fn run(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    fold(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn default_width_80() {
    let short = "hello world";
    let result = run(&[], short);
    assert_eq!(result, "hello world\n");
}

#[test]
fn default_wraps_at_80() {
    let long = "a".repeat(100);
    let result = run(&[], &long);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].len(), 80);
    assert_eq!(lines[1].len(), 20);
}

#[test]
fn custom_width_40() {
    let input = "a".repeat(60);
    let result = run(&["-w", "40"], &input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].len(), 40);
    assert_eq!(lines[1].len(), 20);
}

#[test]
fn spaces_flag() {
    let input = "hello world this is a test of fold";
    let result = run(&["-w", "20", "-s"], input);
    // Should break at spaces
    for line in result.trim_end().split('\n') {
        assert!(line.len() <= 20, "line too long: '{line}'");
    }
}

#[test]
fn bytes_flag() {
    let input = "a".repeat(100);
    let result = run(&["-b", "-w", "50"], &input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].len(), 50);
}

#[test]
fn short_line_unchanged() {
    let result = run(&["-w", "40"], "short line");
    assert_eq!(result, "short line\n");
}

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(FoldConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(FoldConfig::from_args(&owned).is_none());
}

#[test]
fn long_width_option() {
    let input = "a".repeat(60);
    let result = run(&["--width=40"], &input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 2);
}
