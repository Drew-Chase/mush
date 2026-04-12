use std::io::Cursor;

use clap::Parser;

use expand::cli::ExpandConfig;
use expand::ops::expand;

fn parse(args: &[&str]) -> ExpandConfig {
    let mut full = vec!["expand"];
    full.extend_from_slice(args);
    ExpandConfig::parse_from(full)
}

fn run(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    expand(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn default_tab_width() {
    let result = run(&[], "\thello");
    assert_eq!(result, "        hello\n");
}

#[test]
fn tab_width_4() {
    let result = run(&["-t", "4"], "\thello");
    assert_eq!(result, "    hello\n");
}

#[test]
fn initial_only() {
    let result = run(&["-i"], "\thello\tworld");
    assert_eq!(result, "        hello\tworld\n");
}

#[test]
fn combined_initial_and_tab_width() {
    let result = run(&["-i", "-t", "4"], "\thello\tworld");
    assert_eq!(result, "    hello\tworld\n");
}

#[test]
fn multiple_tabs() {
    let result = run(&[], "\t\thello");
    assert_eq!(result, "                hello\n");
}

#[test]
fn tab_at_position() {
    let result = run(&[], "ab\tcd");
    assert_eq!(result, "ab      cd\n");
}

#[test]
fn no_tabs() {
    let result = run(&[], "hello world");
    assert_eq!(result, "hello world\n");
}

#[test]
fn help_returns_err() {
    assert!(ExpandConfig::try_parse_from(["expand", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(ExpandConfig::try_parse_from(["expand", "--version"]).is_err());
}

#[test]
fn long_tabs_option() {
    let result = run(&["--tabs=4"], "\thello");
    assert_eq!(result, "    hello\n");
}
