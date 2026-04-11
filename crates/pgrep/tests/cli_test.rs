use pgrep::cli::PgrepConfig;

fn parse(args: &[&str]) -> PgrepConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    PgrepConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config_with_pattern() {
    let config = parse(&["myproc"]);
    assert!(!config.list_name);
    assert!(!config.list_full);
    assert!(!config.count);
    assert_eq!(config.delimiter, "\n");
    assert!(!config.full);
    assert!(!config.ignore_case);
    assert!(!config.exact);
    assert!(config.user_filter.is_none());
    assert!(!config.newest);
    assert!(!config.oldest);
    assert_eq!(config.pattern, "myproc");
}

#[test]
fn flag_l_list_name() {
    let config = parse(&["-l", "proc"]);
    assert!(config.list_name);
}

#[test]
fn long_list_name() {
    let config = parse(&["--list-name", "proc"]);
    assert!(config.list_name);
}

#[test]
fn flag_a_list_full() {
    let config = parse(&["-a", "proc"]);
    assert!(config.list_full);
}

#[test]
fn long_list_full() {
    let config = parse(&["--list-full", "proc"]);
    assert!(config.list_full);
}

#[test]
fn flag_c_count() {
    let config = parse(&["-c", "proc"]);
    assert!(config.count);
}

#[test]
fn long_count() {
    let config = parse(&["--count", "proc"]);
    assert!(config.count);
}

#[test]
fn flag_d_delimiter() {
    let config = parse(&["-d", ",", "proc"]);
    assert_eq!(config.delimiter, ",");
}

#[test]
fn long_delimiter() {
    let config = parse(&["--delimiter", "|", "proc"]);
    assert_eq!(config.delimiter, "|");
}

#[test]
fn flag_f_full() {
    let config = parse(&["-f", "proc"]);
    assert!(config.full);
}

#[test]
fn long_full() {
    let config = parse(&["--full", "proc"]);
    assert!(config.full);
}

#[test]
fn flag_i_ignore_case() {
    let config = parse(&["-i", "proc"]);
    assert!(config.ignore_case);
}

#[test]
fn long_ignore_case() {
    let config = parse(&["--ignore-case", "proc"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_x_exact() {
    let config = parse(&["-x", "proc"]);
    assert!(config.exact);
}

#[test]
fn long_exact() {
    let config = parse(&["--exact", "proc"]);
    assert!(config.exact);
}

#[test]
fn flag_u_user_filter() {
    let config = parse(&["-u", "root", "proc"]);
    assert_eq!(config.user_filter, Some("root".to_string()));
}

#[test]
fn long_euid() {
    let config = parse(&["--euid", "alice", "proc"]);
    assert_eq!(config.user_filter, Some("alice".to_string()));
}

#[test]
fn flag_n_newest() {
    let config = parse(&["-n", "proc"]);
    assert!(config.newest);
}

#[test]
fn long_newest() {
    let config = parse(&["--newest", "proc"]);
    assert!(config.newest);
}

#[test]
fn flag_o_oldest() {
    let config = parse(&["-o", "proc"]);
    assert!(config.oldest);
}

#[test]
fn long_oldest() {
    let config = parse(&["--oldest", "proc"]);
    assert!(config.oldest);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(PgrepConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(PgrepConfig::from_args(&owned).is_none());
}

#[test]
fn multiple_flags() {
    let config = parse(&["-l", "-i", "-f", "proc"]);
    assert!(config.list_name);
    assert!(config.ignore_case);
    assert!(config.full);
    assert_eq!(config.pattern, "proc");
}
