use echo::cli::EchoConfig;

fn parse(args: &[&str]) -> EchoConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    EchoConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn no_args() {
    let config = parse(&[]);
    assert!(!config.no_newline);
    assert!(!config.interpret_escapes);
    assert!(config.args.is_empty());
}

#[test]
fn simple_string() {
    let config = parse(&["hello"]);
    assert_eq!(config.args, vec!["hello"]);
}

#[test]
fn multiple_strings() {
    let config = parse(&["hello", "world"]);
    assert_eq!(config.args, vec!["hello", "world"]);
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "hello"]);
    assert!(config.no_newline);
    assert!(!config.interpret_escapes);
    assert_eq!(config.args, vec!["hello"]);
}

#[test]
fn flag_e() {
    let config = parse(&["-e", "hello"]);
    assert!(!config.no_newline);
    assert!(config.interpret_escapes);
    assert_eq!(config.args, vec!["hello"]);
}

#[test]
fn flag_big_e_default() {
    let config = parse(&["-E", "hello"]);
    assert!(!config.interpret_escapes);
    assert_eq!(config.args, vec!["hello"]);
}

#[test]
fn combined_ne() {
    let config = parse(&["-ne", "hello"]);
    assert!(config.no_newline);
    assert!(config.interpret_escapes);
}

#[test]
fn combined_nee() {
    let config = parse(&["-nee", "hello"]);
    assert!(config.no_newline);
    assert!(config.interpret_escapes);
}

#[test]
fn combined_ne_big_e_last_wins() {
    let config = parse(&["-neE", "hello"]);
    assert!(config.no_newline);
    assert!(!config.interpret_escapes);
}

#[test]
fn combined_n_big_e_e_last_wins() {
    let config = parse(&["-nEe", "hello"]);
    assert!(config.no_newline);
    assert!(config.interpret_escapes);
}

#[test]
fn separate_flags() {
    let config = parse(&["-n", "-e", "hello"]);
    assert!(config.no_newline);
    assert!(config.interpret_escapes);
    assert_eq!(config.args, vec!["hello"]);
}

#[test]
fn unknown_flag_is_literal() {
    let config = parse(&["-a", "hello"]);
    assert!(!config.no_newline);
    assert_eq!(config.args, vec!["-a", "hello"]);
}

#[test]
fn partial_unknown_flag_is_literal() {
    let config = parse(&["-na", "hello"]);
    assert!(!config.no_newline);
    assert_eq!(config.args, vec!["-na", "hello"]);
}

#[test]
fn double_dash_is_literal() {
    let config = parse(&["--", "hello"]);
    assert_eq!(config.args, vec!["--", "hello"]);
}

#[test]
fn bare_dash_is_literal() {
    let config = parse(&["-"]);
    assert_eq!(config.args, vec!["-"]);
}

#[test]
fn flag_then_unknown_stops_scanning() {
    let config = parse(&["-n", "-a", "hello"]);
    assert!(config.no_newline);
    assert_eq!(config.args, vec!["-a", "hello"]);
}

#[test]
fn only_flag_no_positional() {
    let config = parse(&["-n"]);
    assert!(config.no_newline);
    assert!(config.args.is_empty());
}

#[test]
fn empty_string_arg() {
    let config = parse(&[""]);
    assert_eq!(config.args, vec![""]);
}

#[test]
fn flag_like_later_arg_is_literal() {
    let config = parse(&["hello", "-n"]);
    assert!(!config.no_newline);
    assert_eq!(config.args, vec!["hello", "-n"]);
}
