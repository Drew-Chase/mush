use find::cli::{Action, Cmp, FileType, FindConfig, Predicate, SizeSpec};

fn parse(args: &[&str]) -> FindConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    FindConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults_no_args() {
    let config = parse(&[]);
    assert_eq!(config.paths, vec!["."]);
    assert!(config.predicates.is_empty());
    assert_eq!(config.actions, vec![Action::Print]);
    assert!(config.max_depth.is_none());
    assert!(config.min_depth.is_none());
}

#[test]
fn default_path_with_predicate() {
    let config = parse(&["-name", "*.rs"]);
    assert_eq!(config.paths, vec!["."]);
    assert_eq!(config.predicates, vec![Predicate::Name("*.rs".to_string())]);
}

#[test]
fn explicit_path() {
    let config = parse(&["/tmp", "-name", "*.txt"]);
    assert_eq!(config.paths, vec!["/tmp"]);
}

#[test]
fn multiple_paths() {
    let config = parse(&["src", "tests", "-name", "*.rs"]);
    assert_eq!(config.paths, vec!["src", "tests"]);
}

#[test]
fn flag_name() {
    let config = parse(&["-name", "*.rs"]);
    assert_eq!(config.predicates, vec![Predicate::Name("*.rs".to_string())]);
}

#[test]
fn flag_iname() {
    let config = parse(&["-iname", "*.RS"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::IName("*.RS".to_string())]
    );
}

#[test]
fn flag_type_file() {
    let config = parse(&["-type", "f"]);
    assert_eq!(config.predicates, vec![Predicate::Type(FileType::File)]);
}

#[test]
fn flag_type_dir() {
    let config = parse(&["-type", "d"]);
    assert_eq!(config.predicates, vec![Predicate::Type(FileType::Dir)]);
}

#[test]
fn flag_type_symlink() {
    let config = parse(&["-type", "l"]);
    assert_eq!(config.predicates, vec![Predicate::Type(FileType::Symlink)]);
}

#[test]
fn flag_size_greater() {
    let config = parse(&["-size", "+1M"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Size(SizeSpec {
            cmp: Cmp::GreaterThan,
            bytes: 1024 * 1024,
        })]
    );
}

#[test]
fn flag_size_less() {
    let config = parse(&["-size", "-100k"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Size(SizeSpec {
            cmp: Cmp::LessThan,
            bytes: 100 * 1024,
        })]
    );
}

#[test]
fn flag_size_exact_bytes() {
    let config = parse(&["-size", "0c"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Size(SizeSpec {
            cmp: Cmp::Exact,
            bytes: 0,
        })]
    );
}

#[test]
fn flag_size_blocks_default() {
    let config = parse(&["-size", "10"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Size(SizeSpec {
            cmp: Cmp::Exact,
            bytes: 10 * 512,
        })]
    );
}

#[test]
fn flag_size_gigabytes() {
    let config = parse(&["-size", "+2G"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Size(SizeSpec {
            cmp: Cmp::GreaterThan,
            bytes: 2 * 1024 * 1024 * 1024,
        })]
    );
}

#[test]
fn flag_empty() {
    let config = parse(&["-empty"]);
    assert_eq!(config.predicates, vec![Predicate::Empty]);
}

#[test]
fn flag_newer() {
    let config = parse(&["-newer", "reference.txt"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Newer("reference.txt".to_string())]
    );
}

#[test]
fn flag_path() {
    let config = parse(&["-path", "*/src/*.rs"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Path("*/src/*.rs".to_string())]
    );
}

#[test]
fn flag_regex() {
    let config = parse(&["-regex", r".*\.rs$"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Regex(r".*\.rs$".to_string())]
    );
}

#[test]
fn flag_maxdepth() {
    let config = parse(&["-maxdepth", "3"]);
    assert_eq!(config.max_depth, Some(3));
}

#[test]
fn flag_mindepth() {
    let config = parse(&["-mindepth", "1"]);
    assert_eq!(config.min_depth, Some(1));
}

#[test]
fn flag_mtime() {
    let config = parse(&["-mtime", "+7"]);
    assert_eq!(config.predicates, vec![Predicate::Mtime(7)]);
}

#[test]
fn flag_mtime_negative() {
    let config = parse(&["-mtime", "-3"]);
    assert_eq!(config.predicates, vec![Predicate::Mtime(-3)]);
}

#[test]
fn flag_mmin() {
    let config = parse(&["-mmin", "+60"]);
    assert_eq!(config.predicates, vec![Predicate::Mmin(60)]);
}

#[test]
fn flag_perm() {
    let config = parse(&["-perm", "755"]);
    assert_eq!(config.predicates, vec![Predicate::Perm(0o755)]);
}

#[test]
fn flag_not_name() {
    let config = parse(&["-not", "-name", "*.tmp"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Not(Box::new(Predicate::Name(
            "*.tmp".to_string()
        )))]
    );
}

#[test]
fn flag_bang_name() {
    let config = parse(&["!", "-name", "*.tmp"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Not(Box::new(Predicate::Name(
            "*.tmp".to_string()
        )))]
    );
}

#[test]
fn flag_or() {
    let config = parse(&["-name", "*.rs", "-o", "-name", "*.toml"]);
    assert_eq!(
        config.predicates,
        vec![Predicate::Or(
            Box::new(Predicate::Name("*.rs".to_string())),
            Box::new(Predicate::Name("*.toml".to_string())),
        )]
    );
}

#[test]
fn flag_print() {
    let config = parse(&["-name", "*.rs", "-print"]);
    assert_eq!(config.actions, vec![Action::Print]);
}

#[test]
fn flag_print0() {
    let config = parse(&["-name", "*.rs", "-print0"]);
    assert_eq!(config.actions, vec![Action::Print0]);
}

#[test]
fn flag_delete() {
    let config = parse(&["-name", "*.tmp", "-delete"]);
    assert_eq!(config.actions, vec![Action::Delete]);
}

#[test]
fn flag_exec_semicolon() {
    let config = parse(&["-exec", "echo", "{}", ";"]);
    assert_eq!(
        config.actions,
        vec![Action::Exec(vec![
            "echo".to_string(),
            "{}".to_string(),
        ])]
    );
}

#[test]
fn flag_exec_plus() {
    let config = parse(&["-exec", "echo", "{}", "+"]);
    assert_eq!(
        config.actions,
        vec![Action::ExecPlus(vec![
            "echo".to_string(),
            "{}".to_string(),
        ])]
    );
}

#[test]
fn combined_predicates() {
    let config = parse(&[
        "src", "-type", "f", "-name", "*.rs", "-maxdepth", "2",
    ]);
    assert_eq!(config.paths, vec!["src"]);
    assert_eq!(config.max_depth, Some(2));
    assert!(config
        .predicates
        .contains(&Predicate::Type(FileType::File)));
    assert!(config
        .predicates
        .contains(&Predicate::Name("*.rs".to_string())));
}

#[test]
fn default_action_is_print() {
    let config = parse(&["-name", "*.rs"]);
    assert_eq!(config.actions, vec![Action::Print]);
}

#[test]
fn multiple_actions() {
    let config = parse(&["-name", "*.rs", "-print", "-print0"]);
    assert_eq!(config.actions, vec![Action::Print, Action::Print0]);
}

#[test]
fn help_returns_none() {
    let args: Vec<String> = vec!["--help".to_string()];
    assert!(FindConfig::from_args(&args).is_none());
}

#[test]
fn version_returns_none() {
    let args: Vec<String> = vec!["--version".to_string()];
    assert!(FindConfig::from_args(&args).is_none());
}
