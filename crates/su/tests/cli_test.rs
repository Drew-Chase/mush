use su::cli::SuConfig;

fn parse(args: &[&str]) -> SuConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SuConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(config.command.is_none());
    assert!(!config.login);
    assert!(config.shell.is_none());
    assert_eq!(config.user, "root");
}

#[test]
fn flag_c_command() {
    let config = parse(&["-c", "whoami"]);
    assert_eq!(config.command, Some("whoami".to_string()));
}

#[test]
fn long_command() {
    let config = parse(&["--command", "id"]);
    assert_eq!(config.command, Some("id".to_string()));
}

#[test]
fn flag_l_login() {
    let config = parse(&["-l"]);
    assert!(config.login);
}

#[test]
fn long_login() {
    let config = parse(&["--login"]);
    assert!(config.login);
}

#[test]
fn dash_login() {
    let config = parse(&["-"]);
    assert!(config.login);
}

#[test]
fn flag_s_shell() {
    let config = parse(&["-s", "/bin/zsh"]);
    assert_eq!(config.shell, Some("/bin/zsh".to_string()));
}

#[test]
fn long_shell() {
    let config = parse(&["--shell", "/bin/bash"]);
    assert_eq!(config.shell, Some("/bin/bash".to_string()));
}

#[test]
fn positional_user() {
    let config = parse(&["admin"]);
    assert_eq!(config.user, "admin");
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(SuConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(SuConfig::from_args(&owned).is_none());
}

#[test]
fn all_options() {
    let config = parse(&["-l", "-s", "/bin/bash", "-c", "echo hi", "bob"]);
    assert!(config.login);
    assert_eq!(config.shell, Some("/bin/bash".to_string()));
    assert_eq!(config.command, Some("echo hi".to_string()));
    assert_eq!(config.user, "bob");
}
