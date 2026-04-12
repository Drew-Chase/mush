use free::cli::FreeConfig;

fn parse(args: &[&str]) -> FreeConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    FreeConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.bytes);
    assert!(config.kibi);
    assert!(!config.mebi);
    assert!(!config.gibi);
    assert!(!config.human);
    assert!(!config.si);
    assert!(!config.total);
    assert!(!config.wide);
}

#[test]
fn flag_b_bytes() {
    let config = parse(&["-b"]);
    assert!(config.bytes);
    assert!(!config.kibi);
}

#[test]
fn flag_k_kibi() {
    let config = parse(&["-k"]);
    assert!(config.kibi);
}

#[test]
fn flag_m_mebi() {
    let config = parse(&["-m"]);
    assert!(config.mebi);
    assert!(!config.kibi);
}

#[test]
fn flag_g_gibi() {
    let config = parse(&["-g"]);
    assert!(config.gibi);
    assert!(!config.kibi);
}

#[test]
fn flag_h_human() {
    let config = parse(&["-h"]);
    assert!(config.human);
    assert!(!config.kibi);
}

#[test]
fn long_human() {
    let config = parse(&["--human"]);
    assert!(config.human);
}

#[test]
fn flag_si() {
    let config = parse(&["--si"]);
    assert!(config.si);
}

#[test]
fn flag_t_total() {
    let config = parse(&["-t"]);
    assert!(config.total);
}

#[test]
fn long_total() {
    let config = parse(&["--total"]);
    assert!(config.total);
}

#[test]
fn flag_w_wide() {
    let config = parse(&["-w"]);
    assert!(config.wide);
}

#[test]
fn long_wide() {
    let config = parse(&["--wide"]);
    assert!(config.wide);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(FreeConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(FreeConfig::from_args(&owned).is_none());
}

#[test]
fn last_unit_wins() {
    let config = parse(&["-b", "-m", "-g"]);
    assert!(config.gibi);
    assert!(!config.bytes);
    assert!(!config.mebi);
    assert!(!config.kibi);
}
