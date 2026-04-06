use rm::cli::{InteractiveMode, RmConfig};

fn parse(args: &[&str]) -> RmConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    RmConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn no_args() {
    let config = parse(&[]);
    assert!(config.paths.is_empty());
    assert!(!config.force);
    assert!(!config.recursive);
    assert!(!config.verbose);
    assert!(!config.dir);
    assert!(config.preserve_root);
    assert_eq!(config.interactive, InteractiveMode::Never);
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
    let config = parse(&["-f", "file"]);
    assert!(config.force);
    assert_eq!(config.interactive, InteractiveMode::Never);
}

#[test]
fn flag_r() {
    let config = parse(&["-r", "dir"]);
    assert!(config.recursive);
}

#[test]
fn flag_capital_r() {
    let config = parse(&["-R", "dir"]);
    assert!(config.recursive);
}

#[test]
fn flag_d() {
    let config = parse(&["-d", "dir"]);
    assert!(config.dir);
}

#[test]
fn flag_v() {
    let config = parse(&["-v", "file"]);
    assert!(config.verbose);
}

#[test]
fn flag_i_lowercase() {
    let config = parse(&["-i", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Always);
    assert!(!config.force);
}

#[test]
fn flag_i_uppercase() {
    let config = parse(&["-I", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Once);
}

#[test]
fn combined_rf() {
    let config = parse(&["-rf", "dir"]);
    assert!(config.recursive);
    assert!(config.force);
    assert_eq!(config.interactive, InteractiveMode::Never);
}

#[test]
fn combined_rv() {
    let config = parse(&["-rv", "dir"]);
    assert!(config.recursive);
    assert!(config.verbose);
}

#[test]
fn combined_rfi_last_wins() {
    let config = parse(&["-rfi", "dir"]);
    assert!(config.recursive);
    assert!(!config.force);
    assert_eq!(config.interactive, InteractiveMode::Always);
}

#[test]
fn combined_rif_last_wins() {
    let config = parse(&["-rif", "dir"]);
    assert!(config.recursive);
    assert!(config.force);
    assert_eq!(config.interactive, InteractiveMode::Never);
}

#[test]
fn long_force() {
    let config = parse(&["--force", "file"]);
    assert!(config.force);
}

#[test]
fn long_recursive() {
    let config = parse(&["--recursive", "dir"]);
    assert!(config.recursive);
}

#[test]
fn long_dir() {
    let config = parse(&["--dir", "dir"]);
    assert!(config.dir);
}

#[test]
fn long_verbose() {
    let config = parse(&["--verbose", "file"]);
    assert!(config.verbose);
}

#[test]
fn interactive_equals_always() {
    let config = parse(&["--interactive=always", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Always);
}

#[test]
fn interactive_equals_once() {
    let config = parse(&["--interactive=once", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Once);
}

#[test]
fn interactive_equals_never() {
    let config = parse(&["--interactive=never", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Never);
}

#[test]
fn interactive_no_value_defaults_always() {
    let config = parse(&["--interactive", "file"]);
    assert_eq!(config.interactive, InteractiveMode::Always);
}

#[test]
fn interactive_invalid_returns_none() {
    let owned = vec!["--interactive=foo".to_string(), "file".to_string()];
    assert!(RmConfig::from_args(&owned).is_none());
}

#[test]
fn no_preserve_root() {
    let config = parse(&["--no-preserve-root", "/"]);
    assert!(!config.preserve_root);
}

#[test]
fn preserve_root_default() {
    let config = parse(&["/"]);
    assert!(config.preserve_root);
}

#[test]
fn preserve_root_all() {
    let config = parse(&["--preserve-root=all", "/"]);
    assert!(config.preserve_root);
    assert!(config.preserve_root_all);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-rf"]);
    assert!(!config.recursive);
    assert!(!config.force);
    assert_eq!(config.paths, vec!["-rf"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(RmConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(RmConfig::from_args(&owned).is_none());
}

#[test]
fn flags_before_paths() {
    let config = parse(&["-rfv", "a", "b", "c"]);
    assert!(config.recursive);
    assert!(config.force);
    assert!(config.verbose);
    assert_eq!(config.paths, vec!["a", "b", "c"]);
}

#[test]
fn separate_flags() {
    let config = parse(&["-r", "-f", "-v", "dir"]);
    assert!(config.recursive);
    assert!(config.force);
    assert!(config.verbose);
    assert_eq!(config.paths, vec!["dir"]);
}

#[test]
fn force_then_interactive_overrides() {
    let config = parse(&["--force", "--interactive=always", "file"]);
    assert!(!config.force);
    assert_eq!(config.interactive, InteractiveMode::Always);
}

#[test]
fn interactive_then_force_overrides() {
    let config = parse(&["-i", "-f", "file"]);
    assert!(config.force);
    assert_eq!(config.interactive, InteractiveMode::Never);
}
