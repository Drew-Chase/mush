use kill::cli::KillConfig;

fn parse(args: &[&str]) -> KillConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    KillConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_signal_is_term() {
    let config = parse(&["1234"]);
    assert_eq!(config.signal, 15);
    assert_eq!(config.pids, vec![1234]);
}

#[test]
fn flag_9() {
    let config = parse(&["-9", "1234"]);
    assert_eq!(config.signal, 9);
    assert_eq!(config.pids, vec![1234]);
}

#[test]
fn flag_kill() {
    let config = parse(&["-KILL", "1234"]);
    assert_eq!(config.signal, 9);
}

#[test]
fn flag_sigterm() {
    let config = parse(&["-SIGTERM", "1234"]);
    assert_eq!(config.signal, 15);
}

#[test]
fn flag_term() {
    let config = parse(&["-TERM", "1234"]);
    assert_eq!(config.signal, 15);
}

#[test]
fn flag_s_kill() {
    let config = parse(&["-s", "KILL", "1234"]);
    assert_eq!(config.signal, 9);
}

#[test]
fn flag_s_numeric() {
    let config = parse(&["-s", "2", "1234"]);
    assert_eq!(config.signal, 2);
}

#[test]
fn flag_s_hup() {
    let config = parse(&["-s", "HUP", "1234"]);
    assert_eq!(config.signal, 1);
}

#[test]
fn flag_l() {
    let config = parse(&["-l"]);
    assert!(config.list);
}

#[test]
fn long_list() {
    let config = parse(&["--list"]);
    assert!(config.list);
}

#[test]
fn flag_big_l() {
    let config = parse(&["-L"]);
    assert!(config.table);
}

#[test]
fn long_table() {
    let config = parse(&["--table"]);
    assert!(config.table);
}

#[test]
fn multiple_pids() {
    let config = parse(&["1234", "5678", "9012"]);
    assert_eq!(config.pids, vec![1234, 5678, 9012]);
}

#[test]
fn signal_and_multiple_pids() {
    let config = parse(&["-9", "1234", "5678"]);
    assert_eq!(config.signal, 9);
    assert_eq!(config.pids, vec![1234, 5678]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(KillConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(KillConfig::from_args(&owned).is_none());
}

#[test]
fn flag_int() {
    let config = parse(&["-INT", "42"]);
    assert_eq!(config.signal, 2);
}

#[test]
fn flag_sigint() {
    let config = parse(&["-SIGINT", "42"]);
    assert_eq!(config.signal, 2);
}
