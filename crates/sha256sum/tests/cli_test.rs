use sha256sum::cli::Sha256sumConfig;

fn parse(args: &[&str]) -> Sha256sumConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    Sha256sumConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert_eq!(config.algorithm, "sha256");
    assert!(!config.binary);
    assert!(!config.check);
    assert!(!config.tag);
    assert!(!config.quiet);
    assert!(!config.status);
    assert!(!config.warn);
    assert!(config.files.is_empty());
}

#[test]
fn flag_b() {
    let config = parse(&["-b"]);
    assert!(config.binary);
}

#[test]
fn flag_c() {
    let config = parse(&["-c"]);
    assert!(config.check);
}

#[test]
fn flag_q() {
    let config = parse(&["-q"]);
    assert!(config.quiet);
}

#[test]
fn flag_w() {
    let config = parse(&["-w"]);
    assert!(config.warn);
}

#[test]
fn long_flags() {
    let config = parse(&["--binary", "--check", "--tag", "--quiet", "--status", "--warn"]);
    assert!(config.binary);
    assert!(config.check);
    assert!(config.tag);
    assert!(config.quiet);
    assert!(config.status);
    assert!(config.warn);
}

#[test]
fn algorithm_flag() {
    let config = parse(&["-a", "sha256"]);
    assert_eq!(config.algorithm, "sha256");
}

#[test]
fn algorithm_long_flag() {
    let config = parse(&["--algorithm", "sha256"]);
    assert_eq!(config.algorithm, "sha256");
}

#[test]
fn combined_short_flags() {
    let config = parse(&["-bcqw"]);
    assert!(config.binary);
    assert!(config.check);
    assert!(config.quiet);
    assert!(config.warn);
}

#[test]
fn files_collected() {
    let config = parse(&["-b", "foo.txt", "bar.txt"]);
    assert!(config.binary);
    assert_eq!(config.files, vec!["foo.txt", "bar.txt"]);
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["-"]);
    assert_eq!(config.files, vec!["-"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-b"]);
    assert!(!config.binary);
    assert_eq!(config.files, vec!["-b"]);
}

#[test]
fn hash_reader_works() {
    use sha256sum::ops::hash_reader;
    let mut data = b"hello world" as &[u8];
    let hash = hash_reader(&mut data).unwrap();
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn hash_file_works() {
    use sha256sum::ops::hash_file;
    use std::io::Write;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(b"hello world").unwrap();
    drop(f);
    let hash = hash_file(&path).unwrap();
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn format_hash_default() {
    use sha256sum::ops::format_hash;
    let config = Sha256sumConfig::default();
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "abc123  file.txt");
}

#[test]
fn format_hash_binary() {
    use sha256sum::ops::format_hash;
    let mut config = Sha256sumConfig::default();
    config.binary = true;
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "abc123 *file.txt");
}

#[test]
fn format_hash_tag() {
    use sha256sum::ops::format_hash;
    let mut config = Sha256sumConfig::default();
    config.tag = true;
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "SHA256 (file.txt) = abc123");
}
