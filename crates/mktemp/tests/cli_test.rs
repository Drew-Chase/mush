use mktemp::cli::MktempConfig;
use mktemp::ops;

fn parse(args: &[&str]) -> MktempConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    MktempConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_no_args() {
    let config = parse(&[]);
    assert!(!config.directory);
    assert!(!config.dry_run);
    assert!(!config.quiet);
    assert!(config.tmpdir.is_none());
    assert!(config.suffix.is_none());
    assert!(config.template.is_none());
}

#[test]
fn directory_flag_short() {
    let config = parse(&["-d"]);
    assert!(config.directory);
}

#[test]
fn directory_flag_long() {
    let config = parse(&["--directory"]);
    assert!(config.directory);
}

#[test]
fn dry_run_flag() {
    let config = parse(&["-u"]);
    assert!(config.dry_run);
}

#[test]
fn quiet_flag() {
    let config = parse(&["-q"]);
    assert!(config.quiet);
}

#[test]
fn tmpdir_short() {
    let config = parse(&["-p", "/tmp"]);
    assert_eq!(config.tmpdir.as_deref(), Some("/tmp"));
}

#[test]
fn tmpdir_long() {
    let config = parse(&["--tmpdir", "/tmp"]);
    assert_eq!(config.tmpdir.as_deref(), Some("/tmp"));
}

#[test]
fn tmpdir_long_eq() {
    let config = parse(&["--tmpdir=/var/tmp"]);
    assert_eq!(config.tmpdir.as_deref(), Some("/var/tmp"));
}

#[test]
fn suffix_flag() {
    let config = parse(&["--suffix", ".txt"]);
    assert_eq!(config.suffix.as_deref(), Some(".txt"));
}

#[test]
fn suffix_flag_eq() {
    let config = parse(&["--suffix=.log"]);
    assert_eq!(config.suffix.as_deref(), Some(".log"));
}

#[test]
fn template_positional() {
    let config = parse(&["myfile.XXXXXX"]);
    assert_eq!(config.template.as_deref(), Some("myfile.XXXXXX"));
}

#[test]
fn combined_flags() {
    let config = parse(&["-duq", "test.XXXXXX"]);
    assert!(config.directory);
    assert!(config.dry_run);
    assert!(config.quiet);
    assert_eq!(config.template.as_deref(), Some("test.XXXXXX"));
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(MktempConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(MktempConfig::from_args(&owned).is_none());
}

#[test]
fn mktemp_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    let config = MktempConfig {
        tmpdir: Some(dir.path().to_string_lossy().to_string()),
        ..Default::default()
    };
    let path = ops::mktemp(&config).unwrap();
    assert!(path.exists());
    assert!(path.is_file());
}

#[test]
fn mktemp_creates_directory() {
    let dir = tempfile::tempdir().unwrap();
    let config = MktempConfig {
        directory: true,
        tmpdir: Some(dir.path().to_string_lossy().to_string()),
        ..Default::default()
    };
    let path = ops::mktemp(&config).unwrap();
    assert!(path.exists());
    assert!(path.is_dir());
}

#[test]
fn mktemp_dry_run_no_create() {
    let dir = tempfile::tempdir().unwrap();
    let config = MktempConfig {
        dry_run: true,
        tmpdir: Some(dir.path().to_string_lossy().to_string()),
        ..Default::default()
    };
    let path = ops::mktemp(&config).unwrap();
    assert!(!path.exists());
}

#[test]
fn mktemp_with_suffix() {
    let dir = tempfile::tempdir().unwrap();
    let config = MktempConfig {
        suffix: Some(".txt".to_string()),
        tmpdir: Some(dir.path().to_string_lossy().to_string()),
        ..Default::default()
    };
    let path = ops::mktemp(&config).unwrap();
    assert!(path.to_string_lossy().ends_with(".txt"));
    assert!(path.exists());
}

#[test]
fn mktemp_with_custom_template() {
    let dir = tempfile::tempdir().unwrap();
    let config = MktempConfig {
        template: Some("myapp.XXXXXX".to_string()),
        tmpdir: Some(dir.path().to_string_lossy().to_string()),
        ..Default::default()
    };
    let path = ops::mktemp(&config).unwrap();
    let name = path.file_name().unwrap().to_string_lossy();
    assert!(name.starts_with("myapp."));
    assert!(path.exists());
}

#[test]
fn mktemp_too_few_xs_fails() {
    let config = MktempConfig {
        template: Some("tmpXX".to_string()),
        ..Default::default()
    };
    assert!(ops::mktemp(&config).is_err());
}
