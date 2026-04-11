use ln::cli::LnConfig;

fn parse(args: &[&str]) -> LnConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    LnConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.symbolic);
    assert!(!config.force);
    assert!(!config.interactive);
    assert!(!config.verbose);
    assert!(!config.no_deref);
    assert!(config.targets.is_empty());
}

#[test]
fn flag_s() {
    let config = parse(&["-s"]);
    assert!(config.symbolic);
}

#[test]
fn long_symbolic() {
    let config = parse(&["--symbolic"]);
    assert!(config.symbolic);
}

#[test]
fn flag_f() {
    let config = parse(&["-f"]);
    assert!(config.force);
}

#[test]
fn long_force() {
    let config = parse(&["--force"]);
    assert!(config.force);
}

#[test]
fn flag_i() {
    let config = parse(&["-i"]);
    assert!(config.interactive);
}

#[test]
fn long_interactive() {
    let config = parse(&["--interactive"]);
    assert!(config.interactive);
}

#[test]
fn flag_v() {
    let config = parse(&["-v"]);
    assert!(config.verbose);
}

#[test]
fn long_verbose() {
    let config = parse(&["--verbose"]);
    assert!(config.verbose);
}

#[test]
fn flag_n() {
    let config = parse(&["-n"]);
    assert!(config.no_deref);
}

#[test]
fn long_no_dereference() {
    let config = parse(&["--no-dereference"]);
    assert!(config.no_deref);
}

#[test]
fn combined_sf() {
    let config = parse(&["-sf"]);
    assert!(config.symbolic);
    assert!(config.force);
}

#[test]
fn combined_sfv() {
    let config = parse(&["-sfv"]);
    assert!(config.symbolic);
    assert!(config.force);
    assert!(config.verbose);
}

#[test]
fn positional_targets() {
    let config = parse(&["source.txt", "link.txt"]);
    assert_eq!(config.targets, vec!["source.txt", "link.txt"]);
}

#[test]
fn flags_and_targets() {
    let config = parse(&["-sf", "source.txt", "link.txt"]);
    assert!(config.symbolic);
    assert!(config.force);
    assert_eq!(config.targets, vec!["source.txt", "link.txt"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-s"]);
    assert!(!config.symbolic);
    assert_eq!(config.targets, vec!["-s"]);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(LnConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(LnConfig::from_args(&owned).is_none());
}

#[cfg(test)]
mod ops_tests {
    use std::path::Path;
    use tempfile::TempDir;

    use ln::cli::LnConfig;
    use ln::ops::create_link;

    #[test]
    fn hard_link_default() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("original.txt");
        std::fs::write(&target, "content").unwrap();

        let link = dir.path().join("hardlink.txt");
        let config = LnConfig::default();
        create_link(&target, &link, &config).unwrap();

        assert!(link.exists());
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "content");
    }

    #[test]
    fn force_removes_existing() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("source.txt");
        std::fs::write(&target, "new").unwrap();

        let link = dir.path().join("link.txt");
        std::fs::write(&link, "old").unwrap();

        let config = LnConfig {
            force: true,
            ..Default::default()
        };
        create_link(&target, &link, &config).unwrap();

        assert_eq!(std::fs::read_to_string(&link).unwrap(), "new");
    }

    #[test]
    fn error_without_force_on_existing() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("source.txt");
        std::fs::write(&target, "data").unwrap();

        let link = dir.path().join("link.txt");
        std::fs::write(&link, "existing").unwrap();

        let config = LnConfig::default();
        let result = create_link(&target, &link, &config);
        assert!(result.is_err());
    }

    #[test]
    fn verbose_flag_accepted() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("source.txt");
        std::fs::write(&target, "data").unwrap();

        let link = dir.path().join("link.txt");
        let config = LnConfig {
            verbose: true,
            ..Default::default()
        };
        create_link(&target, &link, &config).unwrap();
        assert!(link.exists());
    }

    #[test]
    #[cfg(windows)]
    fn symbolic_link_windows() {
        // Symbolic links on Windows may require elevated privileges
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("source.txt");
        std::fs::write(&target, "symdata").unwrap();

        let link = dir.path().join("symlink.txt");
        let config = LnConfig {
            symbolic: true,
            ..Default::default()
        };

        // This may fail without admin privileges, which is expected
        let _ = create_link(Path::new(&target), &link, &config);
    }

    #[test]
    #[cfg(unix)]
    fn symbolic_link_unix() {
        let dir = TempDir::new().unwrap();
        let target = dir.path().join("source.txt");
        std::fs::write(&target, "symdata").unwrap();

        let link = dir.path().join("symlink.txt");
        let config = LnConfig {
            symbolic: true,
            ..Default::default()
        };
        create_link(&target, &link, &config).unwrap();

        assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read_to_string(&link).unwrap(), "symdata");
    }
}
