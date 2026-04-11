use whoami::cli::WhoamiConfig;

#[test]
fn no_args() {
    let config = WhoamiConfig::from_args(&[]);
    assert!(config.is_some());
}

#[test]
fn help_returns_none() {
    let args = vec!["--help".to_string()];
    assert!(WhoamiConfig::from_args(&args).is_none());
}

#[test]
fn version_returns_none() {
    let args = vec!["--version".to_string()];
    assert!(WhoamiConfig::from_args(&args).is_none());
}
