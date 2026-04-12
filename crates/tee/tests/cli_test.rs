use clap::Parser;

use tee::cli::TeeConfig;

fn parse(args: &[&str]) -> TeeConfig {
    let mut full = vec!["tee"];
    full.extend_from_slice(args);
    TeeConfig::parse_from(full)
}

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.append);
    assert!(config.files.is_empty());
}

#[test]
fn flag_a() {
    let config = parse(&["-a"]);
    assert!(config.append);
}

#[test]
fn long_append() {
    let config = parse(&["--append"]);
    assert!(config.append);
}

#[test]
fn positional_files() {
    let config = parse(&["file1.txt", "file2.txt"]);
    assert_eq!(config.files, vec!["file1.txt", "file2.txt"]);
}

#[test]
fn append_with_files() {
    let config = parse(&["-a", "out1.txt", "out2.txt"]);
    assert!(config.append);
    assert_eq!(config.files, vec!["out1.txt", "out2.txt"]);
}

#[test]
fn double_dash_stops_flags() {
    let config = parse(&["--", "-a"]);
    assert!(!config.append);
    assert_eq!(config.files, vec!["-a"]);
}

#[test]
fn help_returns_err() {
    assert!(TeeConfig::try_parse_from(["tee", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(TeeConfig::try_parse_from(["tee", "--version"]).is_err());
}

#[test]
fn multiple_files() {
    let config = parse(&["a.txt", "b.txt", "c.txt"]);
    assert_eq!(config.files, vec!["a.txt", "b.txt", "c.txt"]);
}

#[cfg(test)]
mod ops_tests {
    use std::io::Cursor;
    use tempfile::NamedTempFile;

    #[test]
    fn tee_writes_to_files() {
        let f1 = NamedTempFile::new().unwrap();
        let f2 = NamedTempFile::new().unwrap();
        let paths = vec![
            f1.path().to_str().unwrap().to_string(),
            f2.path().to_str().unwrap().to_string(),
        ];

        let data = b"hello world\n";
        let mut input = Cursor::new(data.as_slice());
        tee::ops::tee(&mut input, &paths, false).unwrap();

        assert_eq!(std::fs::read_to_string(f1.path()).unwrap(), "hello world\n");
        assert_eq!(std::fs::read_to_string(f2.path()).unwrap(), "hello world\n");
    }

    #[test]
    fn tee_append_mode() {
        let f1 = NamedTempFile::new().unwrap();
        let path = f1.path().to_str().unwrap().to_string();

        std::fs::write(&path, "first\n").unwrap();

        let data = b"second\n";
        let mut input = Cursor::new(data.as_slice());
        tee::ops::tee(&mut input, &[path.clone()], true).unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "first\nsecond\n");
    }

    #[test]
    fn tee_overwrite_mode() {
        let f1 = NamedTempFile::new().unwrap();
        let path = f1.path().to_str().unwrap().to_string();

        std::fs::write(&path, "old content\n").unwrap();

        let data = b"new\n";
        let mut input = Cursor::new(data.as_slice());
        tee::ops::tee(&mut input, &[path.clone()], false).unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new\n");
    }
}
