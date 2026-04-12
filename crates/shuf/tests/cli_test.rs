use shuf::cli::ShufConfig;
use shuf::ops::{XorShift64, range_to_lines, read_lines, shuf_lines, shuffle};

fn parse(args: &[&str]) -> ShufConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    ShufConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.echo_mode);
    assert!(!config.repeat);
    assert!(config.range.is_none());
}

#[test]
fn parse_echo() {
    let config = parse(&["-e", "a", "b", "c"]);
    assert!(config.echo_mode);
    assert_eq!(config.echo_args, vec!["a", "b", "c"]);
}

#[test]
fn parse_range() {
    let config = parse(&["-i", "1-10"]);
    assert_eq!(config.range, Some((1, 10)));
}

#[test]
fn parse_head_count() {
    let config = parse(&["-n", "5"]);
    assert_eq!(config.head_count, Some(5));
}

#[test]
fn parse_repeat() {
    let config = parse(&["-r", "-n", "3"]);
    assert!(config.repeat);
    assert_eq!(config.head_count, Some(3));
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(ShufConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(ShufConfig::from_args(&owned).is_none());
}

#[test]
fn xorshift_deterministic() {
    let mut rng1 = XorShift64::from_seed(42);
    let mut rng2 = XorShift64::from_seed(42);
    for _ in 0..100 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn shuffle_preserves_elements() {
    let mut items: Vec<String> = vec!["a", "b", "c", "d"].into_iter().map(String::from).collect();
    let mut rng = XorShift64::from_seed(12345);
    shuffle(&mut items, &mut rng);
    items.sort();
    assert_eq!(items, vec!["a", "b", "c", "d"]);
}

#[test]
fn range_to_lines_basic() {
    let lines = range_to_lines(1, 5);
    assert_eq!(lines, vec!["1", "2", "3", "4", "5"]);
}

#[test]
fn shuf_lines_head_count() {
    let lines: Vec<String> = (1..=10).map(|n| n.to_string()).collect();
    let mut rng = XorShift64::from_seed(99);
    let mut output = Vec::new();
    shuf_lines(&lines, Some(3), false, &mut output, &mut rng).unwrap();
    let result = String::from_utf8(output).unwrap();
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn shuf_lines_repeat() {
    let lines = vec!["x".to_string()];
    let mut rng = XorShift64::from_seed(99);
    let mut output = Vec::new();
    shuf_lines(&lines, Some(5), true, &mut output, &mut rng).unwrap();
    let result = String::from_utf8(output).unwrap();
    assert_eq!(result.lines().count(), 5);
    for line in result.lines() {
        assert_eq!(line, "x");
    }
}

#[test]
fn read_lines_basic() {
    let input = "one\ntwo\nthree\n";
    let mut cursor = std::io::Cursor::new(input.as_bytes());
    let lines = read_lines(&mut cursor).unwrap();
    assert_eq!(lines, vec!["one", "two", "three"]);
}

#[test]
fn shuf_empty() {
    let lines: Vec<String> = vec![];
    let mut rng = XorShift64::from_seed(1);
    let mut output = Vec::new();
    shuf_lines(&lines, None, false, &mut output, &mut rng).unwrap();
    assert!(output.is_empty());
}
