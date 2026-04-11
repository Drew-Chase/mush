use base64::cli::Base64Config;

fn parse(args: &[&str]) -> Base64Config {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    Base64Config::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.decode);
    assert!(!config.ignore_garbage);
    assert_eq!(config.wrap, 76);
    assert_eq!(config.file, None);
}

#[test]
fn flag_d() {
    let config = parse(&["-d"]);
    assert!(config.decode);
}

#[test]
fn flag_i() {
    let config = parse(&["-i"]);
    assert!(config.ignore_garbage);
}

#[test]
fn flag_w() {
    let config = parse(&["-w", "0"]);
    assert_eq!(config.wrap, 0);
}

#[test]
fn flag_w_inline() {
    let config = parse(&["-w80"]);
    assert_eq!(config.wrap, 80);
}

#[test]
fn long_flags() {
    let config = parse(&["--decode", "--ignore-garbage", "--wrap", "120"]);
    assert!(config.decode);
    assert!(config.ignore_garbage);
    assert_eq!(config.wrap, 120);
}

#[test]
fn combined_di() {
    let config = parse(&["-di"]);
    assert!(config.decode);
    assert!(config.ignore_garbage);
}

#[test]
fn file_positional() {
    let config = parse(&["input.txt"]);
    assert_eq!(config.file, Some("input.txt".to_string()));
}

#[test]
fn dash_is_stdin() {
    let config = parse(&["-d", "-"]);
    assert!(config.decode);
    assert_eq!(config.file, Some("-".to_string()));
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-d"]);
    assert!(!config.decode);
    assert_eq!(config.file, Some("-d".to_string()));
}

#[test]
fn encode_simple() {
    use base64::ops::encode;
    let result = encode(b"hello world", 76);
    assert_eq!(result, "aGVsbG8gd29ybGQ=");
}

#[test]
fn encode_no_wrap() {
    use base64::ops::encode;
    let result = encode(b"hello world", 0);
    assert_eq!(result, "aGVsbG8gd29ybGQ=");
}

#[test]
fn encode_with_wrap() {
    use base64::ops::encode;
    let result = encode(b"hello world, this is a longer string to test wrapping", 20);
    assert!(result.contains('\n'));
    for line in result.lines() {
        assert!(line.len() <= 20);
    }
}

#[test]
fn decode_simple() {
    use base64::ops::decode;
    let result = decode("aGVsbG8gd29ybGQ=", false).unwrap();
    assert_eq!(result, b"hello world");
}

#[test]
fn decode_with_newlines() {
    use base64::ops::decode;
    let result = decode("aGVsbG8g\nd29ybGQ=\n", false).unwrap();
    assert_eq!(result, b"hello world");
}

#[test]
fn decode_ignore_garbage() {
    use base64::ops::decode;
    let result = decode("aGVsbG8g!!!d29ybGQ=", true).unwrap();
    assert_eq!(result, b"hello world");
}

#[test]
fn encode_empty() {
    use base64::ops::encode;
    let result = encode(b"", 76);
    assert_eq!(result, "");
}

#[test]
fn decode_empty() {
    use base64::ops::decode;
    let result = decode("", false).unwrap();
    assert!(result.is_empty());
}
