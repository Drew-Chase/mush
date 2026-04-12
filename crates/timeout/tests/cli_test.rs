use clap::Parser;

use timeout::cli::{TimeoutConfig, parse_duration};

fn parse(args: &[&str]) -> TimeoutConfig {
    let mut full = vec!["timeout"];
    full.extend_from_slice(args);
    let mut config = TimeoutConfig::parse_from(full);
    config.resolve().expect("should not error");
    config
}

// --- duration parsing tests ---

#[test]
fn parse_bare_number() {
    assert_eq!(parse_duration("5"), Some(5.0));
}

#[test]
fn parse_seconds_suffix() {
    assert_eq!(parse_duration("0.5s"), Some(0.5));
}

#[test]
fn parse_minutes_suffix() {
    assert_eq!(parse_duration("1m"), Some(60.0));
}

#[test]
fn parse_hours_suffix() {
    assert_eq!(parse_duration("2h"), Some(7200.0));
}

#[test]
fn parse_days_suffix() {
    assert_eq!(parse_duration("1d"), Some(86400.0));
}

#[test]
fn parse_fractional_minutes() {
    assert_eq!(parse_duration("0.5m"), Some(30.0));
}

#[test]
fn parse_invalid_duration() {
    assert_eq!(parse_duration("abc"), None);
}

// --- CLI parsing tests ---

#[test]
fn basic_command() {
    let config = parse(&["5", "sleep", "10"]);
    assert_eq!(config.duration_secs, 5.0);
    assert_eq!(config.signal, "TERM");
    assert_eq!(config.command, vec!["sleep", "10"]);
    assert!(!config.preserve_status);
    assert!(!config.verbose);
    assert!(config.kill_after.is_none());
}

#[test]
fn signal_short_flag() {
    let config = parse(&["-s", "HUP", "5", "sleep", "10"]);
    assert_eq!(config.signal, "HUP");
    assert_eq!(config.duration_secs, 5.0);
}

#[test]
fn signal_long_flag_equals() {
    let config = parse(&["--signal=INT", "5", "sleep", "10"]);
    assert_eq!(config.signal, "INT");
}

#[test]
fn signal_long_flag_separate() {
    let config = parse(&["--signal", "QUIT", "5", "sleep", "10"]);
    assert_eq!(config.signal, "QUIT");
}

#[test]
fn kill_after_short() {
    let config = parse(&["-k", "10s", "5", "sleep", "10"]);
    assert_eq!(config.kill_after, Some(10.0));
}

#[test]
fn kill_after_long_equals() {
    let config = parse(&["--kill-after=1m", "5", "sleep", "10"]);
    assert_eq!(config.kill_after, Some(60.0));
}

#[test]
fn kill_after_long_separate() {
    let config = parse(&["--kill-after", "30", "5", "sleep", "10"]);
    assert_eq!(config.kill_after, Some(30.0));
}

#[test]
fn preserve_status_flag() {
    let config = parse(&["--preserve-status", "5", "sleep", "10"]);
    assert!(config.preserve_status);
}

#[test]
fn verbose_short_flag() {
    let config = parse(&["-v", "5", "sleep", "10"]);
    assert!(config.verbose);
}

#[test]
fn verbose_long_flag() {
    let config = parse(&["--verbose", "5", "sleep", "10"]);
    assert!(config.verbose);
}

#[test]
fn command_with_args() {
    let config = parse(&["10", "echo", "hello", "world"]);
    assert_eq!(config.command, vec!["echo", "hello", "world"]);
}

#[test]
fn duration_with_suffix() {
    let config = parse(&["1m", "sleep", "120"]);
    assert_eq!(config.duration_secs, 60.0);
}

#[test]
fn missing_command() {
    let mut config = TimeoutConfig::parse_from(vec!["timeout", "5"]);
    let result = config.resolve();
    assert!(result.is_err());
}

#[test]
fn combined_flags() {
    let config = parse(&["-v", "-s", "HUP", "-k", "5", "--preserve-status", "10", "sleep", "20"]);
    assert!(config.verbose);
    assert_eq!(config.signal, "HUP");
    assert_eq!(config.kill_after, Some(5.0));
    assert!(config.preserve_status);
    assert_eq!(config.duration_secs, 10.0);
    assert_eq!(config.command, vec!["sleep", "20"]);
}
