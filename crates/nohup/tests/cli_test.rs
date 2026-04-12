use clap::Parser;

use nohup::cli::NohupConfig;

fn parse(args: &[&str]) -> NohupConfig {
    let mut full = vec!["nohup"];
    full.extend_from_slice(args);
    NohupConfig::parse_from(full)
}

#[test]
fn empty_args() {
    let config = parse(&[]);
    assert!(config.command.is_empty());
}

#[test]
fn single_command() {
    let config = parse(&["sleep"]);
    assert_eq!(config.command, vec!["sleep"]);
}

#[test]
fn command_with_args() {
    let config = parse(&["sleep", "10"]);
    assert_eq!(config.command, vec!["sleep", "10"]);
}

#[test]
fn complex_command() {
    let config = parse(&["bash", "-c", "echo hello"]);
    assert_eq!(config.command, vec!["bash", "-c", "echo hello"]);
}

#[test]
fn help_returns_err() {
    assert!(NohupConfig::try_parse_from(["nohup", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(NohupConfig::try_parse_from(["nohup", "--version"]).is_err());
}

#[test]
fn command_starting_with_dash() {
    let config = parse(&["my-program", "--flag"]);
    assert_eq!(config.command, vec!["my-program", "--flag"]);
}
