use nohup::cli::NohupConfig;

fn parse(args: &[&str]) -> NohupConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    NohupConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn empty_args() {
    let config = parse(&[]);
    assert!(config.command.is_empty());
}

#[test]
fn single_command() {
    let config = parse(&["sleep"]);
    assert_eq!(config.command, vec!["sleep"]);
}

#[test]
fn command_with_args() {
    let config = parse(&["sleep", "10"]);
    assert_eq!(config.command, vec!["sleep", "10"]);
}

#[test]
fn complex_command() {
    let config = parse(&["bash", "-c", "echo hello"]);
    assert_eq!(config.command, vec!["bash", "-c", "echo hello"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(NohupConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(NohupConfig::from_args(&owned).is_none());
}

#[test]
fn command_starting_with_dash() {
    // If a command starts with something unknown, it's treated as the command
    let config = parse(&["my-program", "--flag"]);
    assert_eq!(config.command, vec!["my-program", "--flag"]);
}
