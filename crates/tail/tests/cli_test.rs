use tail::cli::TailConfig;

fn parse(args: &[&str]) -> TailConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    TailConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert_eq!(config.lines, 10);
    assert_eq!(config.bytes, None);
    assert!(!config.follow);
    assert!(!config.quiet);
    assert!(!config.verbose);
    assert!(config.files.is_empty());
}

#[test]
fn flag_n_separate() {
    let config = parse(&["-n", "20"]);
    assert_eq!(config.lines, 20);
}

#[test]
fn flag_n_attached() {
    let config = parse(&["-n5"]);
    assert_eq!(config.lines, 5);
}

#[test]
fn long_lines_equals() {
    let config = parse(&["--lines=15"]);
    assert_eq!(config.lines, 15);
}

#[test]
fn long_lines_separate() {
    let config = parse(&["--lines", "25"]);
    assert_eq!(config.lines, 25);
}

#[test]
fn flag_c_separate() {
    let config = parse(&["-c", "100"]);
    assert_eq!(config.bytes, Some(100));
}

#[test]
fn flag_c_attached() {
    let config = parse(&["-c50"]);
    assert_eq!(config.bytes, Some(50));
}

#[test]
fn long_bytes_equals() {
    let config = parse(&["--bytes=200"]);
    assert_eq!(config.bytes, Some(200));
}

#[test]
fn long_bytes_separate() {
    let config = parse(&["--bytes", "300"]);
    assert_eq!(config.bytes, Some(300));
}

#[test]
fn flag_f() {
    let config = parse(&["-f"]);
    assert!(config.follow);
}

#[test]
fn long_follow() {
    let config = parse(&["--follow"]);
    assert!(config.follow);
}

#[test]
fn flag_q() {
    let config = parse(&["-q"]);
    assert!(config.quiet);
}

#[test]
fn long_quiet() {
    let config = parse(&["--quiet"]);
    assert!(config.quiet);
}

#[test]
fn long_silent() {
    let config = parse(&["--silent"]);
    assert!(config.quiet);
}

#[test]
fn flag_v() {
    let config = parse(&["-v"]);
    assert!(config.verbose);
}

#[test]
fn long_verbose() {
    let config = parse(&["--verbose"]);
    assert!(config.verbose);
}

#[test]
fn combined_fq() {
    let config = parse(&["-fq"]);
    assert!(config.follow);
    assert!(config.quiet);
}

#[test]
fn combined_fv() {
    let config = parse(&["-fv"]);
    assert!(config.follow);
    assert!(config.verbose);
}

#[test]
fn positional_files() {
    let config = parse(&["file1.txt", "file2.txt"]);
    assert_eq!(config.files, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn stdin_dash() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn flags_and_files() {
    let config = parse(&["-n", "5", "-f", "myfile.txt"]);
    assert_eq!(config.lines, 5);
    assert!(config.follow);
    assert_eq!(config.files, vec!["myfile.txt"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-f"]);
    assert!(!config.follow);
    assert_eq!(config.files, vec!["-f"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(TailConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(TailConfig::from_args(&owned).is_none());
}
