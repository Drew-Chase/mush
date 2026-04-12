use clap::Parser;

use reset::cli::ResetConfig;
use reset::ops::{RESET_SEQUENCE, reset_terminal};

#[test]
fn default_config() {
    let _config = ResetConfig::parse_from(["reset"]);
}

#[test]
fn help_returns_err() {
    let result = ResetConfig::try_parse_from(["reset", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = ResetConfig::try_parse_from(["reset", "--version"]);
    assert!(result.is_err());
}

#[test]
fn reset_sequence_content() {
    assert_eq!(RESET_SEQUENCE, b"\x1bc\x1b[0m\x1b[2J\x1b[H");
}

#[test]
fn reset_writes_sequence() {
    let mut output = Vec::new();
    reset_terminal(&mut output).unwrap();
    assert_eq!(output, RESET_SEQUENCE);
}
