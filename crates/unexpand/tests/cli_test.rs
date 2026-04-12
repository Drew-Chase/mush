use std::io::Cursor;

use clap::Parser;

use unexpand::cli::UnexpandConfig;
use unexpand::ops::unexpand;

fn parse(args: &[&str]) -> UnexpandConfig {
    let mut full = vec!["unexpand"];
    full.extend_from_slice(args);
    UnexpandConfig::parse_from(full)
}

fn run(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    unexpand(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn default_leading_spaces() {
    let result = run(&[], "        hello");
    assert_eq!(result, "\thello\n");
}

#[test]
fn default_no_inner_conversion() {
    let result = run(&[], "hello        world");
    assert_eq!(result, "hello        world\n");
}

#[test]
fn all_flag() {
    let result = run(&["-a"], "hello   world");
    assert_eq!(result, "hello\tworld\n");
}

#[test]
fn custom_tab_width() {
    let result = run(&["-t", "4"], "    hello");
    assert_eq!(result, "\thello\n");
}

#[test]
fn combined_all_and_tab_width() {
    let result = run(&["-a", "-t", "4"], "    hello   world");
    assert_eq!(result, "\thello\tworld\n");
}

#[test]
fn partial_spaces_not_converted() {
    let result = run(&[], "   hello");
    assert_eq!(result, "   hello\n");
}

#[test]
fn help_returns_err() {
    assert!(UnexpandConfig::try_parse_from(["unexpand", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(UnexpandConfig::try_parse_from(["unexpand", "--version"]).is_err());
}

#[test]
fn long_all_option() {
    let result = run(&["--all"], "hello   world");
    assert_eq!(result, "hello\tworld\n");
}
