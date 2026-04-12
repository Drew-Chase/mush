use clap::Parser;

use touch::cli::TouchConfig;

fn parse(args: &[&str]) -> TouchConfig {
    let mut full = vec!["touch"];
    full.extend_from_slice(args);
    TouchConfig::parse_from(full)
}

#[test]
fn defaults_with_file() {
    let config = parse(&["file.txt"]);
    assert!(!config.access_only);
    assert!(!config.modify_only);
    assert!(!config.no_create);
    assert!(config.reference.is_none());
    assert!(config.date.is_none());
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn flag_a() {
    let config = parse(&["-a", "file.txt"]);
    assert!(config.access_only);
    assert!(!config.modify_only);
}

#[test]
fn flag_m() {
    let config = parse(&["-m", "file.txt"]);
    assert!(config.modify_only);
    assert!(!config.access_only);
}

#[test]
fn flag_c() {
    let config = parse(&["-c", "file.txt"]);
    assert!(config.no_create);
}

#[test]
fn long_no_create() {
    let config = parse(&["--no-create", "file.txt"]);
    assert!(config.no_create);
}

#[test]
fn flag_r_separate() {
    let config = parse(&["-r", "ref.txt", "file.txt"]);
    assert_eq!(config.reference.as_deref(), Some("ref.txt"));
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn long_reference_equals() {
    let config = parse(&["--reference=ref.txt", "file.txt"]);
    assert_eq!(config.reference.as_deref(), Some("ref.txt"));
}

#[test]
fn long_reference_separate() {
    let config = parse(&["--reference", "ref.txt", "file.txt"]);
    assert_eq!(config.reference.as_deref(), Some("ref.txt"));
}

#[test]
fn flag_d_separate() {
    let config = parse(&["-d", "2024-01-01 00:00:00", "file.txt"]);
    assert_eq!(config.date.as_deref(), Some("2024-01-01 00:00:00"));
}

#[test]
fn long_date_equals() {
    let config = parse(&["--date=2024-01-01", "file.txt"]);
    assert_eq!(config.date.as_deref(), Some("2024-01-01"));
}

#[test]
fn long_date_separate() {
    let config = parse(&["--date", "2024-01-01", "file.txt"]);
    assert_eq!(config.date.as_deref(), Some("2024-01-01"));
}

#[test]
fn combined_am() {
    let config = parse(&["-am", "file.txt"]);
    assert!(config.access_only);
    assert!(config.modify_only);
}

#[test]
fn combined_acm() {
    let config = parse(&["-acm", "file.txt"]);
    assert!(config.access_only);
    assert!(config.no_create);
    assert!(config.modify_only);
}

#[test]
fn multiple_files() {
    let config = parse(&["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
}
