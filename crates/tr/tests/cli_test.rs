use std::io::Cursor;

use tr::cli::TrConfig;
use tr::ops::{expand_set, translate};

fn parse(args: &[&str]) -> TrConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    TrConfig::from_args(&owned).expect("should not be --help/--version")
}

fn run_tr(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    translate(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

// --- expand_set tests ---

#[test]
fn expand_range() {
    let chars = expand_set("a-f");
    assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn expand_upper_class() {
    let chars = expand_set("[:upper:]");
    assert_eq!(chars.len(), 26);
    assert_eq!(chars[0], 'A');
    assert_eq!(chars[25], 'Z');
}

#[test]
fn expand_lower_class() {
    let chars = expand_set("[:lower:]");
    assert_eq!(chars.len(), 26);
    assert_eq!(chars[0], 'a');
    assert_eq!(chars[25], 'z');
}

#[test]
fn expand_digit_class() {
    let chars = expand_set("[:digit:]");
    assert_eq!(chars.len(), 10);
    assert_eq!(chars[0], '0');
    assert_eq!(chars[9], '9');
}

#[test]
fn expand_alpha_class() {
    let chars = expand_set("[:alpha:]");
    assert_eq!(chars.len(), 52);
}

#[test]
fn expand_alnum_class() {
    let chars = expand_set("[:alnum:]");
    assert_eq!(chars.len(), 62);
}

#[test]
fn expand_escape_sequences() {
    let chars = expand_set(r"\n\t\\");
    assert_eq!(chars, vec!['\n', '\t', '\\']);
}

// --- basic translate tests ---

#[test]
fn basic_translate() {
    let result = run_tr(&["abc", "xyz"], "aabbcc");
    assert_eq!(result, "xxyyzz");
}

#[test]
fn translate_uppercase() {
    let result = run_tr(&["[:lower:]", "[:upper:]"], "hello");
    assert_eq!(result, "HELLO");
}

#[test]
fn translate_lowercase() {
    let result = run_tr(&["[:upper:]", "[:lower:]"], "HELLO");
    assert_eq!(result, "hello");
}

// --- delete tests ---

#[test]
fn delete_chars() {
    let result = run_tr(&["-d", "aeiou"], "hello world");
    assert_eq!(result, "hll wrld");
}

#[test]
fn delete_digits() {
    let result = run_tr(&["-d", "[:digit:]"], "abc123def456");
    assert_eq!(result, "abcdef");
}

// --- squeeze tests ---

#[test]
fn squeeze_repeats() {
    let result = run_tr(&["-s", "a"], "aaabbbccc");
    assert_eq!(result, "abbbccc");
}

#[test]
fn squeeze_spaces() {
    let result = run_tr(&["-s", " "], "hello   world");
    assert_eq!(result, "hello world");
}

// --- complement tests ---

#[test]
fn complement_delete() {
    let result = run_tr(&["-cd", "[:alpha:]"], "hello123world456");
    assert_eq!(result, "helloworld");
}

#[test]
fn complement_translate() {
    let result = run_tr(&["-c", "[:alpha:]", "_"], "hello 123 world");
    assert_eq!(result, "hello_____world");
}

// --- combined flags ---

#[test]
fn translate_and_squeeze() {
    let result = run_tr(&["-s", "ab", "xx"], "aabbcc");
    assert_eq!(result, "xcc");
}

#[test]
fn truncate_set1() {
    let result = run_tr(&["-t", "abcd", "xy"], "abcd");
    assert_eq!(result, "xycd");
}

// --- CLI parsing tests ---

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(TrConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(TrConfig::from_args(&owned).is_none());
}

#[test]
fn parse_combined_flags() {
    let config = parse(&["-ds", "abc"]);
    assert!(config.delete);
    assert!(config.squeeze);
    assert_eq!(config.set1, "abc");
}

#[test]
fn parse_long_flags() {
    let config = parse(&["--complement", "--delete", "abc"]);
    assert!(config.complement);
    assert!(config.delete);
    assert_eq!(config.set1, "abc");
}
