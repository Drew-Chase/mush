use yes::cli::YesConfig;

fn parse(args: &[&str]) -> YesConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    YesConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn no_args_defaults_to_y() {
    let config = parse(&[]);
    assert_eq!(config.string, "y");
}

#[test]
fn custom_string() {
    let config = parse(&["hello"]);
    assert_eq!(config.string, "hello");
}

#[test]
fn multiple_strings_joined() {
    let config = parse(&["hello", "world"]);
    assert_eq!(config.string, "hello world");
}
