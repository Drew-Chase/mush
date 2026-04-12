use std::io::Cursor;

use unexpand::cli::UnexpandConfig;
use unexpand::ops::unexpand;

fn parse(args: &[&str]) -> UnexpandConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    UnexpandConfig::from_args(&owned).expect("should not be --help/--version")
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
    // 8 spaces at start -> one tab
    let result = run(&[], "        hello");
    assert_eq!(result, "\thello\n");
}

#[test]
fn default_no_inner_conversion() {
    // Without -a, inner spaces are not converted
    let result = run(&[], "hello        world");
    assert_eq!(result, "hello        world\n");
}

#[test]
fn all_flag() {
    // With -a, convert all space sequences at tab stops
    let result = run(&["-a"], "hello   world");
    assert_eq!(result, "hello\tworld\n");
}

#[test]
fn custom_tab_width() {
    // 4 spaces at start with -t 4 -> one tab
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
    // 3 spaces with default tab width 8: not enough for a tab stop
    let result = run(&[], "   hello");
    assert_eq!(result, "   hello\n");
}

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(UnexpandConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(UnexpandConfig::from_args(&owned).is_none());
}

#[test]
fn long_all_option() {
    let result = run(&["--all"], "hello   world");
    assert_eq!(result, "hello\tworld\n");
}
