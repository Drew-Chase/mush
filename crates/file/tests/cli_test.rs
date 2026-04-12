use clap::Parser;

use file::cli::FileConfig;
use file::ops::detect_file_type;

fn parse(args: &[&str]) -> FileConfig {
    let mut full = vec!["file"];
    full.extend_from_slice(args);
    FileConfig::parse_from(full)
}

#[test]
fn basic_file_arg() {
    let config = parse(&["test.txt"]);
    assert_eq!(config.files, vec!["test.txt"]);
    assert!(!config.brief);
    assert!(!config.mime);
}

#[test]
fn brief_flag_short() {
    let config = parse(&["-b", "test.txt"]);
    assert!(config.brief);
}

#[test]
fn brief_flag_long() {
    let config = parse(&["--brief", "test.txt"]);
    assert!(config.brief);
}

#[test]
fn mime_flag_short() {
    let config = parse(&["-i", "test.txt"]);
    assert!(config.mime);
}

#[test]
fn mime_flag_long() {
    let config = parse(&["--mime", "test.txt"]);
    assert!(config.mime);
}

#[test]
fn mime_type_flag() {
    let config = parse(&["--mime-type", "test.txt"]);
    assert!(config.mime_type);
}

#[test]
fn dereference_flag_short() {
    let config = parse(&["-L", "test.txt"]);
    assert!(config.dereference);
}

#[test]
fn dereference_flag_long() {
    let config = parse(&["--dereference", "test.txt"]);
    assert!(config.dereference);
}

#[test]
fn multiple_files() {
    let config = parse(&["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
}

#[test]
fn combined_flags() {
    let config = parse(&["-biL", "test.txt"]);
    assert!(config.brief);
    assert!(config.mime);
    assert!(config.dereference);
}

#[test]
fn detect_empty_file() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("empty");
    std::fs::write(&f, b"").unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "empty");
}

#[test]
fn detect_text_file() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("text.txt");
    std::fs::write(&f, "Hello, world!\n").unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "ASCII text");
}

#[test]
fn detect_text_file_mime() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("text.txt");
    std::fs::write(&f, "Hello, world!\n").unwrap();
    let config = FileConfig { mime: true, ..Default::default() };
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "text/plain");
}

#[test]
fn detect_directory() {
    let dir = tempfile::tempdir().unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(dir.path(), &config).unwrap();
    assert_eq!(result, "directory");
}

#[test]
fn detect_directory_mime() {
    let dir = tempfile::tempdir().unwrap();
    let config = FileConfig { mime: true, ..Default::default() };
    let result = detect_file_type(dir.path(), &config).unwrap();
    assert_eq!(result, "inode/directory");
}

#[test]
fn detect_png_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.png");
    let mut data = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    data.extend_from_slice(&[0u8; 100]);
    std::fs::write(&f, &data).unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "PNG image data");
}

#[test]
fn detect_jpeg_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.jpg");
    let mut data = vec![0xff, 0xd8, 0xff, 0xe0];
    data.extend_from_slice(&[0u8; 100]);
    std::fs::write(&f, &data).unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "JPEG image data");
}

#[test]
fn detect_pdf_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.pdf");
    std::fs::write(&f, b"%PDF-1.4 some content here").unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "PDF document");
}

#[test]
fn detect_gzip_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.gz");
    let mut data = vec![0x1f, 0x8b];
    data.extend_from_slice(&[0u8; 100]);
    std::fs::write(&f, &data).unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "gzip compressed data");
}

#[test]
fn detect_zip_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.zip");
    let mut data = vec![b'P', b'K', 0x03, 0x04];
    data.extend_from_slice(&[0u8; 100]);
    std::fs::write(&f, &data).unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "Zip archive data");
}

#[test]
fn detect_pe_magic() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("test.exe");
    let mut data = vec![b'M', b'Z'];
    data.extend_from_slice(&[0u8; 100]);
    std::fs::write(&f, &data).unwrap();
    let config = FileConfig::default();
    let result = detect_file_type(&f, &config).unwrap();
    assert_eq!(result, "PE32 executable (Windows)");
}
