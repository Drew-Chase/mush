use uniq::cli::UniqConfig;

fn parse(args: &[&str]) -> UniqConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    UniqConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.count);
    assert!(!config.repeated);
    assert!(!config.all_repeated);
    assert!(!config.unique);
    assert!(!config.ignore_case);
    assert_eq!(config.skip_fields, 0);
    assert_eq!(config.skip_chars, 0);
    assert_eq!(config.check_chars, None);
    assert_eq!(config.input, None);
    assert_eq!(config.output, None);
}

#[test]
fn flag_c() {
    let config = parse(&["-c"]);
    assert!(config.count);
}

#[test]
fn flag_d() {
    let config = parse(&["-d"]);
    assert!(config.repeated);
}

#[test]
fn flag_big_d() {
    let config = parse(&["-D"]);
    assert!(config.all_repeated);
}

#[test]
fn flag_u() {
    let config = parse(&["-u"]);
    assert!(config.unique);
}

#[test]
fn flag_i() {
    let config = parse(&["-i"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_f() {
    let config = parse(&["-f", "3"]);
    assert_eq!(config.skip_fields, 3);
}

#[test]
fn flag_f_attached() {
    let config = parse(&["-f5"]);
    assert_eq!(config.skip_fields, 5);
}

#[test]
fn flag_s() {
    let config = parse(&["-s", "2"]);
    assert_eq!(config.skip_chars, 2);
}

#[test]
fn flag_w() {
    let config = parse(&["-w", "10"]);
    assert_eq!(config.check_chars, Some(10));
}

#[test]
fn long_flags() {
    let config = parse(&["--count", "--repeated", "--unique", "--ignore-case"]);
    assert!(config.count);
    assert!(config.repeated);
    assert!(config.unique);
    assert!(config.ignore_case);
}

#[test]
fn long_skip_fields() {
    let config = parse(&["--skip-fields", "4"]);
    assert_eq!(config.skip_fields, 4);
}

#[test]
fn long_skip_fields_eq() {
    let config = parse(&["--skip-fields=4"]);
    assert_eq!(config.skip_fields, 4);
}

#[test]
fn long_skip_chars() {
    let config = parse(&["--skip-chars", "7"]);
    assert_eq!(config.skip_chars, 7);
}

#[test]
fn long_check_chars() {
    let config = parse(&["--check-chars", "3"]);
    assert_eq!(config.check_chars, Some(3));
}

#[test]
fn positional_input_output() {
    let config = parse(&["input.txt", "output.txt"]);
    assert_eq!(config.input, Some("input.txt".to_string()));
    assert_eq!(config.output, Some("output.txt".to_string()));
}

#[test]
fn combined_flags() {
    let config = parse(&["-cdi"]);
    assert!(config.count);
    assert!(config.repeated);
    assert!(config.ignore_case);
}

// ops integration tests

use std::io::Cursor;
use uniq::ops;

fn run_uniq(input: &str, args: &[&str]) -> String {
    let config = parse(args);
    let mut inp = Cursor::new(input.as_bytes().to_vec());
    let mut out = Vec::new();
    ops::uniq(&mut inp, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn default_collapse() {
    let result = run_uniq("aaa\naaa\nbbb\nccc\nccc\n", &[]);
    assert_eq!(result, "aaa\nbbb\nccc\n");
}

#[test]
fn count_flag() {
    let result = run_uniq("aaa\naaa\nbbb\n", &["-c"]);
    assert_eq!(result, "      2 aaa\n      1 bbb\n");
}

#[test]
fn repeated_flag() {
    let result = run_uniq("aaa\naaa\nbbb\nccc\nccc\n", &["-d"]);
    assert_eq!(result, "aaa\nccc\n");
}

#[test]
fn all_repeated_flag() {
    let result = run_uniq("aaa\naaa\nbbb\nccc\nccc\n", &["-D"]);
    assert_eq!(result, "aaa\naaa\nccc\nccc\n");
}

#[test]
fn unique_flag() {
    let result = run_uniq("aaa\naaa\nbbb\nccc\nccc\n", &["-u"]);
    assert_eq!(result, "bbb\n");
}

#[test]
fn ignore_case_flag() {
    let result = run_uniq("AAA\naaa\nbbb\n", &["-i"]);
    assert_eq!(result, "AAA\nbbb\n");
}

#[test]
fn skip_fields_flag() {
    let result = run_uniq("1 aaa\n2 aaa\n3 bbb\n", &["-f", "1"]);
    assert_eq!(result, "1 aaa\n3 bbb\n");
}

#[test]
fn skip_chars_flag() {
    let result = run_uniq("Xaaa\nYaaa\nZbbb\n", &["-s", "1"]);
    assert_eq!(result, "Xaaa\nZbbb\n");
}

#[test]
fn check_chars_flag() {
    let result = run_uniq("abcX\nabcY\ndefZ\n", &["-w", "3"]);
    assert_eq!(result, "abcX\ndefZ\n");
}
