use df::cli::DfConfig;

fn parse(args: &[&str]) -> DfConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    DfConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.human_readable);
    assert!(!config.si);
    assert!(!config.print_type);
    assert!(config.type_filter.is_none());
    assert!(!config.all);
    assert!(!config.total);
    assert!(config.files.is_empty());
}

#[test]
fn flag_h_human_readable() {
    let config = parse(&["-h"]);
    assert!(config.human_readable);
}

#[test]
fn long_human_readable() {
    let config = parse(&["--human-readable"]);
    assert!(config.human_readable);
}

#[test]
fn flag_big_h_si() {
    let config = parse(&["-H"]);
    assert!(config.si);
}

#[test]
fn long_si() {
    let config = parse(&["--si"]);
    assert!(config.si);
}

#[test]
fn flag_big_t_print_type() {
    let config = parse(&["-T"]);
    assert!(config.print_type);
}

#[test]
fn long_print_type() {
    let config = parse(&["--print-type"]);
    assert!(config.print_type);
}

#[test]
fn flag_t_type_filter() {
    let config = parse(&["-t", "ext4"]);
    assert_eq!(config.type_filter, Some("ext4".to_string()));
}

#[test]
fn long_type_filter() {
    let config = parse(&["--type", "ntfs"]);
    assert_eq!(config.type_filter, Some("ntfs".to_string()));
}

#[test]
fn flag_a_all() {
    let config = parse(&["-a"]);
    assert!(config.all);
}

#[test]
fn long_all() {
    let config = parse(&["--all"]);
    assert!(config.all);
}

#[test]
fn flag_total() {
    let config = parse(&["--total"]);
    assert!(config.total);
}

#[test]
fn positional_files() {
    let config = parse(&["/home", "/tmp"]);
    assert_eq!(config.files, vec!["/home", "/tmp"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(DfConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(DfConfig::from_args(&owned).is_none());
}

#[test]
fn multiple_flags() {
    let config = parse(&["-h", "-T", "--total", "-a"]);
    assert!(config.human_readable);
    assert!(config.print_type);
    assert!(config.total);
    assert!(config.all);
}
