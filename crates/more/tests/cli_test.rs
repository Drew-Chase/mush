use clap::Parser;

use more::cli::MoreConfig;

fn parse(args: &[&str]) -> MoreConfig {
    let mut full = vec!["more"];
    full.extend_from_slice(args);
    MoreConfig::parse_from(full)
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.squeeze);
    assert_eq!(config.lines_per_screen, None);
    assert_eq!(config.start_line, None);
    assert!(config.files.is_empty());
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.squeeze);
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "20"]);
    assert_eq!(config.lines_per_screen, Some(20));
}

#[test]
fn start_line() {
    let config = parse(&["--start-line", "50"]);
    assert_eq!(config.start_line, Some(50));
}

#[test]
fn files_collected() {
    let config = parse(&["-s", "foo.txt", "bar.txt"]);
    assert!(config.squeeze);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn combined_flags() {
    let config = parse(&["-s", "-n", "30", "--start-line", "10", "file.txt"]);
    assert!(config.squeeze);
    assert_eq!(config.lines_per_screen, Some(30));
    assert_eq!(config.start_line, Some(10));
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn help_returns_err() {
    assert!(MoreConfig::try_parse_from(["more", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(MoreConfig::try_parse_from(["more", "--version"]).is_err());
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-s"]);
    assert!(!config.squeeze);
    assert_eq!(config.files, vec!["-s"]);
}

#[test]
fn stdin_dash() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}
