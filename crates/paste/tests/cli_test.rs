use std::io::Cursor;

use clap::Parser;

use paste_cmd::cli::PasteConfig;
use paste_cmd::ops;

fn parse(args: &[&str]) -> PasteConfig {
    let mut full = vec!["paste"];
    full.extend_from_slice(args);
    PasteConfig::parse_from(full)
}

fn run_paste(inputs: &[&str], args: &[&str]) -> String {
    let config = parse(args);
    let mut boxed: Vec<Box<dyn std::io::Read>> = inputs
        .iter()
        .map(|s| Box::new(Cursor::new(s.as_bytes().to_vec())) as Box<dyn std::io::Read>)
        .collect();
    let mut out = Vec::new();
    ops::paste(&mut boxed, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert_eq!(config.delimiters, "\t");
    assert!(!config.serial);
    assert!(config.files.is_empty());
}

#[test]
fn default_tab_delimiter() {
    let result = run_paste(&["a\nb\nc\n", "1\n2\n3\n"], &[]);
    assert_eq!(result, "a\t1\nb\t2\nc\t3\n");
}

#[test]
fn comma_delimiter() {
    let result = run_paste(&["a\nb\n", "1\n2\n"], &["-d", ","]);
    assert_eq!(result, "a,1\nb,2\n");
}

#[test]
fn serial_mode() {
    let result = run_paste(&["a\nb\nc\n", "1\n2\n3\n"], &["-s"]);
    assert_eq!(result, "a\tb\tc\n1\t2\t3\n");
}

#[test]
fn serial_with_comma() {
    let result = run_paste(&["a\nb\nc\n"], &["-s", "-d", ","]);
    assert_eq!(result, "a,b,c\n");
}

#[test]
fn multiple_files_uneven() {
    let result = run_paste(&["a\nb\n", "1\n2\n3\n"], &[]);
    assert_eq!(result, "a\t1\nb\t2\n\t3\n");
}

#[test]
fn multiple_delimiters_cycle() {
    let result = run_paste(&["a\nb\n", "1\n2\n", "x\ny\n"], &["-d", ",:"]);
    assert_eq!(result, "a,1:x\nb,2:y\n");
}
