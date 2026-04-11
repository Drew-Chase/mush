use hostname::cli::HostnameConfig;

fn parse(args: &[&str]) -> HostnameConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    HostnameConfig::from_args(&owned).expect("should not be --help/--version")
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
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(HostnameConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(HostnameConfig::from_args(&owned).is_none());
}
