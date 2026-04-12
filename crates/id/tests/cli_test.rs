use clap::Parser;

use id::cli::IdConfig;

fn parse(args: &[&str]) -> IdConfig {
    let mut full = vec!["id"];
    full.extend_from_slice(args);
    IdConfig::parse_from(full)
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.user_only);
    assert!(!config.group_only);
    assert!(!config.groups_only);
    assert!(!config.name);
    assert!(!config.real);
    assert!(config.target_user.is_none());
}

#[test]
fn flag_u_user() {
    let config = parse(&["-u"]);
    assert!(config.user_only);
}

#[test]
fn long_user() {
    let config = parse(&["--user"]);
    assert!(config.user_only);
}

#[test]
fn flag_g_group() {
    let config = parse(&["-g"]);
    assert!(config.group_only);
}

#[test]
fn long_group() {
    let config = parse(&["--group"]);
    assert!(config.group_only);
}

#[test]
fn flag_big_g_groups() {
    let config = parse(&["-G"]);
    assert!(config.groups_only);
}

#[test]
fn long_groups() {
    let config = parse(&["--groups"]);
    assert!(config.groups_only);
}

#[test]
fn flag_n_name() {
    let config = parse(&["-n"]);
    assert!(config.name);
}

#[test]
fn long_name() {
    let config = parse(&["--name"]);
    assert!(config.name);
}

#[test]
fn flag_r_real() {
    let config = parse(&["-r"]);
    assert!(config.real);
}

#[test]
fn long_real() {
    let config = parse(&["--real"]);
    assert!(config.real);
}

#[test]
fn positional_user() {
    let config = parse(&["root"]);
    assert_eq!(config.target_user, Some("root".to_string()));
}

#[test]
fn help_is_err() {
    assert!(IdConfig::try_parse_from(["id", "--help"]).is_err());
}

#[test]
fn version_is_err() {
    assert!(IdConfig::try_parse_from(["id", "--version"]).is_err());
}

#[test]
fn combined_flags() {
    let config = parse(&["-u", "-n", "-r"]);
    assert!(config.user_only);
    assert!(config.name);
    assert!(config.real);
}
