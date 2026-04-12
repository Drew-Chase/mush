use clap::Parser;

use sleep::cli::SleepConfig;

fn parse(args: &[&str]) -> SleepConfig {
    let mut full = vec!["sleep"];
    full.extend_from_slice(args);
    SleepConfig::parse_from(full)
}

#[test]
fn bare_number() {
    let config = parse(&["5"]);
    assert_eq!(config.parse_duration(), 5.0);
}

#[test]
fn fractional_number() {
    let config = parse(&["1.5"]);
    assert_eq!(config.parse_duration(), 1.5);
}

#[test]
fn minutes_suffix() {
    let config = parse(&["1m"]);
    assert_eq!(config.parse_duration(), 60.0);
}

#[test]
fn hours_suffix() {
    let config = parse(&["1h"]);
    assert_eq!(config.parse_duration(), 3600.0);
}

#[test]
fn days_suffix() {
    let config = parse(&["1d"]);
    assert_eq!(config.parse_duration(), 86400.0);
}

#[test]
fn multiple_args_summed() {
    let config = parse(&["1m", "30s"]);
    assert_eq!(config.parse_duration(), 90.0);
}

#[test]
fn fractional_with_suffix() {
    let config = parse(&["0.5s"]);
    assert_eq!(config.parse_duration(), 0.5);
}

#[test]
fn help_returns_err() {
    assert!(SleepConfig::try_parse_from(["sleep", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(SleepConfig::try_parse_from(["sleep", "--version"]).is_err());
}
