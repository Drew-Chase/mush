use clap::Parser;

use mkdir::cli::MkdirConfig;

fn parse(args: &[&str]) -> MkdirConfig {
    let mut full = vec!["mkdir"];
    full.extend_from_slice(args);
    MkdirConfig::parse_from(full)
}

#[test]
fn no_args() {
    let config = parse(&[]);
    assert!(config.directories.is_empty());
    assert!(!config.parents);
    assert!(!config.verbose);
    assert_eq!(config.mode, None);
}

#[test]
fn single_directory() {
    let config = parse(&["foo"]);
    assert_eq!(config.directories, vec!["foo"]);
}

#[test]
fn multiple_directories() {
    let config = parse(&["foo", "bar", "baz"]);
    assert_eq!(config.directories, vec!["foo", "bar", "baz"]);
}

#[test]
fn flag_p() {
    let config = parse(&["-p", "foo"]);
    assert!(config.parents);
    assert_eq!(config.directories, vec!["foo"]);
}

#[test]
fn flag_v() {
    let config = parse(&["-v", "foo"]);
    assert!(config.verbose);
}

#[test]
fn combined_pv() {
    let config = parse(&["-p", "-v", "foo"]);
    assert!(config.parents);
    assert!(config.verbose);
}

#[test]
fn flag_m_separate() {
    let config = parse(&["-m", "755", "foo"]);
    assert_eq!(config.mode, Some(0o755));
    assert_eq!(config.directories, vec!["foo"]);
}

#[test]
fn flag_m_attached() {
    let config = parse(&["-m755", "foo"]);
    assert_eq!(config.mode, Some(0o755));
    assert_eq!(config.directories, vec!["foo"]);
}

#[test]
fn long_mode_equals() {
    let config = parse(&["--mode=755", "foo"]);
    assert_eq!(config.mode, Some(0o755));
}

#[test]
fn long_mode_separate() {
    let config = parse(&["--mode", "755", "foo"]);
    assert_eq!(config.mode, Some(0o755));
}

#[test]
fn long_parents() {
    let config = parse(&["--parents", "foo"]);
    assert!(config.parents);
}

#[test]
fn long_verbose() {
    let config = parse(&["--verbose", "foo"]);
    assert!(config.verbose);
}

#[test]
fn combined_pm() {
    let config = parse(&["-p", "-m", "755", "foo"]);
    assert!(config.parents);
    assert_eq!(config.mode, Some(0o755));
    assert_eq!(config.directories, vec!["foo"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-p"]);
    assert!(!config.parents);
    assert_eq!(config.directories, vec!["-p"]);
}

#[test]
fn mode_700() {
    let config = parse(&["-m", "700", "secret"]);
    assert_eq!(config.mode, Some(0o700));
}

#[test]
fn flags_before_and_dirs_after() {
    let config = parse(&["-p", "-v", "a", "b", "c"]);
    assert!(config.parents);
    assert!(config.verbose);
    assert_eq!(config.directories, vec!["a", "b", "c"]);
}

#[test]
fn help_is_err() {
    assert!(MkdirConfig::try_parse_from(["mkdir", "--help"]).is_err());
}

#[test]
fn version_is_err() {
    assert!(MkdirConfig::try_parse_from(["mkdir", "--version"]).is_err());
}
