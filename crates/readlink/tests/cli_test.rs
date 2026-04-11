use readlink::cli::ReadlinkConfig;
use readlink::ops::readlink;

fn parse(args: &[&str]) -> ReadlinkConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    ReadlinkConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn readlink_nonexistent() {
    let config = ReadlinkConfig::default();
    let result = readlink("/nonexistent_path_xyz_123", &config);
    assert!(result.is_err());
}

#[test]
fn canonicalize_existing_dir() {
    let config = ReadlinkConfig { canonicalize_existing: true, ..Default::default() };
    let result = readlink(".", &config);
    assert!(result.is_ok());
}

#[test]
fn canonicalize_missing_nonexistent() {
    let config = ReadlinkConfig { canonicalize_missing: true, ..Default::default() };
    let result = readlink("/nonexistent_xyz_123/foo/bar", &config);
    assert!(result.is_ok());
}

#[test]
fn canonicalize_f_current_dir() {
    let config = ReadlinkConfig { canonicalize: true, ..Default::default() };
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
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(ReadlinkConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(ReadlinkConfig::from_args(&owned).is_none());
}
