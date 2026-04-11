use sed::cli::SedConfig;

fn parse(args: &[&str]) -> SedConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SedConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn positional_script() {
    let config = parse(&["s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn expression_short_flag() {
    let config = parse(&["-e", "s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn expression_long_flag() {
    let config = parse(&["--expression=s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn expression_long_flag_separate() {
    let config = parse(&["--expression", "s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn multiple_expressions() {
    let config = parse(&["-e", "s/a/b/", "-e", "s/c/d/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/", "s/c/d/"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn file_short_flag() {
    let config = parse(&["-f", "script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn file_long_flag() {
    let config = parse(&["--file=script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn file_long_flag_separate() {
    let config = parse(&["--file", "script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.files, vec!["file.txt"]);
}

#[test]
fn in_place_no_suffix() {
    let config = parse(&["-i", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(None));
}

#[test]
fn in_place_with_suffix() {
    let config = parse(&["-i.bak", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(Some(".bak".to_string())));
}

#[test]
fn in_place_long_no_suffix() {
    let config = parse(&["--in-place", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(None));
}

#[test]
fn in_place_long_with_suffix() {
    let config = parse(&["--in-place=.bak", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(Some(".bak".to_string())));
}

#[test]
fn quiet_short_flag() {
    let config = parse(&["-n", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_long_flag() {
    let config = parse(&["--quiet", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn silent_long_flag() {
    let config = parse(&["--silent", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn extended_regexp_r() {
    let config = parse(&["-r", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn extended_regexp_big_e() {
    let config = parse(&["-E", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn extended_regexp_long() {
    let config = parse(&["--regexp-extended", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn separate_short() {
    let config = parse(&["-s", "s/a/b/", "file.txt"]);
    assert!(config.separate);
}

#[test]
fn separate_long() {
    let config = parse(&["--separate", "s/a/b/", "file.txt"]);
    assert!(config.separate);
}

#[test]
fn combined_short_flags() {
    let config = parse(&["-nE", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
    assert!(config.extended_regexp);
}

#[test]
fn script_and_files() {
    let config = parse(&["-e", "s/a/b/", "file1.txt", "file2.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.files, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn no_files_defaults_empty() {
    let config = parse(&["s/a/b/"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert!(config.files.is_empty());
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(SedConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(SedConfig::from_args(&owned).is_none());
}

#[test]
fn stdin_dash() {
    let config = parse(&["s/a/b/", "-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["-e", "s/a/b/", "--", "-n"]);
    assert!(!config.quiet);
    assert_eq!(config.files, vec!["-n"]);
}
