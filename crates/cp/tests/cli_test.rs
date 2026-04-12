use clap::Parser;

use cp::cli::{CpConfig, OverwriteMode};

fn parse(args: &[&str]) -> CpConfig {
    let mut full = vec!["cp"];
    full.extend_from_slice(args);
    CpConfig::parse_from(full)
}

#[test]
fn no_args() {
    let config = parse(&[]);
    assert!(config.paths.is_empty());
    assert_eq!(config.overwrite(), OverwriteMode::Force);
    assert!(!config.recursive);
    assert!(!config.update);
    assert!(!config.verbose);
    assert!(config.target_directory.is_none());
    assert!(!config.no_target_directory);
}

#[test]
fn single_path() {
    let config = parse(&["file.txt"]);
    assert_eq!(config.paths, vec!["file.txt"]);
}

#[test]
fn multiple_paths() {
    let config = parse(&["a", "b", "c"]);
    assert_eq!(config.paths, vec!["a", "b", "c"]);
}

#[test]
fn flag_f() {
    let config = parse(&["-f", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::Force);
}

#[test]
fn flag_i() {
    let config = parse(&["-i", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::Interactive);
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::NoClobber);
}

#[test]
fn flag_r() {
    let config = parse(&["-r", "dir", "dest"]);
    assert!(config.recursive);
}

#[test]
fn flag_capital_r() {
    let config = parse(&["-R", "dir", "dest"]);
    assert!(config.recursive);
}

#[test]
fn long_recursive() {
    let config = parse(&["--recursive", "dir", "dest"]);
    assert!(config.recursive);
}

#[test]
fn long_force() {
    let config = parse(&["--force", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::Force);
}

#[test]
fn long_interactive() {
    let config = parse(&["--interactive", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::Interactive);
}

#[test]
fn long_no_clobber() {
    let config = parse(&["--no-clobber", "a", "b"]);
    assert_eq!(config.overwrite(), OverwriteMode::NoClobber);
}

#[test]
fn flag_u() {
    let config = parse(&["-u", "a", "b"]);
    assert!(config.update);
}

#[test]
fn long_update() {
    let config = parse(&["--update", "a", "b"]);
    assert!(config.update);
}

#[test]
fn flag_v() {
    let config = parse(&["-v", "a", "b"]);
    assert!(config.verbose);
}

#[test]
fn long_verbose() {
    let config = parse(&["--verbose", "a", "b"]);
    assert!(config.verbose);
}

#[test]
fn flag_big_t() {
    let config = parse(&["-T", "a", "b"]);
    assert!(config.no_target_directory);
}

#[test]
fn long_no_target_directory() {
    let config = parse(&["--no-target-directory", "a", "b"]);
    assert!(config.no_target_directory);
}

#[test]
fn flag_t_separate() {
    let config = parse(&["-t", "/tmp/dir", "a"]);
    assert_eq!(config.target_directory, Some("/tmp/dir".to_string()));
    assert_eq!(config.paths, vec!["a"]);
}

#[test]
fn long_target_directory_separate() {
    let config = parse(&["--target-directory", "/tmp/dir", "a"]);
    assert_eq!(config.target_directory, Some("/tmp/dir".to_string()));
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-r"]);
    assert!(!config.recursive);
    assert_eq!(config.paths, vec!["-r"]);
}

#[test]
fn help_returns_err() {
    assert!(CpConfig::try_parse_from(["cp", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(CpConfig::try_parse_from(["cp", "--version"]).is_err());
}

#[test]
fn flags_before_paths() {
    let config = parse(&["-r", "-f", "-v", "-u", "a", "b", "c"]);
    assert!(config.recursive);
    assert_eq!(config.overwrite(), OverwriteMode::Force);
    assert!(config.verbose);
    assert!(config.update);
    assert_eq!(config.paths, vec!["a", "b", "c"]);
}
