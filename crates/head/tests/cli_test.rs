use clap::Parser;

use head::cli::HeadConfig;

fn parse(args: &[&str]) -> HeadConfig {
    let mut full = vec!["head"];
    full.extend_from_slice(args);
    HeadConfig::parse_from(full)
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert_eq!(config.lines, 10);
    assert_eq!(config.bytes, None);
    assert!(!config.quiet);
    assert!(!config.verbose);
    assert!(config.files.is_empty());
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "5"]);
    assert_eq!(config.lines, 5);
    assert_eq!(config.bytes, None);
}

#[test]
fn flag_c() {
    let config = parse(&["-c", "100"]);
    assert_eq!(config.bytes, Some(100));
}

#[test]
fn long_lines() {
    let config = parse(&["--lines", "20"]);
    assert_eq!(config.lines, 20);
}

#[test]
fn long_lines_eq() {
    let config = parse(&["--lines=20"]);
    assert_eq!(config.lines, 20);
}

#[test]
fn long_bytes_eq() {
    let config = parse(&["--bytes=50"]);
    assert_eq!(config.bytes, Some(50));
}

#[test]
fn flag_q() {
    let config = parse(&["-q"]);
    assert!(config.quiet);
    assert!(!config.verbose);
}

#[test]
fn flag_quiet() {
    let config = parse(&["--quiet"]);
    assert!(config.quiet);
}

#[test]
fn flag_silent() {
    let config = parse(&["--silent"]);
    assert!(config.quiet);
}

#[test]
fn flag_v() {
    let config = parse(&["-v"]);
    assert!(config.verbose);
    assert!(!config.quiet);
}

#[test]
fn flag_verbose() {
    let config = parse(&["--verbose"]);
    assert!(config.verbose);
}

#[test]
fn files_collected() {
    let config = parse(&["-n", "5", "foo.txt", "bar.txt"]);
    assert_eq!(config.lines, 5);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn multiple_files() {
    let config = parse(&["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.lines, 10);
}
