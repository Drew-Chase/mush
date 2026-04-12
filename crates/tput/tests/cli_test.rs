use clap::Parser;

use tput::cli::{TputCapability, TputConfig};
use tput::ops::execute_capability;

fn parse(args: &[&str]) -> TputConfig {
    let mut full = vec!["tput"];
    full.extend_from_slice(args);
    let mut config = TputConfig::parse_from(full);
    config.resolve().expect("should resolve successfully");
    config
}

#[test]
fn parse_cols() {
    let config = parse(&["cols"]);
    assert_eq!(*config.get_capability(), TputCapability::Cols);
}

#[test]
fn parse_lines() {
    let config = parse(&["lines"]);
    assert_eq!(*config.get_capability(), TputCapability::Lines);
}

#[test]
fn parse_colors() {
    let config = parse(&["colors"]);
    assert_eq!(*config.get_capability(), TputCapability::Colors);
}

#[test]
fn parse_bold() {
    let config = parse(&["bold"]);
    assert_eq!(*config.get_capability(), TputCapability::Bold);
}

#[test]
fn parse_sgr0() {
    let config = parse(&["sgr0"]);
    assert_eq!(*config.get_capability(), TputCapability::Sgr0);
}

#[test]
fn parse_setaf() {
    let config = parse(&["setaf", "1"]);
    assert_eq!(*config.get_capability(), TputCapability::Setaf(1));
}

#[test]
fn parse_clear() {
    let config = parse(&["clear"]);
    assert_eq!(*config.get_capability(), TputCapability::Clear);
}

#[test]
fn parse_cup() {
    let config = parse(&["cup", "10", "20"]);
    assert_eq!(*config.get_capability(), TputCapability::Cup(10, 20));
}

#[test]
fn unknown_cap_returns_err() {
    let mut full = vec!["tput"];
    full.push("foobar");
    let mut config = TputConfig::parse_from(full);
    assert!(config.resolve().is_err());
}

#[test]
fn execute_colors() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Colors, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap().trim(), "256");
}

#[test]
fn execute_bold() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Bold, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\x1b[1m");
}

#[test]
fn execute_sgr0() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Sgr0, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\x1b[0m");
}

#[test]
fn execute_setaf_red() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Setaf(1), &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\x1b[31m");
}

#[test]
fn execute_clear() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Clear, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\x1b[2J\x1b[H");
}

#[test]
fn execute_cup() {
    let mut output = Vec::new();
    execute_capability(&TputCapability::Cup(5, 10), &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\x1b[6;11H");
}
