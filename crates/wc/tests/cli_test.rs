use wc::cli::WcConfig;

fn parse(args: &[&str]) -> WcConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    WcConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults_lines_words_bytes() {
    let config = parse(&[]);
    assert!(config.lines);
    assert!(config.words);
    assert!(config.bytes);
    assert!(!config.chars);
    assert!(!config.max_line_length);
}

#[test]
fn flag_l() {
    let config = parse(&["-l"]);
    assert!(config.lines);
    assert!(!config.words);
    assert!(!config.bytes);
}

#[test]
fn flag_w() {
    let config = parse(&["-w"]);
    assert!(!config.lines);
    assert!(config.words);
    assert!(!config.bytes);
}

#[test]
fn flag_c() {
    let config = parse(&["-c"]);
    assert!(!config.lines);
    assert!(!config.words);
    assert!(config.bytes);
}

#[test]
fn flag_m() {
    let config = parse(&["-m"]);
    assert!(!config.lines);
    assert!(!config.words);
    assert!(!config.bytes);
    assert!(config.chars);
}

#[test]
fn flag_big_l() {
    let config = parse(&["-L"]);
    assert!(!config.lines);
    assert!(config.max_line_length);
}

#[test]
fn combined_lw() {
    let config = parse(&["-lw"]);
    assert!(config.lines);
    assert!(config.words);
    assert!(!config.bytes);
}

#[test]
fn long_flags() {
    let config = parse(&["--lines", "--words"]);
    assert!(config.lines);
    assert!(config.words);
    assert!(!config.bytes);
}

#[test]
fn long_flag_max_line_length() {
    let config = parse(&["--max-line-length"]);
    assert!(config.max_line_length);
    assert!(!config.lines);
}

#[test]
fn files_collected() {
    let config = parse(&["-l", "foo.txt", "bar.txt"]);
    assert!(config.lines);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["-l", "-"]);
    assert!(config.lines);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-l"]);
    assert!(config.lines); // defaults since no flags
    assert!(config.words);
    assert!(config.bytes);
    assert_eq!(config.files, vec!["-l"]);
}

#[test]
fn multiple_files_no_flags() {
    let config = parse(&["a.txt", "b.txt"]);
    assert!(config.lines);
    assert!(config.words);
    assert!(config.bytes);
    assert_eq!(config.files, vec!["a.txt", "b.txt"]);
}
