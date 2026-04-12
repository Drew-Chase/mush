use clap::Parser;

use xxd::cli::XxdConfig;
use xxd::ops::{xxd_hex_dump, xxd_reverse};

fn parse(args: &[&str]) -> XxdConfig {
    let mut full = vec!["xxd"];
    full.extend_from_slice(args);
    XxdConfig::parse_from(full)
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert_eq!(config.cols, 16);
    assert_eq!(config.group_size, 2);
    assert!(!config.upper);
    assert!(!config.plain);
}

#[test]
fn parse_cols() {
    let config = parse(&["-c", "8"]);
    assert_eq!(config.cols, 8);
}

#[test]
fn parse_upper() {
    let config = parse(&["-u"]);
    assert!(config.upper);
}

#[test]
fn parse_plain() {
    let config = parse(&["-p"]);
    assert!(config.plain);
}

#[test]
fn parse_reverse() {
    let config = parse(&["-r"]);
    assert!(config.reverse);
}

#[test]
fn hex_dump_basic() {
    let input = b"Hello";
    let config = XxdConfig::default();
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert!(s.contains("4865 6c6c 6f"));
    assert!(s.contains("Hello"));
}

#[test]
fn hex_dump_upper() {
    let input = b"AB";
    let config = XxdConfig { upper: true, ..Default::default() };
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert!(s.contains("4142"));
}

#[test]
fn hex_dump_plain() {
    let input = b"Hi";
    let config = XxdConfig { plain: true, ..Default::default() };
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert_eq!(s.trim(), "4869");
}

#[test]
fn hex_dump_bits() {
    let input = b"A";
    let config = XxdConfig { bits: true, ..Default::default() };
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert!(s.contains("01000001"));
}

#[test]
fn reverse_plain_hex() {
    let hex = "48656c6c6f\n";
    let mut input = std::io::Cursor::new(hex.as_bytes());
    let mut output = Vec::new();
    xxd_reverse(&mut input, &mut output).unwrap();
    assert_eq!(&output, b"Hello");
}

#[test]
fn hex_dump_with_length() {
    let input = b"Hello, World!";
    let config = XxdConfig { length: Some(5), ..Default::default() };
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert!(s.contains("Hello"));
    assert!(!s.contains("World"));
}

#[test]
fn hex_dump_include() {
    let input = b"AB";
    let config = XxdConfig { include: true, file: Some("test.bin".to_string()), ..Default::default() };
    let mut output = Vec::new();
    xxd_hex_dump(&mut &input[..], &mut output, &config).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert!(s.contains("unsigned char test_bin[]"));
    assert!(s.contains("0x41"));
    assert!(s.contains("0x42"));
}
