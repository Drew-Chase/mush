use less::cli::LessConfig;

fn parse(args: &[&str]) -> LessConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    LessConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.line_numbers);
    assert!(!config.chop_long_lines);
    assert!(!config.ignore_case);
    assert!(!config.quit_if_one_screen);
    assert!(!config.raw_control_chars);
    assert!(!config.no_init);
    assert_eq!(config.start_line, None);
    assert_eq!(config.start_pattern, None);
    assert!(config.files.is_empty());
}

#[test]
fn flag_line_numbers_short() {
    let config = parse(&["-N"]);
    assert!(config.line_numbers);
}

#[test]
fn flag_line_numbers_long() {
    let config = parse(&["--line-numbers"]);
    assert!(config.line_numbers);
}

#[test]
fn flag_chop_long_lines_short() {
    let config = parse(&["-S"]);
    assert!(config.chop_long_lines);
}

#[test]
fn flag_chop_long_lines_long() {
    let config = parse(&["--chop-long-lines"]);
    assert!(config.chop_long_lines);
}

#[test]
fn flag_ignore_case_short() {
    let config = parse(&["-i"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_ignore_case_long() {
    let config = parse(&["--ignore-case"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_quit_if_one_screen_short() {
    let config = parse(&["-F"]);
    assert!(config.quit_if_one_screen);
}

#[test]
fn flag_quit_if_one_screen_long() {
    let config = parse(&["--quit-if-one-screen"]);
    assert!(config.quit_if_one_screen);
}

#[test]
fn flag_raw_control_chars_short() {
    let config = parse(&["-R"]);
    assert!(config.raw_control_chars);
}

#[test]
fn flag_raw_control_chars_long() {
    let config = parse(&["--RAW-CONTROL-CHARS"]);
    assert!(config.raw_control_chars);
}

#[test]
fn flag_no_init_short() {
    let config = parse(&["-X"]);
    assert!(config.no_init);
}

#[test]
fn flag_no_init_long() {
    let config = parse(&["--no-init"]);
    assert!(config.no_init);
}

#[test]
fn start_line_with_n() {
    let config = parse(&["-n", "42"]);
    assert_eq!(config.start_line, Some(42));
}

#[test]
fn start_line_with_plus() {
    let config = parse(&["+100"]);
    assert_eq!(config.start_line, Some(100));
}

#[test]
fn start_pattern() {
    let config = parse(&["+/TODO"]);
    assert_eq!(config.start_pattern, Some("TODO".to_string()));
}

#[test]
fn files_collected() {
    let config = parse(&["-N", "-S", "foo.txt", "bar.txt"]);
    assert!(config.line_numbers);
    assert!(config.chop_long_lines);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn combined_short_flags() {
    let config = parse(&["-NSi"]);
    assert!(config.line_numbers);
    assert!(config.chop_long_lines);
    assert!(config.ignore_case);
}

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(LessConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(LessConfig::from_args(&owned).is_none());
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-N"]);
    assert!(!config.line_numbers);
    assert_eq!(config.files, vec!["-N"]);
}

#[test]
fn stdin_dash() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn all_flags_combined() {
    let config = parse(&["-N", "-S", "-i", "-F", "-R", "-X", "+/pattern", "file.txt"]);
    assert!(config.line_numbers);
    assert!(config.chop_long_lines);
    assert!(config.ignore_case);
    assert!(config.quit_if_one_screen);
    assert!(config.raw_control_chars);
    assert!(config.no_init);
    assert_eq!(config.start_pattern, Some("pattern".to_string()));
    assert_eq!(config.files, vec!["file.txt"]);
}
