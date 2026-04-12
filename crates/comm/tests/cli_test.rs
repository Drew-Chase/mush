use std::io::Cursor;

use clap::Parser;

use comm::cli::CommConfig;
use comm::ops;

fn parse(args: &[&str]) -> CommConfig {
    let mut full = vec!["comm"];
    full.extend_from_slice(args);
    CommConfig::parse_from(full)
}

fn run_comm(input1: &str, input2: &str, args: &[&str]) -> String {
    // Build args with file placeholders at end
    let mut full_args: Vec<&str> = args.to_vec();
    full_args.push("file1");
    full_args.push("file2");
    let config = parse(&full_args);

    let mut inp1 = Cursor::new(input1.as_bytes().to_vec());
    let mut inp2 = Cursor::new(input2.as_bytes().to_vec());
    let mut out = Vec::new();
    ops::comm(&mut inp1, &mut inp2, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn default_three_columns() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &[]);
    assert_eq!(result, "a\n\t\tb\n\tc\n\t\td\n");
}

#[test]
fn suppress_col1() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &["-1"]);
    assert_eq!(result, "\tb\nc\n\td\n");
}

#[test]
fn suppress_col2() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &["-2"]);
    assert_eq!(result, "a\n\tb\n\td\n");
}

#[test]
fn suppress_col3() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &["-3"]);
    assert_eq!(result, "a\n\tc\n");
}

#[test]
fn suppress_col12() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &["-1", "-2"]);
    assert_eq!(result, "b\nd\n");
}

#[test]
fn suppress_col23() {
    let result = run_comm("a\nb\nd\n", "b\nc\nd\n", &["-2", "-3"]);
    assert_eq!(result, "a\n");
}

#[test]
fn ignore_case() {
    let result = run_comm("A\nB\n", "a\nb\nc\n", &["-i"]);
    assert_eq!(result, "\t\tA\n\t\tB\n\tc\n");
}
