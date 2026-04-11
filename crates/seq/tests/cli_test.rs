use seq::cli::SeqConfig;

fn parse(args: &[&str]) -> SeqConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    SeqConfig::from_args(&owned)
        .expect("should not error")
        .expect("should not be --help/--version")
}

#[test]
fn one_arg_last_only() {
    let config = parse(&["5"]);
    assert_eq!(config.first, 1.0);
    assert_eq!(config.increment, 1.0);
    assert_eq!(config.last, 5.0);
}

#[test]
fn two_args_first_last() {
    let config = parse(&["3", "7"]);
    assert_eq!(config.first, 3.0);
    assert_eq!(config.increment, 1.0);
    assert_eq!(config.last, 7.0);
}

#[test]
fn three_args_first_increment_last() {
    let config = parse(&["1", "2", "10"]);
    assert_eq!(config.first, 1.0);
    assert_eq!(config.increment, 2.0);
    assert_eq!(config.last, 10.0);
}

#[test]
fn separator_flag() {
    let config = parse(&["-s", ", ", "5"]);
    assert_eq!(config.separator, ", ");
    assert_eq!(config.last, 5.0);
}

#[test]
fn format_flag() {
    let config = parse(&["-f", "%03.0f", "5"]);
    assert_eq!(config.format, Some("%03.0f".to_string()));
}

#[test]
fn equal_width_flag() {
    let config = parse(&["-w", "10"]);
    assert!(config.equal_width);
    assert_eq!(config.last, 10.0);
}

#[test]
fn combined_flags() {
    let config = parse(&["-w", "-s", ":", "1", "2", "10"]);
    assert!(config.equal_width);
    assert_eq!(config.separator, ":");
    assert_eq!(config.first, 1.0);
    assert_eq!(config.increment, 2.0);
    assert_eq!(config.last, 10.0);
}

#[test]
fn negative_numbers() {
    let config = parse(&["-5", "5"]);
    assert_eq!(config.first, -5.0);
    assert_eq!(config.last, 5.0);
}

#[test]
fn missing_operand() {
    let owned: Vec<String> = Vec::new();
    let result = SeqConfig::from_args(&owned);
    assert!(result.is_err());
}

#[test]
fn extra_operand() {
    let owned: Vec<String> = ["1", "2", "3", "4"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let result = SeqConfig::from_args(&owned);
    assert!(result.is_err());
}
