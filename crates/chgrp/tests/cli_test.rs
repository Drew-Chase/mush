use chgrp::cli::ChgrpConfig;

fn parse(args: &[&str]) -> ChgrpConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    ChgrpConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn basic_group_file() {
    let config = parse(&["staff", "file.txt"]);
    assert_eq!(config.group, "staff");
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn recursive_flag_short() {
    let config = parse(&["-R", "staff", "dir"]);
    assert!(config.recursive);
}

#[test]
fn recursive_flag_long() {
    let config = parse(&["--recursive", "staff", "dir"]);
    assert!(config.recursive);
}

#[test]
fn verbose_flag() {
    let config = parse(&["-v", "staff", "file.txt"]);
    assert!(config.verbose);
}

#[test]
fn changes_flag() {
    let config = parse(&["-c", "staff", "file.txt"]);
    assert!(config.changes);
}

#[test]
fn quiet_flag() {
    let config = parse(&["-f", "staff", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_flag_long() {
    let config = parse(&["--quiet", "staff", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn no_deref_flag_short() {
    let config = parse(&["-h", "staff", "link"]);
    assert!(config.no_deref);
}

#[test]
fn no_deref_flag_long() {
    let config = parse(&["--no-dereference", "staff", "link"]);
    assert!(config.no_deref);
}

#[test]
fn reference_flag() {
    let config = parse(&["--reference=reffile", "target"]);
    assert_eq!(config.reference.as_deref(), Some("reffile"));
    assert!(config.group.is_empty());
    assert_eq!(config.files, vec!["target"]);
}

#[test]
fn reference_flag_space() {
    let config = parse(&["--reference", "reffile", "target"]);
    assert_eq!(config.reference.as_deref(), Some("reffile"));
}

#[test]
fn combined_flags() {
    let config = parse(&["-Rvcf", "wheel", "dir1", "dir2"]);
    assert!(config.recursive);
    assert!(config.verbose);
    assert!(config.changes);
    assert!(config.quiet);
    assert_eq!(config.files, vec!["dir1", "dir2"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(ChgrpConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(ChgrpConfig::from_args(&owned).is_none());
}

#[test]
fn missing_operand_returns_none() {
    let owned: Vec<String> = vec![];
    assert!(ChgrpConfig::from_args(&owned).is_none());
}

#[test]
fn missing_file_returns_none() {
    let owned = vec!["staff".to_string()];
    assert!(ChgrpConfig::from_args(&owned).is_none());
}

#[cfg(not(unix))]
#[test]
fn chgrp_unsupported_on_windows() {
    use chgrp::ops::chgrp;
    let config = ChgrpConfig {
        group: "staff".to_string(),
        files: vec!["dummy".to_string()],
        ..Default::default()
    };
    assert!(chgrp(&config).is_err());
}
