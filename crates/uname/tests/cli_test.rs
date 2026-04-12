use clap::Parser;

use uname::cli::UnameConfig;

fn parse(args: &[&str]) -> UnameConfig {
    let mut full = vec!["uname"];
    full.extend_from_slice(args);
    let mut config = UnameConfig::parse_from(full);
    config.resolve();
    config
}

#[test]
fn default_kernel_name_only() {
    let config = parse(&[]);
    assert!(config.kernel_name);
    assert!(!config.nodename);
    assert!(!config.machine);
    assert!(!config.operating_system);
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.kernel_name);
    assert!(!config.all);
}

#[test]
fn flag_m() {
    let config = parse(&["-m"]);
    assert!(config.machine);
}

#[test]
fn flag_o() {
    let config = parse(&["-o"]);
    assert!(config.operating_system);
}

#[test]
fn flag_a_enables_all() {
    let config = parse(&["-a"]);
    assert!(config.all);
    assert!(config.kernel_name);
    assert!(config.nodename);
    assert!(config.kernel_release);
    assert!(config.kernel_version);
    assert!(config.machine);
    assert!(config.processor);
    assert!(config.operating_system);
}

#[test]
fn long_all() {
    let config = parse(&["--all"]);
    assert!(config.all);
    assert!(config.kernel_name);
    assert!(config.machine);
}

#[test]
fn combined_flags_sm() {
    let config = parse(&["-sm"]);
    assert!(config.kernel_name);
    assert!(config.machine);
    assert!(!config.nodename);
}

#[test]
fn combined_flags_nro() {
    let config = parse(&["-nro"]);
    assert!(config.nodename);
    assert!(config.kernel_release);
    assert!(config.operating_system);
}

#[test]
fn long_kernel_name() {
    let config = parse(&["--kernel-name"]);
    assert!(config.kernel_name);
}

#[test]
fn long_machine() {
    let config = parse(&["--machine"]);
    assert!(config.machine);
}

#[test]
fn help_returns_err() {
    assert!(UnameConfig::try_parse_from(["uname", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(UnameConfig::try_parse_from(["uname", "--version"]).is_err());
}
