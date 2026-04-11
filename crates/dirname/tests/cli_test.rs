use dirname::cli::DirnameConfig;
use dirname::ops::dirname;

fn parse(args: &[&str]) -> DirnameConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    DirnameConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn basic_path() {
    assert_eq!(dirname("/usr/bin/sort"), "/usr/bin");
}

#[test]
fn file_in_current_dir() {
    assert_eq!(dirname("hello"), ".");
}

#[test]
fn root_path() {
    assert_eq!(dirname("/"), "/");
}

#[test]
fn trailing_slash() {
    assert_eq!(dirname("/usr/bin/"), "/usr");
}

#[test]
fn nested_path() {
    assert_eq!(dirname("/a/b/c/d"), "/a/b/c");
}

#[test]
fn empty_string() {
    assert_eq!(dirname(""), ".");
}

#[test]
fn single_component() {
    assert_eq!(dirname("filename.txt"), ".");
}

#[test]
fn parse_basic() {
    let config = parse(&["/usr/bin/sort"]);
    assert_eq!(config.names, vec!["/usr/bin/sort"]);
    assert!(!config.zero);
}

#[test]
fn parse_zero_flag() {
    let config = parse(&["-z", "/usr/bin/sort"]);
    assert!(config.zero);
    assert_eq!(config.names, vec!["/usr/bin/sort"]);
}

#[test]
fn parse_long_zero() {
    let config = parse(&["--zero", "/usr/bin/sort"]);
    assert!(config.zero);
}

#[test]
fn parse_multiple_names() {
    let config = parse(&["/usr/bin/sort", "/usr/lib/foo"]);
    assert_eq!(config.names, vec!["/usr/bin/sort", "/usr/lib/foo"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(DirnameConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(DirnameConfig::from_args(&owned).is_none());
}
