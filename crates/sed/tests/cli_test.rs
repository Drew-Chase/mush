use clap::Parser;

use sed::cli::SedConfig;

fn parse(args: &[&str]) -> SedConfig {
    let mut full = vec!["sed"];
    full.extend_from_slice(args);
    SedConfig::parse_from(full)
}

#[test]
fn positional_script() {
    let config = parse(&["s/a/b/", "file.txt"]);
    assert_eq!(config.effective_scripts(), vec!["s/a/b/"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn expression_short_flag() {
    let config = parse(&["-e", "s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn expression_long_flag() {
    let config = parse(&["--expression=s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn expression_long_flag_separate() {
    let config = parse(&["--expression", "s/a/b/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn multiple_expressions() {
    let config = parse(&["-e", "s/a/b/", "-e", "s/c/d/", "file.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/", "s/c/d/"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn file_short_flag() {
    let config = parse(&["-f", "script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn file_long_flag() {
    let config = parse(&["--file=script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn file_long_flag_separate() {
    let config = parse(&["--file", "script.sed", "file.txt"]);
    assert_eq!(config.script_files, vec!["script.sed"]);
    assert_eq!(config.effective_files(), &["file.txt"]);
}

#[test]
fn in_place_no_suffix() {
    let config = parse(&["-i", "-e", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(None));
}

#[test]
fn in_place_long_no_suffix() {
    let config = parse(&["--in-place", "-e", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(None));
}

#[test]
fn in_place_long_with_suffix() {
    let config = parse(&["--in-place=.bak", "-e", "s/a/b/", "file.txt"]);
    assert_eq!(config.in_place, Some(Some(".bak".to_string())));
}

#[test]
fn quiet_short_flag() {
    let config = parse(&["-n", "-e", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn quiet_long_flag() {
    let config = parse(&["--quiet", "-e", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn silent_long_flag() {
    let config = parse(&["--silent", "-e", "s/a/b/", "file.txt"]);
    assert!(config.quiet);
}

#[test]
fn extended_regexp_r() {
    let config = parse(&["-r", "-e", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn extended_regexp_big_e() {
    let config = parse(&["-E", "-e", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn extended_regexp_long() {
    let config = parse(&["--regexp-extended", "-e", "s/a/b/", "file.txt"]);
    assert!(config.extended_regexp);
}

#[test]
fn separate_short() {
    let config = parse(&["-s", "-e", "s/a/b/", "file.txt"]);
    assert!(config.separate);
}

#[test]
fn separate_long() {
    let config = parse(&["--separate", "-e", "s/a/b/", "file.txt"]);
    assert!(config.separate);
}

#[test]
fn script_and_files() {
    let config = parse(&["-e", "s/a/b/", "file1.txt", "file2.txt"]);
    assert_eq!(config.scripts, vec!["s/a/b/"]);
    assert_eq!(config.effective_files(), &["file1.txt", "file2.txt"]);
}

#[test]
fn no_files_defaults_empty() {
    let config = parse(&["s/a/b/"]);
    assert_eq!(config.effective_scripts(), vec!["s/a/b/"]);
    assert!(config.effective_files().is_empty());
}

#[test]
fn stdin_dash() {
    let config = parse(&["-e", "s/a/b/", "-"]);
    assert_eq!(config.effective_files(), &["-"]);
}
