use tree::cli::TreeConfig;

fn parse(args: &[&str]) -> TreeConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    TreeConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.all);
    assert!(!config.dirs_only);
    assert!(!config.full_path);
    assert!(!config.show_size);
    assert!(!config.human_readable);
    assert!(!config.show_date);
    assert!(!config.dirs_first);
    assert!(!config.no_report);
    assert!(!config.color);
    assert!(!config.json);
    assert!(config.level.is_none());
    assert!(config.exclude.is_none());
    assert!(config.pattern.is_none());
    assert!(config.paths.is_empty());
}

#[test]
fn flag_a() {
    let config = parse(&["-a"]);
    assert!(config.all);
}

#[test]
fn flag_d() {
    let config = parse(&["-d"]);
    assert!(config.dirs_only);
}

#[test]
fn flag_f() {
    let config = parse(&["-f"]);
    assert!(config.full_path);
}

#[test]
fn flag_l_separate() {
    let config = parse(&["-L", "3"]);
    assert_eq!(config.level, Some(3));
}

#[test]
fn flag_l_attached() {
    let config = parse(&["-L2"]);
    assert_eq!(config.level, Some(2));
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.show_size);
}

#[test]
fn flag_h() {
    let config = parse(&["-h"]);
    assert!(config.human_readable);
}

#[test]
fn flag_big_d() {
    let config = parse(&["-D"]);
    assert!(config.show_date);
}

#[test]
fn flag_big_c() {
    let config = parse(&["-C"]);
    assert!(config.color);
}

#[test]
fn flag_big_j() {
    let config = parse(&["-J"]);
    assert!(config.json);
}

#[test]
fn long_dirsfirst() {
    let config = parse(&["--dirsfirst"]);
    assert!(config.dirs_first);
}

#[test]
fn long_noreport() {
    let config = parse(&["--noreport"]);
    assert!(config.no_report);
}

#[test]
fn long_all() {
    let config = parse(&["--all"]);
    assert!(config.all);
}

#[test]
fn long_dirs_only() {
    let config = parse(&["--dirs-only"]);
    assert!(config.dirs_only);
}

#[test]
fn long_level() {
    let config = parse(&["--level", "5"]);
    assert_eq!(config.level, Some(5));
}

#[test]
fn exclude_flag() {
    let config = parse(&["-I", "*.log"]);
    assert_eq!(config.exclude, Some("*.log".to_string()));
}

#[test]
fn pattern_flag() {
    let config = parse(&["-P", "*.rs"]);
    assert_eq!(config.pattern, Some("*.rs".to_string()));
}

#[test]
fn long_exclude() {
    let config = parse(&["--exclude", "node_modules"]);
    assert_eq!(config.exclude, Some("node_modules".to_string()));
}

#[test]
fn long_pattern() {
    let config = parse(&["--pattern", "*.txt"]);
    assert_eq!(config.pattern, Some("*.txt".to_string()));
}

#[test]
fn combined_flags() {
    let config = parse(&["-adCJ"]);
    assert!(config.all);
    assert!(config.dirs_only);
    assert!(config.color);
    assert!(config.json);
}

#[test]
fn paths_collected() {
    let config = parse(&["-a", "src", "tests"]);
    assert!(config.all);
    assert_eq!(config.paths, vec!["src", "tests"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-a"]);
    assert!(!config.all);
    assert_eq!(config.paths, vec!["-a"]);
}
