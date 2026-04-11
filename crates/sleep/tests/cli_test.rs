use sleep::cli::SleepConfig;

fn parse(args: &[&str]) -> SleepConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SleepConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn bare_number() {
    let config = parse(&["5"]);
    assert_eq!(config.seconds, 5.0);
}

#[test]
fn fractional_number() {
    let config = parse(&["1.5"]);
    assert_eq!(config.seconds, 1.5);
}

#[test]
fn minutes_suffix() {
    let config = parse(&["1m"]);
    assert_eq!(config.seconds, 60.0);
}

#[test]
fn hours_suffix() {
    let config = parse(&["1h"]);
    assert_eq!(config.seconds, 3600.0);
}

#[test]
fn days_suffix() {
    let config = parse(&["1d"]);
    assert_eq!(config.seconds, 86400.0);
}

#[test]
fn multiple_args_summed() {
    let config = parse(&["1m", "30s"]);
    assert_eq!(config.seconds, 90.0);
}

#[test]
fn fractional_with_suffix() {
    let config = parse(&["0.5s"]);
    assert_eq!(config.seconds, 0.5);
}

#[test]
fn help_returns_none() {
    let owned: Vec<String> = vec!["--help".to_string()];
    assert!(SleepConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned: Vec<String> = vec!["--version".to_string()];
    assert!(SleepConfig::from_args(&owned).is_none());
}
