use install::cli::InstallConfig;
use install::ops::install_files;

fn parse(args: &[&str]) -> InstallConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    InstallConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn directory_mode_flag() {
    let config = parse(&["-d", "dir1", "dir2"]);
    assert!(config.directory_mode);
    assert_eq!(config.files, vec!["dir1", "dir2"]);
}

#[test]
fn mode_flag_short() {
    let config = parse(&["-m", "755", "src", "dest"]);
    assert_eq!(config.mode.as_deref(), Some("755"));
}

#[test]
fn mode_flag_long() {
    let config = parse(&["--mode", "644", "src", "dest"]);
    assert_eq!(config.mode.as_deref(), Some("644"));
}

#[test]
fn mode_flag_long_eq() {
    let config = parse(&["--mode=755", "src", "dest"]);
    assert_eq!(config.mode.as_deref(), Some("755"));
}

#[test]
fn verbose_flag() {
    let config = parse(&["-v", "src", "dest"]);
    assert!(config.verbose);
}

#[test]
fn compare_flag_short() {
    let config = parse(&["-C", "src", "dest"]);
    assert!(config.compare);
}

#[test]
fn compare_flag_long() {
    let config = parse(&["--compare", "src", "dest"]);
    assert!(config.compare);
}

#[test]
fn create_leading_flag() {
    let config = parse(&["-D", "src", "dest"]);
    assert!(config.create_leading);
}

#[test]
fn target_dir_short() {
    let config = parse(&["-t", "/usr/bin", "file1"]);
    assert_eq!(config.target_dir.as_deref(), Some("/usr/bin"));
}

#[test]
fn target_dir_long() {
    let config = parse(&["--target-directory", "/usr/bin", "file1"]);
    assert_eq!(config.target_dir.as_deref(), Some("/usr/bin"));
}

#[test]
fn target_dir_long_eq() {
    let config = parse(&["--target-directory=/usr/bin", "file1"]);
    assert_eq!(config.target_dir.as_deref(), Some("/usr/bin"));
}

#[test]
fn combined_flags() {
    let config = parse(&["-dvC", "src", "dest"]);
    assert!(config.directory_mode);
    assert!(config.verbose);
    assert!(config.compare);
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(InstallConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(InstallConfig::from_args(&owned).is_none());
}

#[test]
fn install_creates_directory() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("newdir");
    let config = InstallConfig {
        directory_mode: true,
        files: vec![target.to_string_lossy().to_string()],
        ..Default::default()
    };
    install_files(&config).unwrap();
    assert!(target.is_dir());
}

#[test]
fn install_copy_file() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");
    std::fs::write(&src, "hello world").unwrap();

    let config = InstallConfig {
        files: vec![
            src.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        ],
        ..Default::default()
    };
    install_files(&config).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "hello world");
}

#[test]
fn install_compare_skip_identical() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");
    std::fs::write(&src, "same content").unwrap();
    std::fs::write(&dest, "same content").unwrap();

    let config = InstallConfig {
        compare: true,
        verbose: true,
        files: vec![
            src.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        ],
        ..Default::default()
    };
    // Should succeed without error (skips copy)
    install_files(&config).unwrap();
}

#[test]
fn install_with_target_dir() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("source.txt");
    let target = dir.path().join("target_dir");
    std::fs::create_dir(&target).unwrap();
    std::fs::write(&src, "content").unwrap();

    let config = InstallConfig {
        target_dir: Some(target.to_string_lossy().to_string()),
        files: vec![src.to_string_lossy().to_string()],
        ..Default::default()
    };
    install_files(&config).unwrap();
    assert!(target.join("source.txt").exists());
}

#[test]
fn install_create_leading_dirs() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("source.txt");
    let dest = dir.path().join("a").join("b").join("c").join("dest.txt");
    std::fs::write(&src, "nested").unwrap();

    let config = InstallConfig {
        create_leading: true,
        files: vec![
            src.to_string_lossy().to_string(),
            dest.to_string_lossy().to_string(),
        ],
        ..Default::default()
    };
    install_files(&config).unwrap();
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "nested");
}
