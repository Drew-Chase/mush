use clap::Parser;

use bc::cli::BcConfig;
use bc::ops::{BcState, bc_repl, eval_expression, format_value};

fn parse(args: &[&str]) -> BcConfig {
    let mut full = vec!["bc"];
    full.extend_from_slice(args);
    BcConfig::parse_from(full)
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert!(!config.math_lib);
    assert!(config.files.is_empty());
}

#[test]
fn parse_math_lib() {
    let config = parse(&["-l"]);
    assert!(config.math_lib);
}

#[test]
fn help_returns_err() {
    assert!(BcConfig::try_parse_from(["bc", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(BcConfig::try_parse_from(["bc", "--version"]).is_err());
}

#[test]
fn eval_addition() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("2+3", &state).unwrap(), 5.0);
}

#[test]
fn eval_subtraction() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("10-3", &state).unwrap(), 7.0);
}

#[test]
fn eval_multiplication() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("4*5", &state).unwrap(), 20.0);
}

#[test]
fn eval_division() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("10/3", &state).unwrap(), 10.0 / 3.0);
}

#[test]
fn eval_modulus() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("10%3", &state).unwrap(), 1.0);
}

#[test]
fn eval_power() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("2^10", &state).unwrap(), 1024.0);
}

#[test]
fn eval_parentheses() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("(2+3)*4", &state).unwrap(), 20.0);
}

#[test]
fn eval_nested_parens() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("((1+2)*(3+4))", &state).unwrap(), 21.0);
}

#[test]
fn eval_negative() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("-5+3", &state).unwrap(), -2.0);
}

#[test]
fn eval_division_by_zero() {
    let state = BcState::new(false);
    assert!(eval_expression("1/0", &state).is_err());
}

#[test]
fn eval_math_sin() {
    let state = BcState::new(true);
    let val = eval_expression("s(0)", &state).unwrap();
    assert!((val - 0.0).abs() < 1e-10);
}

#[test]
fn eval_math_sqrt() {
    let state = BcState::new(true);
    let val = eval_expression("sqrt(4)", &state).unwrap();
    assert!((val - 2.0).abs() < 1e-10);
}

#[test]
fn eval_math_without_flag() {
    let state = BcState::new(false);
    assert!(eval_expression("s(0)", &state).is_err());
}

#[test]
fn format_scale_zero() {
    assert_eq!(format_value(3.7, 0), "3");
    assert_eq!(format_value(-3.7, 0), "-3");
}

#[test]
fn format_scale_two() {
    assert_eq!(format_value(3.14159, 2), "3.14");
}

#[test]
fn repl_basic() {
    let input = "2+3\n4*5\nquit\n";
    let mut cursor = std::io::Cursor::new(input.as_bytes());
    let mut output = Vec::new();
    let mut state = BcState::new(false);
    bc_repl(&mut cursor, &mut output, &mut state).unwrap();
    let s = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines[0], "5");
    assert_eq!(lines[1], "20");
}

#[test]
fn repl_scale() {
    let input = "scale=2\n10/3\n";
    let mut cursor = std::io::Cursor::new(input.as_bytes());
    let mut output = Vec::new();
    let mut state = BcState::new(false);
    bc_repl(&mut cursor, &mut output, &mut state).unwrap();
    let s = String::from_utf8(output).unwrap();
    assert_eq!(s.trim(), "3.33");
}

#[test]
fn eval_precedence() {
    let state = BcState::new(false);
    assert_eq!(eval_expression("2+3*4", &state).unwrap(), 14.0);
}

#[test]
fn eval_spaces() {
    let state = BcState::new(false);
    assert_eq!(eval_expression(" 2 + 3 ", &state).unwrap(), 5.0);
}
