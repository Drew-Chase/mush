use std::io::Cursor;

use fmt::cli::FmtConfig;
use fmt::ops::fmt;

fn parse(args: &[&str]) -> FmtConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    FmtConfig::from_args(&owned).expect("should not be --help/--version")
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
    // "short" should remain separate (not joined)
    assert_eq!(lines[0], "short");
    // Long lines should be split
    for line in &lines[1..] {
        assert!(line.len() <= 40, "line too long: '{line}'");
    }
}

#[test]
fn uniform_spacing() {
    let input = "Hello.  This is a test.   More text here";
    let result = run(&["-u"], input);
    // After sentence-ending punctuation, should have two spaces
    assert!(result.contains(".  "), "should have double space after period");
}

#[test]
fn prefix_filter() {
    let input = "> hello world\n> this is a test\nnot prefixed";
    let result = run(&["-p", ">", "-w", "40"], input);
    // Prefixed lines should be reformatted together
    assert!(result.contains("> "));
    // Non-prefixed line should remain as-is
    assert!(result.contains("not prefixed"));
}

#[test]
fn paragraph_separation() {
    let input = "para one line one\npara one line two\n\npara two line one";
    let result = run(&[], input);
    // Should have a blank line between paragraphs
    assert!(result.contains("\n\n"), "paragraphs should be separated by blank line");
}

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(FmtConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(FmtConfig::from_args(&owned).is_none());
}

#[test]
fn long_width_option() {
    let input = "word ".repeat(20);
    let result = run(&["--width=40"], &input);
    for line in result.trim_end().split('\n') {
        assert!(line.len() <= 40, "line too long: '{line}'");
    }
}
