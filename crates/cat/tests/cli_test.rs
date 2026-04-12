use clap::Parser;

use cat::cli::CatConfig;

fn parse(args: &[&str]) -> CatConfig {
    let mut full = vec!["cat"];
    full.extend_from_slice(args);
    let mut config = CatConfig::parse_from(full);
    config.resolve();
    config
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.number);
    assert!(!config.number_nonblank);
    assert!(!config.squeeze_blank);
    assert!(!config.show_ends);
    assert!(!config.show_tabs);
    assert!(!config.show_nonprinting);
    assert!(config.files.is_empty());
}

#[test]
fn flag_n() {
    let config = parse(&["-n"]);
    assert!(config.number);
}

#[test]
fn long_number() {
    let config = parse(&["--number"]);
    assert!(config.number);
}

#[test]
fn flag_b() {
    let config = parse(&["-b"]);
    assert!(config.number_nonblank);
}

#[test]
fn long_number_nonblank() {
    let config = parse(&["--number-nonblank"]);
    assert!(config.number_nonblank);
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.squeeze_blank);
}

#[test]
fn long_squeeze_blank() {
    let config = parse(&["--squeeze-blank"]);
    assert!(config.squeeze_blank);
}

#[test]
fn flag_big_e() {
    let config = parse(&["-E"]);
    assert!(config.show_ends);
    assert!(!config.show_nonprinting);
}

#[test]
fn long_show_ends() {
    let config = parse(&["--show-ends"]);
    assert!(config.show_ends);
}

#[test]
fn flag_big_t() {
    let config = parse(&["-T"]);
    assert!(config.show_tabs);
    assert!(!config.show_nonprinting);
}

#[test]
fn long_show_tabs() {
    let config = parse(&["--show-tabs"]);
    assert!(config.show_tabs);
}

#[test]
fn flag_v() {
    let config = parse(&["-v"]);
    assert!(config.show_nonprinting);
}

#[test]
fn long_show_nonprinting() {
    let config = parse(&["--show-nonprinting"]);
    assert!(config.show_nonprinting);
}

#[test]
fn flag_big_a() {
    let config = parse(&["-A"]);
    assert!(config.show_nonprinting);
    assert!(config.show_ends);
    assert!(config.show_tabs);
}

#[test]
fn long_show_all() {
    let config = parse(&["--show-all"]);
    assert!(config.show_nonprinting);
    assert!(config.show_ends);
    assert!(config.show_tabs);
}

#[test]
fn flag_e_compound() {
    let config = parse(&["-e"]);
    assert!(config.show_nonprinting);
    assert!(config.show_ends);
    assert!(!config.show_tabs);
}

#[test]
fn flag_t_compound() {
    let config = parse(&["-t"]);
    assert!(config.show_nonprinting);
    assert!(config.show_tabs);
    assert!(!config.show_ends);
}

#[test]
fn combined_nb() {
    let config = parse(&["-nb"]);
    assert!(config.number);
    assert!(config.number_nonblank);
}

#[test]
fn combined_sn() {
    let config = parse(&["-sn"]);
    assert!(config.squeeze_blank);
    assert!(config.number);
}

#[test]
fn combined_vet() {
    let config = parse(&["-vET"]);
    assert!(config.show_nonprinting);
    assert!(config.show_ends);
    assert!(config.show_tabs);
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
    let config = parse(&["-n", "file1.txt", "file2.txt"]);
    assert!(config.number);
    assert_eq!(config.files, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-n"]);
    assert!(!config.number);
    assert_eq!(config.files, vec!["-n"]);
}

#[test]
fn help_returns_err() {
    assert!(CatConfig::try_parse_from(["cat", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(CatConfig::try_parse_from(["cat", "--version"]).is_err());
}

#[test]
fn multiple_files_with_flags() {
    let config = parse(&["-bsE", "a.txt", "b.txt", "c.txt"]);
    assert!(config.number_nonblank);
    assert!(config.squeeze_blank);
    assert!(config.show_ends);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
}
