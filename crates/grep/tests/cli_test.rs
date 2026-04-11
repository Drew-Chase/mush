use grep::cli::GrepConfig;

fn parse(args: &[&str]) -> GrepConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    GrepConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&["pattern"]);
    assert_eq!(config.pattern, "pattern");
    assert!(config.files.is_empty());
    assert!(!config.ignore_case);
    assert!(!config.invert);
    assert!(!config.word_regexp);
    assert!(!config.line_regexp);
    assert!(!config.count);
    assert!(!config.files_with_matches);
    assert!(!config.files_without_match);
    assert!(!config.line_number);
    assert!(!config.with_filename);
    assert!(!config.no_filename);
    assert!(!config.only_matching);
    assert!(!config.quiet);
    assert!(!config.recursive);
    assert_eq!(config.after_context, 0);
    assert_eq!(config.before_context, 0);
    assert_eq!(config.context, 0);
    assert!(config.max_count.is_none());
    assert!(!config.color);
    assert!(!config.fixed_strings);
    assert!(!config.extended_regexp);
    assert!(config.include_glob.is_empty());
    assert!(config.exclude_glob.is_empty());
    assert!(config.exclude_dir.is_empty());
}

#[test]
fn flag_i() {
    let config = parse(&["-i", "pattern"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_ignore_case_long() {
    let config = parse(&["--ignore-case", "pattern"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_v() {
    let config = parse(&["-v", "pattern"]);
    assert!(config.invert);
}

#[test]
fn flag_invert_match_long() {
    let config = parse(&["--invert-match", "pattern"]);
    assert!(config.invert);
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "pattern"]);
    assert!(config.line_number);
}

#[test]
fn flag_c() {
    let config = parse(&["-c", "pattern"]);
    assert!(config.count);
}

#[test]
fn flag_l() {
    let config = parse(&["-l", "pattern"]);
    assert!(config.files_with_matches);
}

#[test]
fn flag_capital_l() {
    let config = parse(&["-L", "pattern"]);
    assert!(config.files_without_match);
}

#[test]
fn flag_r() {
    let config = parse(&["-r", "pattern"]);
    assert!(config.recursive);
}

#[test]
fn flag_capital_r() {
    let config = parse(&["-R", "pattern"]);
    assert!(config.recursive);
}

#[test]
fn flag_recursive_long() {
    let config = parse(&["--recursive", "pattern"]);
    assert!(config.recursive);
}

#[test]
fn flag_a_separate() {
    let config = parse(&["-A", "3", "pattern"]);
    assert_eq!(config.after_context, 3);
}

#[test]
fn flag_a_inline() {
    let config = parse(&["-A3", "pattern"]);
    assert_eq!(config.after_context, 3);
}

#[test]
fn flag_b_separate() {
    let config = parse(&["-B", "2", "pattern"]);
    assert_eq!(config.before_context, 2);
}

#[test]
fn flag_b_inline() {
    let config = parse(&["-B2", "pattern"]);
    assert_eq!(config.before_context, 2);
}

#[test]
fn flag_c_context_separate() {
    let config = parse(&["-C", "5", "pattern"]);
    assert_eq!(config.context, 5);
}

#[test]
fn flag_c_context_inline() {
    let config = parse(&["-C5", "pattern"]);
    assert_eq!(config.context, 5);
}

#[test]
fn flag_capital_f() {
    let config = parse(&["-F", "pattern"]);
    assert!(config.fixed_strings);
}

#[test]
fn flag_fixed_strings_long() {
    let config = parse(&["--fixed-strings", "pattern"]);
    assert!(config.fixed_strings);
}

#[test]
fn flag_capital_e() {
    let config = parse(&["-E", "pattern"]);
    assert!(config.extended_regexp);
}

#[test]
fn flag_extended_regexp_long() {
    let config = parse(&["--extended-regexp", "pattern"]);
    assert!(config.extended_regexp);
}

#[test]
fn flag_color_always() {
    let config = parse(&["--color=always", "pattern"]);
    assert!(config.color);
}

#[test]
fn flag_color_never() {
    let config = parse(&["--color=never", "pattern"]);
    assert!(!config.color);
}

#[test]
fn flag_include() {
    let config = parse(&["--include", "*.rs", "pattern"]);
    assert_eq!(config.include_glob, vec!["*.rs"]);
}

#[test]
fn flag_include_equals() {
    let config = parse(&["--include=*.rs", "pattern"]);
    assert_eq!(config.include_glob, vec!["*.rs"]);
}

#[test]
fn flag_exclude() {
    let config = parse(&["--exclude", "*.log", "pattern"]);
    assert_eq!(config.exclude_glob, vec!["*.log"]);
}

#[test]
fn flag_exclude_equals() {
    let config = parse(&["--exclude=*.log", "pattern"]);
    assert_eq!(config.exclude_glob, vec!["*.log"]);
}

#[test]
fn flag_exclude_dir() {
    let config = parse(&["--exclude-dir", ".git", "pattern"]);
    assert_eq!(config.exclude_dir, vec![".git"]);
}

#[test]
fn flag_exclude_dir_equals() {
    let config = parse(&["--exclude-dir=.git", "pattern"]);
    assert_eq!(config.exclude_dir, vec![".git"]);
}

#[test]
fn flag_m_separate() {
    let config = parse(&["-m", "10", "pattern"]);
    assert_eq!(config.max_count, Some(10));
}

#[test]
fn flag_m_inline() {
    let config = parse(&["-m10", "pattern"]);
    assert_eq!(config.max_count, Some(10));
}

#[test]
fn flag_max_count_long() {
    let config = parse(&["--max-count", "5", "pattern"]);
    assert_eq!(config.max_count, Some(5));
}

#[test]
fn flag_max_count_equals() {
    let config = parse(&["--max-count=5", "pattern"]);
    assert_eq!(config.max_count, Some(5));
}

#[test]
fn flag_o() {
    let config = parse(&["-o", "pattern"]);
    assert!(config.only_matching);
}

#[test]
fn flag_only_matching_long() {
    let config = parse(&["--only-matching", "pattern"]);
    assert!(config.only_matching);
}

#[test]
fn flag_q() {
    let config = parse(&["-q", "pattern"]);
    assert!(config.quiet);
}

#[test]
fn flag_quiet_long() {
    let config = parse(&["--quiet", "pattern"]);
    assert!(config.quiet);
}

#[test]
fn flag_silent_long() {
    let config = parse(&["--silent", "pattern"]);
    assert!(config.quiet);
}

#[test]
fn flag_w() {
    let config = parse(&["-w", "pattern"]);
    assert!(config.word_regexp);
}

#[test]
fn flag_word_regexp_long() {
    let config = parse(&["--word-regexp", "pattern"]);
    assert!(config.word_regexp);
}

#[test]
fn flag_x() {
    let config = parse(&["-x", "pattern"]);
    assert!(config.line_regexp);
}

#[test]
fn flag_line_regexp_long() {
    let config = parse(&["--line-regexp", "pattern"]);
    assert!(config.line_regexp);
}

#[test]
fn flag_capital_h() {
    let config = parse(&["-H", "pattern"]);
    assert!(config.with_filename);
}

#[test]
fn flag_with_filename_long() {
    let config = parse(&["--with-filename", "pattern"]);
    assert!(config.with_filename);
}

#[test]
fn flag_h_no_filename() {
    let config = parse(&["-h", "pattern"]);
    assert!(config.no_filename);
}

#[test]
fn flag_no_filename_long() {
    let config = parse(&["--no-filename", "pattern"]);
    assert!(config.no_filename);
}

#[test]
fn combined_flags() {
    let config = parse(&["-inv", "pattern"]);
    assert!(config.ignore_case);
    assert!(config.line_number);
    assert!(config.invert);
}

#[test]
fn files_collected() {
    let config = parse(&["-i", "pattern", "foo.txt", "bar.txt"]);
    assert!(config.ignore_case);
    assert_eq!(config.pattern, "pattern");
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["pattern", "-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-v"]);
    assert!(!config.invert);
    assert_eq!(config.pattern, "-v");
}

#[test]
fn after_context_long_separate() {
    let config = parse(&["--after-context", "4", "pattern"]);
    assert_eq!(config.after_context, 4);
}

#[test]
fn after_context_long_equals() {
    let config = parse(&["--after-context=4", "pattern"]);
    assert_eq!(config.after_context, 4);
}

#[test]
fn before_context_long_separate() {
    let config = parse(&["--before-context", "4", "pattern"]);
    assert_eq!(config.before_context, 4);
}

#[test]
fn before_context_long_equals() {
    let config = parse(&["--before-context=4", "pattern"]);
    assert_eq!(config.before_context, 4);
}

#[test]
fn context_long_separate() {
    let config = parse(&["--context", "4", "pattern"]);
    assert_eq!(config.context, 4);
}

#[test]
fn context_long_equals() {
    let config = parse(&["--context=4", "pattern"]);
    assert_eq!(config.context, 4);
}

#[test]
fn multiple_include() {
    let config = parse(&["--include", "*.rs", "--include", "*.toml", "pattern"]);
    assert_eq!(config.include_glob, vec!["*.rs", "*.toml"]);
}

#[test]
fn multiple_exclude() {
    let config = parse(&["--exclude", "*.log", "--exclude", "*.tmp", "pattern"]);
    assert_eq!(config.exclude_glob, vec!["*.log", "*.tmp"]);
}
