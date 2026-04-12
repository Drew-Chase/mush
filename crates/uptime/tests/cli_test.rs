use uptime::cli::UptimeConfig;

fn parse(args: &[&str]) -> UptimeConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    UptimeConfig::from_args(&owned).expect("should not be --help/--version")
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
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(UptimeConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(UptimeConfig::from_args(&owned).is_none());
}

#[test]
fn both_flags() {
    let config = parse(&["-p", "-s"]);
    assert!(config.pretty);
    assert!(config.since);
}
