use clap::Parser;

use uptime::cli::UptimeConfig;

fn parse(args: &[&str]) -> UptimeConfig {
    let mut full = vec!["uptime"];
    full.extend_from_slice(args);
    UptimeConfig::parse_from(full)
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.pretty);
    assert!(!config.since);
}

#[test]
fn flag_p_pretty() {
    let config = parse(&["-p"]);
    assert!(config.pretty);
}

#[test]
fn long_pretty() {
    let config = parse(&["--pretty"]);
    assert!(config.pretty);
}

#[test]
fn flag_s_since() {
    let config = parse(&["-s"]);
    assert!(config.since);
}

#[test]
fn long_since() {
    let config = parse(&["--since"]);
    assert!(config.since);
}

#[test]
fn both_flags() {
    let config = parse(&["-p", "-s"]);
    assert!(config.pretty);
    assert!(config.since);
}
