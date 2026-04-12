use std::io::Cursor;

use strings_cmd::cli::StringsConfig;
use strings_cmd::ops;

fn parse(args: &[&str]) -> StringsConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    StringsConfig::from_args(&owned).expect("should not be --help/--version")
}

fn run_strings(input: &[u8], args: &[&str]) -> String {
    let config = parse(args);
    let mut inp = Cursor::new(input.to_vec());
    let mut out = Vec::new();
    ops::strings(&mut inp, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert_eq!(config.min_length, 4);
    assert!(!config.all);
    assert_eq!(config.radix, None);
}

#[test]
fn default_min_4() {
    // "hello" is 5 chars (printable), surrounded by non-printable bytes
    let mut data: Vec<u8> = vec![0x00, 0x01, 0x02];
    data.extend_from_slice(b"hello");
    data.extend_from_slice(&[0x00, 0x01]);
    data.extend_from_slice(b"ab"); // too short (2 < 4)
    data.push(0x00);

    let result = run_strings(&data, &[]);
    assert_eq!(result, "hello\n");
}

#[test]
fn min_length_8() {
    let mut data: Vec<u8> = vec![0x00];
    data.extend_from_slice(b"hello"); // 5 chars, < 8
    data.push(0x00);
    data.extend_from_slice(b"longstring"); // 10 chars, >= 8
    data.push(0x00);

    let result = run_strings(&data, &["-n", "8"]);
    assert_eq!(result, "longstring\n");
}

#[test]
fn radix_hex() {
    let mut data: Vec<u8> = vec![0x00; 16];
    data.extend_from_slice(b"test");
    data.push(0x00);

    let result = run_strings(&data, &["-t", "x"]);
    assert_eq!(result, "     10 test\n");
}

#[test]
fn radix_octal() {
    let mut data: Vec<u8> = vec![0x00; 8];
    data.extend_from_slice(b"test");
    data.push(0x00);

    let result = run_strings(&data, &["-t", "o"]);
    assert_eq!(result, "     10 test\n");
}

#[test]
fn radix_decimal() {
    let mut data: Vec<u8> = vec![0x00; 10];
    data.extend_from_slice(b"test");
    data.push(0x00);

    let result = run_strings(&data, &["-t", "d"]);
    assert_eq!(result, "     10 test\n");
}

#[test]
fn flag_all() {
    let config = parse(&["-a"]);
    assert!(config.all);
}

#[test]
fn multiple_strings() {
    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(b"first");
    data.push(0x00);
    data.extend_from_slice(b"second");
    data.push(0x00);

    let result = run_strings(&data, &[]);
    assert_eq!(result, "first\nsecond\n");
}
