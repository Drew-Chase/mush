use clap::Parser;

use hostname::cli::HostnameConfig;

fn parse(args: &[&str]) -> HostnameConfig {
    let mut full = vec!["hostname"];
    full.extend_from_slice(args);
    HostnameConfig::parse_from(full)
}

#[test]
fn default_no_flags() {
    let config = parse(&[]);
    assert!(!config.short);
    assert!(!config.fqdn);
}

#[test]
fn flag_short() {
    let config = parse(&["-s"]);
    assert!(config.short);
    assert!(!config.fqdn);
}

#[test]
fn flag_fqdn() {
    let config = parse(&["-f"]);
    assert!(config.fqdn);
    assert!(!config.short);
}

#[test]
fn long_short() {
    let config = parse(&["--short"]);
    assert!(config.short);
}

#[test]
fn long_fqdn() {
    let config = parse(&["--fqdn"]);
    assert!(config.fqdn);
}

#[test]
fn long_long() {
    let config = parse(&["--long"]);
    assert!(config.fqdn);
}

#[test]
fn help_returns_err() {
    let result = HostnameConfig::try_parse_from(["hostname", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = HostnameConfig::try_parse_from(["hostname", "--version"]);
    assert!(result.is_err());
}
