use arch::cli::ArchConfig;

fn parse(args: &[&str]) -> ArchConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    ArchConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn no_args() {
    let _config = parse(&[]);
}

#[test]
fn machine_arch_not_empty() {
    assert!(!arch::ops::machine_arch().is_empty());
}
