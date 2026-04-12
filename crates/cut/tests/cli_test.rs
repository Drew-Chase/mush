use clap::Parser;

use cut::cli::{CutConfig, CutMode, Range, parse_ranges};

fn parse(args: &[&str]) -> CutConfig {
    let mut full = vec!["cut"];
    full.extend_from_slice(args);
    let mut config = CutConfig::parse_from(full);
    config.resolve().expect("should resolve mode");
    config
}

// --- Range parsing ---

#[test]
fn parse_single() {
    let ranges = parse_ranges("3").unwrap();
    assert_eq!(ranges, vec![Range::Single(3)]);
}

#[test]
fn parse_from_to() {
    let ranges = parse_ranges("3-5").unwrap();
    assert_eq!(ranges, vec![Range::FromTo(3, 5)]);
}

#[test]
fn parse_from() {
    let ranges = parse_ranges("7-").unwrap();
    assert_eq!(ranges, vec![Range::From(7)]);
}

#[test]
fn parse_to() {
    let ranges = parse_ranges("-3").unwrap();
    assert_eq!(ranges, vec![Range::To(3)]);
}

#[test]
fn parse_complex() {
    let ranges = parse_ranges("1,3-5,7-").unwrap();
    assert_eq!(
        ranges,
        vec![Range::Single(1), Range::FromTo(3, 5), Range::From(7)]
    );
}

// --- CLI flag parsing ---

#[test]
fn fields_flag() {
    let config = parse(&["-f", "1,3"]);
    assert!(matches!(config.mode, Some(CutMode::Fields(_))));
    assert_eq!(config.delimiter_char(), '\t');
}

#[test]
fn fields_attached() {
    let config = parse(&["-f1,3"]);
    assert!(matches!(config.mode, Some(CutMode::Fields(_))));
}

#[test]
fn delimiter_flag() {
    let config = parse(&["-f", "1", "-d", ":"]);
    assert_eq!(config.delimiter_char(), ':');
}

#[test]
fn bytes_flag() {
    let config = parse(&["-b", "1-3"]);
    assert!(matches!(config.mode, Some(CutMode::Bytes(_))));
}

#[test]
fn characters_flag() {
    let config = parse(&["-c", "2-4"]);
    assert!(matches!(config.mode, Some(CutMode::Characters(_))));
}

#[test]
fn only_delimited_flag() {
    let config = parse(&["-f", "1", "-s"]);
    assert!(config.only_delimited);
}

#[test]
fn complement_flag() {
    let config = parse(&["-f", "1", "--complement"]);
    assert!(config.complement);
}

#[test]
fn output_delimiter_flag() {
    let config = parse(&["-f", "1", "--output-delimiter=,"]);
    assert_eq!(config.output_delimiter, Some(",".to_string()));
}

#[test]
fn long_flags() {
    let config = parse(&["--fields=1,3-5", "--delimiter=:", "--only-delimited"]);
    assert!(matches!(config.mode, Some(CutMode::Fields(_))));
    assert_eq!(config.delimiter_char(), ':');
    assert!(config.only_delimited);
}

#[test]
fn files_collected() {
    let config = parse(&["-f", "1", "foo.txt", "bar.txt"]);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

// --- ops integration tests ---

use std::io::Cursor;
use cut::ops;

fn run_cut(input: &str, args: &[&str]) -> String {
    let config = parse(args);
    let mut inp = Cursor::new(input.as_bytes().to_vec());
    let mut out = Vec::new();
    ops::cut(&mut inp, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn cut_fields_tab() {
    let result = run_cut("a\tb\tc\n", &["-f", "2"]);
    assert_eq!(result, "b\n");
}

#[test]
fn cut_fields_custom_delim() {
    let result = run_cut("a:b:c\n", &["-f", "1,3", "-d", ":"]);
    assert_eq!(result, "a:c\n");
}

#[test]
fn cut_fields_output_delim() {
    let result = run_cut("a:b:c\n", &["-f", "1,3", "-d", ":", "--output-delimiter=|"]);
    assert_eq!(result, "a|c\n");
}

#[test]
fn cut_bytes() {
    let result = run_cut("abcdef\n", &["-b", "2-4"]);
    assert_eq!(result, "bcd\n");
}

#[test]
fn cut_characters() {
    let result = run_cut("abcdef\n", &["-c", "1,3,5"]);
    assert_eq!(result, "ace\n");
}

#[test]
fn cut_only_delimited() {
    let result = run_cut("no-delim\na\tb\n", &["-f", "1", "-s"]);
    assert_eq!(result, "a\n");
}

#[test]
fn cut_complement() {
    let result = run_cut("a:b:c:d\n", &["-f", "2", "-d", ":", "--complement"]);
    assert_eq!(result, "a:c:d\n");
}

#[test]
fn cut_range_from() {
    let result = run_cut("a:b:c:d\n", &["-f", "3-", "-d", ":"]);
    assert_eq!(result, "c:d\n");
}

#[test]
fn cut_range_to() {
    let result = run_cut("a:b:c:d\n", &["-f", "-2", "-d", ":"]);
    assert_eq!(result, "a:b\n");
}
