use clap::Parser;

use arch::cli::ArchConfig;

fn parse(args: &[&str]) -> ArchConfig {
    let mut full = vec!["arch"];
    full.extend_from_slice(args);
    ArchConfig::parse_from(full)
}

#[test]
fn no_args() {
    let _config = parse(&[]);
}

#[test]
fn machine_arch_not_empty() {
    assert!(!arch::ops::machine_arch().is_empty());
}

#[test]
fn help_returns_err() {
    let result = ArchConfig::try_parse_from(["arch", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = ArchConfig::try_parse_from(["arch", "--version"]);
    assert!(result.is_err());
}
