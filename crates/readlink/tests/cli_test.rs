use clap::Parser;

use readlink::cli::ReadlinkConfig;
use readlink::ops::readlink;

fn parse(args: &[&str]) -> ReadlinkConfig {
    let mut full = vec!["readlink"];
    full.extend_from_slice(args);
    ReadlinkConfig::parse_from(full)
}

#[test]
fn readlink_nonexistent() {
    let config = ReadlinkConfig::parse_from(["readlink", "/nonexistent_path_xyz_123"]);
    let result = readlink("/nonexistent_path_xyz_123", &config);
    assert!(result.is_err());
}

#[test]
fn canonicalize_existing_dir() {
    let config = ReadlinkConfig::parse_from(["readlink", "-e", "."]);
    let result = readlink(".", &config);
    assert!(result.is_ok());
}

#[test]
fn canonicalize_missing_nonexistent() {
    let config = ReadlinkConfig::parse_from(["readlink", "-m", "/nonexistent_xyz_123/foo/bar"]);
    let result = readlink("/nonexistent_xyz_123/foo/bar", &config);
    assert!(result.is_ok());
}

#[test]
fn canonicalize_f_current_dir() {
    let config = ReadlinkConfig::parse_from(["readlink", "-f", "."]);
    let result = readlink(".", &config);
    assert!(result.is_ok());
}

#[test]
fn parse_short_f() {
    let config = parse(&["-f", "somefile"]);
    assert!(config.canonicalize);
    assert_eq!(config.files, vec!["somefile"]);
}

#[test]
fn parse_short_e() {
    let config = parse(&["-e", "somefile"]);
    assert!(config.canonicalize_existing);
}

#[test]
fn parse_short_m() {
    let config = parse(&["-m", "somefile"]);
    assert!(config.canonicalize_missing);
}

#[test]
fn parse_short_n() {
    let config = parse(&["-n", "somefile"]);
    assert!(config.no_newline);
}

#[test]
fn parse_short_z() {
    let config = parse(&["-z", "somefile"]);
    assert!(config.zero);
}

#[test]
fn parse_combined_nz() {
    let config = parse(&["-nz", "somefile"]);
    assert!(config.no_newline);
    assert!(config.zero);
}

#[test]
fn parse_long_flags() {
    let config = parse(&["--canonicalize", "--no-newline", "--zero", "somefile"]);
    assert!(config.canonicalize);
    assert!(config.no_newline);
    assert!(config.zero);
}

#[test]
fn help_returns_err() {
    let result = ReadlinkConfig::try_parse_from(["readlink", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = ReadlinkConfig::try_parse_from(["readlink", "--version"]);
    assert!(result.is_err());
}
