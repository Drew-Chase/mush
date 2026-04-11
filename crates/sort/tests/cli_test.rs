use sort::cli::{SortConfig, SortKey};

fn parse(args: &[&str]) -> SortConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SortConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.reverse);
    assert!(!config.numeric);
    assert!(!config.human_numeric);
    assert!(!config.ignore_case);
    assert!(!config.dictionary);
    assert!(!config.ignore_blanks);
    assert!(!config.unique);
    assert!(!config.stable);
    assert!(!config.check);
    assert!(!config.merge);
    assert!(config.key.is_empty());
    assert!(config.separator.is_none());
    assert!(config.output.is_none());
    assert!(config.files.is_empty());
}

#[test]
fn flag_r() {
    let config = parse(&["-r"]);
    assert!(config.reverse);
}

#[test]
fn flag_n() {
    let config = parse(&["-n"]);
    assert!(config.numeric);
}

#[test]
fn flag_h() {
    let config = parse(&["-h"]);
    assert!(config.human_numeric);
}

#[test]
fn flag_f() {
    let config = parse(&["-f"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_d() {
    let config = parse(&["-d"]);
    assert!(config.dictionary);
}

#[test]
fn flag_b() {
    let config = parse(&["-b"]);
    assert!(config.ignore_blanks);
}

#[test]
fn flag_u() {
    let config = parse(&["-u"]);
    assert!(config.unique);
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.stable);
}

#[test]
fn flag_c() {
    let config = parse(&["-c"]);
    assert!(config.check);
}

#[test]
fn flag_m() {
    let config = parse(&["-m"]);
    assert!(config.merge);
}

#[test]
fn flag_k_separate() {
    let config = parse(&["-k", "2"]);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0], SortKey { start_field: 2, end_field: None });
}

#[test]
fn flag_k_with_range() {
    let config = parse(&["-k", "2,3"]);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0], SortKey { start_field: 2, end_field: Some(3) });
}

#[test]
fn flag_k_multiple() {
    let config = parse(&["-k", "1", "-k", "3"]);
    assert_eq!(config.key.len(), 2);
    assert_eq!(config.key[0].start_field, 1);
    assert_eq!(config.key[1].start_field, 3);
}

#[test]
fn flag_t_separate() {
    let config = parse(&["-t", ":"]);
    assert_eq!(config.separator, Some(':'));
}

#[test]
fn flag_t_inline() {
    let config = parse(&["-t:"]);
    assert_eq!(config.separator, Some(':'));
}

#[test]
fn flag_o_separate() {
    let config = parse(&["-o", "out.txt"]);
    assert_eq!(config.output, Some("out.txt".to_string()));
}

#[test]
fn flag_o_inline() {
    let config = parse(&["-oout.txt"]);
    assert_eq!(config.output, Some("out.txt".to_string()));
}

#[test]
fn long_reverse() {
    let config = parse(&["--reverse"]);
    assert!(config.reverse);
}

#[test]
fn long_numeric_sort() {
    let config = parse(&["--numeric-sort"]);
    assert!(config.numeric);
}

#[test]
fn long_human_numeric_sort() {
    let config = parse(&["--human-numeric-sort"]);
    assert!(config.human_numeric);
}

#[test]
fn long_ignore_case() {
    let config = parse(&["--ignore-case"]);
    assert!(config.ignore_case);
}

#[test]
fn long_dictionary_order() {
    let config = parse(&["--dictionary-order"]);
    assert!(config.dictionary);
}

#[test]
fn long_ignore_leading_blanks() {
    let config = parse(&["--ignore-leading-blanks"]);
    assert!(config.ignore_blanks);
}

#[test]
fn long_unique() {
    let config = parse(&["--unique"]);
    assert!(config.unique);
}

#[test]
fn long_stable() {
    let config = parse(&["--stable"]);
    assert!(config.stable);
}

#[test]
fn long_check() {
    let config = parse(&["--check"]);
    assert!(config.check);
}

#[test]
fn long_merge() {
    let config = parse(&["--merge"]);
    assert!(config.merge);
}

#[test]
fn long_key_equals() {
    let config = parse(&["--key=2,4"]);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0], SortKey { start_field: 2, end_field: Some(4) });
}

#[test]
fn long_key_separate() {
    let config = parse(&["--key", "3"]);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0], SortKey { start_field: 3, end_field: None });
}

#[test]
fn long_field_separator_equals() {
    let config = parse(&["--field-separator=,"]);
    assert_eq!(config.separator, Some(','));
}

#[test]
fn long_field_separator_separate() {
    let config = parse(&["--field-separator", ","]);
    assert_eq!(config.separator, Some(','));
}

#[test]
fn long_output_equals() {
    let config = parse(&["--output=result.txt"]);
    assert_eq!(config.output, Some("result.txt".to_string()));
}

#[test]
fn long_output_separate() {
    let config = parse(&["--output", "result.txt"]);
    assert_eq!(config.output, Some("result.txt".to_string()));
}

#[test]
fn combined_flags() {
    let config = parse(&["-rnfu"]);
    assert!(config.reverse);
    assert!(config.numeric);
    assert!(config.ignore_case);
    assert!(config.unique);
}

#[test]
fn files_collected() {
    let config = parse(&["-r", "foo.txt", "bar.txt"]);
    assert!(config.reverse);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["-r", "-"]);
    assert!(config.reverse);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-r"]);
    assert!(!config.reverse);
    assert_eq!(config.files, vec!["-r"]);
}

#[test]
fn combined_k_inline() {
    let config = parse(&["-k2,3"]);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0], SortKey { start_field: 2, end_field: Some(3) });
}

#[test]
fn combined_flags_with_k() {
    let config = parse(&["-nk", "2"]);
    assert!(config.numeric);
    assert_eq!(config.key.len(), 1);
    assert_eq!(config.key[0].start_field, 2);
}
