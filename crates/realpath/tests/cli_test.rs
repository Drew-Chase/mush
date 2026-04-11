use realpath::cli::RealpathConfig;
use realpath::ops::resolve_path;

fn parse(args: &[&str]) -> RealpathConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    RealpathConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn resolve_existing_path() {
    let config = parse(&["."]);
    let result = resolve_path(".", &config);
    assert!(result.is_ok());
    let resolved = result.unwrap();
    assert!(resolved.is_absolute());
}

#[test]
fn canonicalize_existing_missing_file() {
    let config = RealpathConfig { canonicalize_existing: true, ..Default::default() };
    let result = resolve_path("/nonexistent_path_xyz_123", &config);
    assert!(result.is_err());
}

#[test]
fn canonicalize_missing_accepts_anything() {
    let config = RealpathConfig { canonicalize_missing: true, ..Default::default() };
    let result = resolve_path("/nonexistent_path_xyz_123/foo/bar", &config);
    assert!(result.is_ok());
    let resolved = result.unwrap();
    assert!(resolved.is_absolute());
}

#[test]
fn no_symlinks_flag() {
    let config = parse(&["-s", "."]);
    assert!(config.no_symlinks);
    let result = resolve_path(".", &config);
    assert!(result.is_ok());
}

#[test]
fn parse_flags_e() {
    let config = parse(&["-e", "somefile"]);
    assert!(config.canonicalize_existing);
    assert_eq!(config.files, vec!["somefile"]);
}

#[test]
fn parse_flags_m() {
    let config = parse(&["-m", "somefile"]);
    assert!(config.canonicalize_missing);
}

#[test]
fn parse_flags_combined() {
    let config = parse(&["-qz", "somefile"]);
    assert!(config.quiet);
    assert!(config.zero);
}

#[test]
fn parse_long_flags() {
    let config = parse(&["--quiet", "--zero", "--strip", "somefile"]);
    assert!(config.quiet);
    assert!(config.zero);
    assert!(config.no_symlinks);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(RealpathConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(RealpathConfig::from_args(&owned).is_none());
}
