use ps::cli::PsConfig;

fn parse(args: &[&str]) -> PsConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    PsConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.all);
    assert!(!config.full);
    assert!(!config.long_format);
    assert!(!config.no_headers);
    assert!(!config.show_threads);
    assert!(config.user_filter.is_none());
    assert!(config.pid_filter.is_empty());
    assert!(config.command_filter.is_none());
    assert!(config.sort_key.is_none());
    assert!(config.format_spec.is_none());
}

#[test]
fn flag_e_selects_all() {
    let config = parse(&["-e"]);
    assert!(config.all);
}

#[test]
fn flag_big_a_selects_all() {
    let config = parse(&["-A"]);
    assert!(config.all);
}

#[test]
fn flag_f_full_format() {
    let config = parse(&["-f"]);
    assert!(config.full);
}

#[test]
fn long_full() {
    let config = parse(&["--full"]);
    assert!(config.full);
}

#[test]
fn flag_l_long_format() {
    let config = parse(&["-l"]);
    assert!(config.long_format);
}

#[test]
fn long_long() {
    let config = parse(&["--long"]);
    assert!(config.long_format);
}

#[test]
fn flag_u_user_filter() {
    let config = parse(&["-u", "alice"]);
    assert_eq!(config.user_filter, Some("alice".to_string()));
}

#[test]
fn long_user_filter() {
    let config = parse(&["--user", "bob"]);
    assert_eq!(config.user_filter, Some("bob".to_string()));
}

#[test]
fn flag_p_single_pid() {
    let config = parse(&["-p", "1234"]);
    assert_eq!(config.pid_filter, vec![1234]);
}

#[test]
fn flag_p_comma_separated_pids() {
    let config = parse(&["-p", "1,2,3"]);
    assert_eq!(config.pid_filter, vec![1, 2, 3]);
}

#[test]
fn long_pid() {
    let config = parse(&["--pid", "42"]);
    assert_eq!(config.pid_filter, vec![42]);
}

#[test]
fn flag_big_c_command_filter() {
    let config = parse(&["-C", "bash"]);
    assert_eq!(config.command_filter, Some("bash".to_string()));
}

#[test]
fn long_command_filter() {
    let config = parse(&["--command", "nginx"]);
    assert_eq!(config.command_filter, Some("nginx".to_string()));
}

#[test]
fn sort_key() {
    let config = parse(&["--sort", "cpu"]);
    assert_eq!(config.sort_key, Some("cpu".to_string()));
}

#[test]
fn flag_o_format_spec() {
    let config = parse(&["-o", "pid,name,cpu"]);
    assert_eq!(config.format_spec, Some("pid,name,cpu".to_string()));
}

#[test]
fn long_format_spec() {
    let config = parse(&["--format", "pid,mem"]);
    assert_eq!(config.format_spec, Some("pid,mem".to_string()));
}

#[test]
fn no_headers() {
    let config = parse(&["--no-headers"]);
    assert!(config.no_headers);
}

#[test]
fn flag_a_show_threads() {
    let config = parse(&["-a"]);
    assert!(config.show_threads);
}

#[test]
fn flag_x_show_threads() {
    let config = parse(&["-x"]);
    assert!(config.show_threads);
}

#[test]
fn combined_ef() {
    let config = parse(&["-ef"]);
    assert!(config.all);
    assert!(config.full);
}

#[test]
fn combined_aux() {
    let config = parse(&["-aux"]);
    assert!(config.show_threads);
}

#[test]
fn combined_flags_with_options() {
    let config = parse(&["-ef", "--sort", "mem", "--no-headers"]);
    assert!(config.all);
    assert!(config.full);
    assert_eq!(config.sort_key, Some("mem".to_string()));
    assert!(config.no_headers);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(PsConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(PsConfig::from_args(&owned).is_none());
}

#[test]
fn multiple_filters() {
    let config = parse(&["-u", "root", "-C", "sshd", "--sort", "pid"]);
    assert_eq!(config.user_filter, Some("root".to_string()));
    assert_eq!(config.command_filter, Some("sshd".to_string()));
    assert_eq!(config.sort_key, Some("pid".to_string()));
}
