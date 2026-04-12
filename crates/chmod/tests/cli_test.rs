use clap::Parser;

use chmod::cli::ChmodConfig;
use chmod::ops::parse_mode;

fn parse(args: &[&str]) -> ChmodConfig {
    let mut full = vec!["chmod"];
    full.extend_from_slice(args);
    ChmodConfig::parse_from(full).resolve().expect("resolve failed")
}

#[test]
fn basic_octal_mode() {
    let config = parse(&["755", "file.txt"]);
    assert_eq!(config.mode, "755");
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn symbolic_mode() {
    let config = parse(&["u+x", "file.txt"]);
    assert_eq!(config.mode, "u+x");
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn recursive_flag_short() {
    let config = parse(&["-R", "755", "dir"]);
    assert!(config.recursive);
    assert_eq!(config.mode, "755");
}

#[test]
fn recursive_flag_long() {
    let config = parse(&["--recursive", "755", "dir"]);
    assert!(config.recursive);
}

#[test]
fn verbose_flag() {
    let config = parse(&["-v", "644", "file.txt"]);
    assert!(config.verbose);
}

#[test]
fn changes_flag() {
    let config = parse(&["-c", "644", "file.txt"]);
    assert!(config.changes);
}

#[test]
fn quiet_flag_f() {
    let config = parse(&["-f", "644", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_flag_long_silent() {
    let config = parse(&["--silent", "644", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_flag_long_quiet() {
    let config = parse(&["--quiet", "644", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn help_returns_err() {
    assert!(ChmodConfig::try_parse_from(["chmod", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(ChmodConfig::try_parse_from(["chmod", "--version"]).is_err());
}

#[test]
fn parse_mode_octal() {
    assert_eq!(parse_mode("755", 0).unwrap(), 0o755);
    assert_eq!(parse_mode("644", 0).unwrap(), 0o644);
    assert_eq!(parse_mode("000", 0).unwrap(), 0o000);
    assert_eq!(parse_mode("777", 0).unwrap(), 0o777);
}

#[test]
fn parse_mode_symbolic_u_plus_x() {
    assert_eq!(parse_mode("u+x", 0o644).unwrap(), 0o744);
}

#[test]
fn parse_mode_symbolic_combined() {
    assert_eq!(parse_mode("u+x,go-w", 0o666).unwrap(), 0o744);
}

#[test]
fn parse_mode_symbolic_equals() {
    assert_eq!(parse_mode("u=rwx,go=rx", 0o000).unwrap(), 0o755);
}

#[cfg(not(unix))]
#[test]
fn chmod_readonly_on_windows() {
    use chmod::ops::chmod;
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();

    let config = ChmodConfig {
        mode: "444".to_string(),
        files: vec![file.to_string_lossy().to_string()],
        ..Default::default()
    };
    chmod(&file, &config).unwrap();

    let meta = std::fs::metadata(&file).unwrap();
    assert!(meta.permissions().readonly());

    // Clean up: make writable so tempdir can delete
    let mut perms = meta.permissions();
    perms.set_readonly(false);
    std::fs::set_permissions(&file, perms).unwrap();
}
