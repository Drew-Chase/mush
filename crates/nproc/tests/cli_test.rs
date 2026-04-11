use nproc::cli::NprocConfig;
use nproc::ops::nproc;

fn parse(args: &[&str]) -> NprocConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    NprocConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_returns_at_least_one() {
    let config = parse(&[]);
    assert!(nproc(&config) >= 1);
}

#[test]
fn all_flag() {
    let config = parse(&["--all"]);
    assert!(config.all);
    assert!(nproc(&config) >= 1);
}

#[test]
fn ignore_equals() {
    let config = parse(&["--ignore=2"]);
    assert_eq!(config.ignore, 2);
}

#[test]
fn ignore_separate() {
    let config = parse(&["--ignore", "3"]);
    assert_eq!(config.ignore, 3);
}

#[test]
fn ignore_never_below_one() {
    let config = NprocConfig { all: false, ignore: 99999 };
    assert_eq!(nproc(&config), 1);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(NprocConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(NprocConfig::from_args(&owned).is_none());
}
