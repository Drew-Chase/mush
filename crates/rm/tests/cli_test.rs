use clap::Parser;

use rm::cli::{InteractiveMode, RmConfig};

fn parse(args: &[&str]) -> RmConfig {
    let mut full = vec!["rm"];
    full.extend_from_slice(args);
    RmConfig::parse_from(full).resolve().expect("resolve failed")
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
    let config = parse(&["-r", "-f", "dir"]);
    assert!(config.recursive);
    assert!(config.force);
    assert_eq!(config.interactive, InteractiveMode::Never);
}

#[test]
fn combined_rv() {
    let config = parse(&["-r", "-v", "dir"]);
    assert!(config.recursive);
    assert!(config.verbose);
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
fn help_returns_err() {
    assert!(RmConfig::try_parse_from(["rm", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(RmConfig::try_parse_from(["rm", "--version"]).is_err());
}

#[test]
fn separate_flags() {
    let config = parse(&["-r", "-f", "-v", "dir"]);
    assert!(config.recursive);
    assert!(config.force);
    assert!(config.verbose);
    assert_eq!(config.paths, vec!["dir"]);
}
