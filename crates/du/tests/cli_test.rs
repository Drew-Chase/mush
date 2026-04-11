use du::cli::DuConfig;

fn parse(args: &[&str]) -> DuConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    DuConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.human_readable);
    assert!(!config.summarize);
    assert!(!config.all);
    assert!(!config.total);
    assert!(!config.bytes);
    assert!(!config.kilobytes);
    assert!(!config.megabytes);
    assert!(config.max_depth.is_none());
    assert!(config.files.is_empty());
}

#[test]
fn flag_h() {
    let config = parse(&["-h"]);
    assert!(config.human_readable);
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.summarize);
}

#[test]
fn flag_a() {
    let config = parse(&["-a"]);
    assert!(config.all);
}

#[test]
fn flag_c() {
    let config = parse(&["-c"]);
    assert!(config.total);
}

#[test]
fn flag_b() {
    let config = parse(&["-b"]);
    assert!(config.bytes);
    assert!(config.apparent_size);
}

#[test]
fn flag_k() {
    let config = parse(&["-k"]);
    assert!(config.kilobytes);
}

#[test]
fn flag_m() {
    let config = parse(&["-m"]);
    assert!(config.megabytes);
}

#[test]
fn flag_d_separate() {
    let config = parse(&["-d", "3"]);
    assert_eq!(config.max_depth, Some(3));
}

#[test]
fn flag_d_attached() {
    let config = parse(&["-d2"]);
    assert_eq!(config.max_depth, Some(2));
}

#[test]
fn long_max_depth() {
    let config = parse(&["--max-depth", "5"]);
    assert_eq!(config.max_depth, Some(5));
}

#[test]
fn long_max_depth_eq() {
    let config = parse(&["--max-depth=4"]);
    assert_eq!(config.max_depth, Some(4));
}

#[test]
fn long_flags() {
    let config = parse(&["--human-readable", "--summarize", "--all", "--total"]);
    assert!(config.human_readable);
    assert!(config.summarize);
    assert!(config.all);
    assert!(config.total);
}

#[test]
fn long_apparent_size() {
    let config = parse(&["--apparent-size"]);
    assert!(config.apparent_size);
}

#[test]
fn long_bytes() {
    let config = parse(&["--bytes"]);
    assert!(config.bytes);
    assert!(config.apparent_size);
}

#[test]
fn combined_flags() {
    let config = parse(&["-shac"]);
    assert!(config.summarize);
    assert!(config.human_readable);
    assert!(config.all);
    assert!(config.total);
}

#[test]
fn files_collected() {
    let config = parse(&["-s", "foo", "bar"]);
    assert!(config.summarize);
    assert_eq!(config.files, vec!["foo", "bar"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-s"]);
    assert!(!config.summarize);
    assert_eq!(config.files, vec!["-s"]);
}
