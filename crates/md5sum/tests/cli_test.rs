use clap::Parser;

use md5sum::cli::Md5sumConfig;

fn parse(args: &[&str]) -> Md5sumConfig {
    let mut full = vec!["md5sum"];
    full.extend_from_slice(args);
    Md5sumConfig::parse_from(full)
}

#[test]
fn defaults() {
    let config = parse(&[]);
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
    use md5sum::ops::hash_reader;
    let mut data = b"hello world" as &[u8];
    let hash = hash_reader(&mut data).unwrap();
    assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
}

#[test]
fn hash_file_works() {
    use md5sum::ops::hash_file;
    use std::io::Write;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(b"hello world").unwrap();
    drop(f);
    let hash = hash_file(&path).unwrap();
    assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
}

#[test]
fn format_hash_default() {
    use md5sum::ops::format_hash;
    let config = Md5sumConfig::default();
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "abc123  file.txt");
}

#[test]
fn format_hash_binary() {
    use md5sum::ops::format_hash;
    let mut config = Md5sumConfig::default();
    config.binary = true;
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "abc123 *file.txt");
}

#[test]
fn format_hash_tag() {
    use md5sum::ops::format_hash;
    let mut config = Md5sumConfig::default();
    config.tag = true;
    let result = format_hash("abc123", "file.txt", &config);
    assert_eq!(result, "MD5 (file.txt) = abc123");
}
