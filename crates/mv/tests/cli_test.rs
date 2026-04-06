use mv::cli::{MvConfig, OverwriteMode};

fn parse(args: &[&str]) -> MvConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    MvConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn no_args() {
    let config = parse(&[]);
    assert!(config.paths.is_empty());
    assert_eq!(config.overwrite, OverwriteMode::Force);
    assert!(!config.update);
    assert!(!config.verbose);
    assert!(!config.strip_trailing_slashes);
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
    assert_eq!(config.overwrite, OverwriteMode::Force);
}

#[test]
fn flag_i() {
    let config = parse(&["-i", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Interactive);
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::NoClobber);
}

#[test]
fn long_force() {
    let config = parse(&["--force", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Force);
}

#[test]
fn long_interactive() {
    let config = parse(&["--interactive", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Interactive);
}

#[test]
fn long_no_clobber() {
    let config = parse(&["--no-clobber", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::NoClobber);
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
fn flag_t_attached() {
    let config = parse(&["-t/tmp/dir", "a"]);
    assert_eq!(config.target_directory, Some("/tmp/dir".to_string()));
}

#[test]
fn long_target_directory_equals() {
    let config = parse(&["--target-directory=/tmp/dir", "a"]);
    assert_eq!(config.target_directory, Some("/tmp/dir".to_string()));
}

#[test]
fn long_target_directory_separate() {
    let config = parse(&["--target-directory", "/tmp/dir", "a"]);
    assert_eq!(config.target_directory, Some("/tmp/dir".to_string()));
}

#[test]
fn strip_trailing_slashes_flag() {
    let config = parse(&["--strip-trailing-slashes", "a/", "b"]);
    assert!(config.strip_trailing_slashes);
}

#[test]
fn combined_fiv_last_wins() {
    let config = parse(&["-fiv", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Interactive);
    assert!(config.verbose);
}

#[test]
fn combined_ifn_last_wins() {
    let config = parse(&["-ifn", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::NoClobber);
}

#[test]
fn combined_nif_last_wins() {
    let config = parse(&["-nif", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Force);
}

#[test]
fn combined_nfi_last_wins() {
    let config = parse(&["-nfi", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Interactive);
}

#[test]
fn combined_uv() {
    let config = parse(&["-uv", "a", "b"]);
    assert!(config.update);
    assert!(config.verbose);
}

#[test]
fn combined_vt_separate() {
    let config = parse(&["-vt", "/tmp", "a"]);
    assert!(config.verbose);
    assert_eq!(config.target_directory, Some("/tmp".to_string()));
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-f"]);
    assert_eq!(config.overwrite, OverwriteMode::Force); // default
    assert_eq!(config.paths, vec!["-f"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(MvConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(MvConfig::from_args(&owned).is_none());
}

#[test]
fn force_then_interactive_overrides() {
    let config = parse(&["--force", "--interactive", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Interactive);
}

#[test]
fn interactive_then_force_overrides() {
    let config = parse(&["-i", "-f", "a", "b"]);
    assert_eq!(config.overwrite, OverwriteMode::Force);
}

#[test]
fn flags_before_paths() {
    let config = parse(&["-fvu", "a", "b", "c"]);
    assert_eq!(config.overwrite, OverwriteMode::Force);
    assert!(config.verbose);
    assert!(config.update);
    assert_eq!(config.paths, vec!["a", "b", "c"]);
}
