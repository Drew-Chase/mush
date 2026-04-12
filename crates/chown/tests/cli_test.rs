use clap::Parser;

use chown::cli::ChownConfig;
use chown::ops::parse_owner_group;

fn parse(args: &[&str]) -> ChownConfig {
    let mut full = vec!["chown"];
    full.extend_from_slice(args);
    ChownConfig::parse_from(full).resolve().expect("resolve failed")
}

#[test]
fn basic_owner_file() {
    let config = parse(&["root", "file.txt"]);
    assert_eq!(config.owner_group, "root");
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn owner_group_colon() {
    let config = parse(&["root:wheel", "file.txt"]);
    assert_eq!(config.owner_group, "root:wheel");
}

#[test]
fn recursive_flag_short() {
    let config = parse(&["-R", "root", "dir"]);
    assert!(config.recursive);
}

#[test]
fn recursive_flag_long() {
    let config = parse(&["--recursive", "root", "dir"]);
    assert!(config.recursive);
}

#[test]
fn verbose_flag() {
    let config = parse(&["-v", "root", "file.txt"]);
    assert!(config.verbose);
}

#[test]
fn changes_flag() {
    let config = parse(&["-c", "root", "file.txt"]);
    assert!(config.changes);
}

#[test]
fn quiet_flag() {
    let config = parse(&["-f", "root", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_flag_long() {
    let config = parse(&["--quiet", "root", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn no_deref_flag_short() {
    let config = parse(&["-h", "root", "link"]);
    assert!(config.no_deref);
}

#[test]
fn no_deref_flag_long() {
    let config = parse(&["--no-dereference", "root", "link"]);
    assert!(config.no_deref);
}

#[test]
fn reference_flag() {
    let config = parse(&["--reference", "reffile", "target"]);
    assert_eq!(config.reference.as_deref(), Some("reffile"));
    assert!(config.owner_group.is_empty());
    assert_eq!(config.files, vec!["target"]);
}

#[test]
fn reference_flag_space() {
    let config = parse(&["--reference", "reffile", "target"]);
    assert_eq!(config.reference.as_deref(), Some("reffile"));
}

#[test]
fn help_returns_err() {
    assert!(ChownConfig::try_parse_from(["chown", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(ChownConfig::try_parse_from(["chown", "--version"]).is_err());
}

#[test]
fn parse_owner_group_owner_only() {
    assert_eq!(parse_owner_group("root"), (Some("root"), None));
}

#[test]
fn parse_owner_group_both() {
    assert_eq!(parse_owner_group("root:wheel"), (Some("root"), Some("wheel")));
}

#[test]
fn parse_owner_group_group_only() {
    assert_eq!(parse_owner_group(":wheel"), (None, Some("wheel")));
}

#[test]
fn parse_owner_group_colon_only() {
    assert_eq!(parse_owner_group(":"), (None, None));
}

#[test]
fn parse_owner_group_numeric() {
    assert_eq!(parse_owner_group("1000:1000"), (Some("1000"), Some("1000")));
}

#[cfg(not(unix))]
#[test]
fn chown_unsupported_on_windows() {
    use chown::ops::chown;
    let config = ChownConfig {
        owner_group: "root".to_string(),
        files: vec!["dummy".to_string()],
        ..Default::default()
    };
    assert!(chown(&config).is_err());
}
