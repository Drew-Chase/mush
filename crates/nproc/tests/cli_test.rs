use clap::Parser;

use nproc::cli::NprocConfig;
use nproc::ops::nproc;

fn parse(args: &[&str]) -> NprocConfig {
    let mut full = vec!["nproc"];
    full.extend_from_slice(args);
    NprocConfig::parse_from(full)
}

#[test]
fn default_returns_at_least_one() {
    let config = parse(&[]);
    assert!(nproc(&config) >= 1);
}

#[test]
fn all_flag() {
    let config = parse(&["--all"]);
    assert!(config.all);
    assert!(nproc(&config) >= 1);
}

#[test]
fn ignore_equals() {
    let config = parse(&["--ignore=2"]);
    assert_eq!(config.ignore, 2);
}

#[test]
fn ignore_separate() {
    let config = parse(&["--ignore", "3"]);
    assert_eq!(config.ignore, 3);
}

#[test]
fn ignore_never_below_one() {
    let config = NprocConfig::parse_from(["nproc", "--ignore", "99999"]);
    assert_eq!(nproc(&config), 1);
}

#[test]
fn help_returns_err() {
    let result = NprocConfig::try_parse_from(["nproc", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = NprocConfig::try_parse_from(["nproc", "--version"]);
    assert!(result.is_err());
}
