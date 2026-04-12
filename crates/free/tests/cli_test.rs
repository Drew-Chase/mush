use clap::Parser;

use free::cli::FreeConfig;

fn parse(args: &[&str]) -> FreeConfig {
    let mut full = vec!["free"];
    full.extend_from_slice(args);
    FreeConfig::parse_from(full)
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.bytes);
    assert!(!config.kibi);
    assert!(!config.mebi);
    assert!(!config.gibi);
    assert!(!config.human);
    assert!(!config.si);
    assert!(!config.total);
    assert!(!config.wide);
}

#[test]
fn flag_b_bytes() {
    let config = parse(&["-b"]);
    assert!(config.bytes);
}

#[test]
fn flag_k_kibi() {
    let config = parse(&["-k"]);
    assert!(config.kibi);
}

#[test]
fn flag_m_mebi() {
    let config = parse(&["-m"]);
    assert!(config.mebi);
}

#[test]
fn flag_g_gibi() {
    let config = parse(&["-g"]);
    assert!(config.gibi);
}

#[test]
fn flag_h_human() {
    let config = parse(&["-h"]);
    assert!(config.human);
}

#[test]
fn long_human() {
    let config = parse(&["--human"]);
    assert!(config.human);
}

#[test]
fn flag_si() {
    let config = parse(&["--si"]);
    assert!(config.si);
}

#[test]
fn flag_t_total() {
    let config = parse(&["-t"]);
    assert!(config.total);
}

#[test]
fn long_total() {
    let config = parse(&["--total"]);
    assert!(config.total);
}

#[test]
fn flag_w_wide() {
    let config = parse(&["-w"]);
    assert!(config.wide);
}

#[test]
fn long_wide() {
    let config = parse(&["--wide"]);
    assert!(config.wide);
}

#[test]
fn help_is_err() {
    assert!(FreeConfig::try_parse_from(["free", "--help"]).is_err());
}

#[test]
fn version_is_err() {
    assert!(FreeConfig::try_parse_from(["free", "--version"]).is_err());
}
