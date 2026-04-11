use sudo::cli::SudoConfig;

fn parse(args: &[&str]) -> SudoConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SudoConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(config.user.is_none());
    assert!(!config.login);
    assert!(!config.shell);
    assert!(!config.preserve_env);
    assert!(config.command.is_empty());
}

#[test]
fn simple_command() {
    let config = parse(&["ls", "-la"]);
    assert_eq!(config.command, vec!["ls", "-la"]);
}

#[test]
fn flag_u_user() {
    let config = parse(&["-u", "root", "whoami"]);
    assert_eq!(config.user, Some("root".to_string()));
    assert_eq!(config.command, vec!["whoami"]);
}

#[test]
fn long_user() {
    let config = parse(&["--user", "admin", "id"]);
    assert_eq!(config.user, Some("admin".to_string()));
    assert_eq!(config.command, vec!["id"]);
}

#[test]
fn flag_i_login() {
    let config = parse(&["-i"]);
    assert!(config.login);
}

#[test]
fn long_login() {
    let config = parse(&["--login"]);
    assert!(config.login);
}

#[test]
fn flag_s_shell() {
    let config = parse(&["-s"]);
    assert!(config.shell);
}

#[test]
fn long_shell() {
    let config = parse(&["--shell"]);
    assert!(config.shell);
}

#[test]
fn flag_e_preserve_env() {
    let config = parse(&["-E", "env"]);
    assert!(config.preserve_env);
    assert_eq!(config.command, vec!["env"]);
}

#[test]
fn long_preserve_env() {
    let config = parse(&["--preserve-env", "printenv"]);
    assert!(config.preserve_env);
    assert_eq!(config.command, vec!["printenv"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(SudoConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(SudoConfig::from_args(&owned).is_none());
}

#[test]
fn command_args_after_command() {
    let config = parse(&["-u", "bob", "grep", "-r", "pattern", "/etc"]);
    assert_eq!(config.user, Some("bob".to_string()));
    assert_eq!(config.command, vec!["grep", "-r", "pattern", "/etc"]);
}

#[test]
fn all_options() {
    let config = parse(&["-u", "root", "-i", "-s", "-E", "bash"]);
    assert_eq!(config.user, Some("root".to_string()));
    assert!(config.login);
    assert!(config.shell);
    assert!(config.preserve_env);
    assert_eq!(config.command, vec!["bash"]);
}
