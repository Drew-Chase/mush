use clap::Parser;

use pkill::cli::PkillConfig;

fn parse(args: &[&str]) -> PkillConfig {
    let mut full = vec!["pkill"];
    full.extend_from_slice(args);
    PkillConfig::parse_from(full)
}

#[test]
fn default_config_with_pattern() {
    let config = parse(&["myproc"]);
    assert_eq!(config.signal, "TERM");
    assert!(!config.full);
    assert!(!config.ignore_case);
    assert!(!config.exact);
    assert!(config.user_filter.is_none());
    assert!(!config.newest);
    assert!(!config.oldest);
    assert_eq!(config.pattern, "myproc");
}

#[test]
fn flag_s_signal() {
    let config = parse(&["-s", "KILL", "proc"]);
    assert_eq!(config.signal, "KILL");
}

#[test]
fn long_signal() {
    let config = parse(&["--signal", "9", "proc"]);
    assert_eq!(config.signal, "9");
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
fn help_returns_err() {
    assert!(PkillConfig::try_parse_from(["pkill", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(PkillConfig::try_parse_from(["pkill", "--version"]).is_err());
}

#[test]
fn multiple_flags() {
    let config = parse(&["-s", "HUP", "-i", "-f", "proc"]);
    assert_eq!(config.signal, "HUP");
    assert!(config.ignore_case);
    assert!(config.full);
    assert_eq!(config.pattern, "proc");
}
