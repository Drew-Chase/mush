use clap::Parser;

use yes::cli::YesConfig;

fn parse(args: &[&str]) -> YesConfig {
    let mut full = vec!["yes"];
    full.extend_from_slice(args);
    YesConfig::parse_from(full)
}

#[test]
fn no_args_defaults_to_y() {
    let config = parse(&[]);
    assert_eq!(config.string(), "y");
}

#[test]
fn custom_string() {
    let config = parse(&["hello"]);
    assert_eq!(config.string(), "hello");
}

#[test]
fn multiple_strings_joined() {
    let config = parse(&["hello", "world"]);
    assert_eq!(config.string(), "hello world");
}
