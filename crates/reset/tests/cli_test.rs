use reset::cli::ResetConfig;
use reset::ops::{RESET_SEQUENCE, reset_terminal};

#[test]
fn default_config() {
    let config = ResetConfig::from_args(&[]);
    assert!(config.is_some());
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(ResetConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(ResetConfig::from_args(&owned).is_none());
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
