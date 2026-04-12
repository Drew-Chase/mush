use patch::cli::PatchConfig;
use patch::ops::{apply_hunk, apply_patch_to_string, parse_patch, strip_path, DiffLine, Hunk};

fn parse(args: &[&str]) -> PatchConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    PatchConfig::from_args(&owned).expect("should not be --help/--version")
}

#[test]
fn default_config() {
    let config = parse(&[]);
    assert_eq!(config.strip, 0);
    assert!(!config.reverse);
    assert!(!config.dry_run);
    assert!(!config.backup);
}

#[test]
fn parse_strip() {
    let config = parse(&["-p1"]);
    assert_eq!(config.strip, 1);
}

#[test]
fn parse_strip_separate() {
    let config = parse(&["-p", "2"]);
    assert_eq!(config.strip, 2);
}

#[test]
fn parse_reverse() {
    let config = parse(&["-R"]);
    assert!(config.reverse);
}

#[test]
fn parse_dry_run() {
    let config = parse(&["--dry-run"]);
    assert!(config.dry_run);
}

#[test]
fn parse_backup() {
    let config = parse(&["-b"]);
    assert!(config.backup);
}

#[test]
fn parse_input_file() {
    let config = parse(&["-i", "fix.patch"]);
    assert_eq!(config.patch_file, Some("fix.patch".to_string()));
}

#[test]
fn help_returns_none() {
    let owned = vec!["--help".to_string()];
    assert!(PatchConfig::from_args(&owned).is_none());
}

#[test]
fn version_returns_none() {
    let owned = vec!["--version".to_string()];
    assert!(PatchConfig::from_args(&owned).is_none());
}

#[test]
fn strip_path_none() {
    assert_eq!(strip_path("file.txt", 0), std::path::PathBuf::from("file.txt"));
}

#[test]
fn strip_path_one() {
    assert_eq!(strip_path("a/b/file.txt", 1), std::path::PathBuf::from("b/file.txt"));
}

#[test]
fn strip_path_two() {
    assert_eq!(strip_path("a/b/file.txt", 2), std::path::PathBuf::from("file.txt"));
}

#[test]
fn strip_path_excess() {
    assert_eq!(strip_path("a/file.txt", 5), std::path::PathBuf::from("file.txt"));
}

#[test]
fn parse_unified_diff() {
    let diff = "\
--- a/hello.txt
+++ b/hello.txt
@@ -1,3 +1,3 @@
 line1
-line2
+LINE2
 line3
";
    let mut cursor = std::io::Cursor::new(diff.as_bytes());
    let patches = parse_patch(&mut cursor).unwrap();
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].hunks.len(), 1);
    assert_eq!(patches[0].hunks[0].old_start, 1);
    assert_eq!(patches[0].hunks[0].old_count, 3);
    assert_eq!(patches[0].hunks[0].new_start, 1);
    assert_eq!(patches[0].hunks[0].new_count, 3);
}

#[test]
fn apply_hunk_replace() {
    let original = vec![
        "line1".to_string(),
        "line2".to_string(),
        "line3".to_string(),
    ];
    let hunk = Hunk {
        old_start: 1,
        old_count: 3,
        new_start: 1,
        new_count: 3,
        lines: vec![
            DiffLine::Context("line1".to_string()),
            DiffLine::Remove("line2".to_string()),
            DiffLine::Add("LINE2".to_string()),
            DiffLine::Context("line3".to_string()),
        ],
    };
    let result = apply_hunk(&original, &hunk, false).unwrap();
    assert_eq!(result, vec!["line1", "LINE2", "line3"]);
}

#[test]
fn apply_hunk_reverse() {
    let modified = vec![
        "line1".to_string(),
        "LINE2".to_string(),
        "line3".to_string(),
    ];
    let hunk = Hunk {
        old_start: 1,
        old_count: 3,
        new_start: 1,
        new_count: 3,
        lines: vec![
            DiffLine::Context("line1".to_string()),
            DiffLine::Remove("line2".to_string()),
            DiffLine::Add("LINE2".to_string()),
            DiffLine::Context("line3".to_string()),
        ],
    };
    let result = apply_hunk(&modified, &hunk, true).unwrap();
    assert_eq!(result, vec!["line1", "line2", "line3"]);
}

#[test]
fn apply_hunk_add_lines() {
    let original = vec![
        "line1".to_string(),
        "line3".to_string(),
    ];
    let hunk = Hunk {
        old_start: 1,
        old_count: 2,
        new_start: 1,
        new_count: 3,
        lines: vec![
            DiffLine::Context("line1".to_string()),
            DiffLine::Add("line2".to_string()),
            DiffLine::Context("line3".to_string()),
        ],
    };
    let result = apply_hunk(&original, &hunk, false).unwrap();
    assert_eq!(result, vec!["line1", "line2", "line3"]);
}

#[test]
fn apply_hunk_remove_lines() {
    let original = vec![
        "line1".to_string(),
        "line2".to_string(),
        "line3".to_string(),
    ];
    let hunk = Hunk {
        old_start: 1,
        old_count: 3,
        new_start: 1,
        new_count: 2,
        lines: vec![
            DiffLine::Context("line1".to_string()),
            DiffLine::Remove("line2".to_string()),
            DiffLine::Context("line3".to_string()),
        ],
    };
    let result = apply_hunk(&original, &hunk, false).unwrap();
    assert_eq!(result, vec!["line1", "line3"]);
}

#[test]
fn apply_patch_to_string_basic() {
    let original = "line1\nline2\nline3";
    let diff = "\
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,3 @@
 line1
-line2
+LINE2
 line3
";
    let result = apply_patch_to_string(original, diff, false).unwrap();
    assert_eq!(result, "line1\nLINE2\nline3");
}

#[test]
fn apply_patch_to_string_reverse() {
    let modified = "line1\nLINE2\nline3";
    let diff = "\
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,3 @@
 line1
-line2
+LINE2
 line3
";
    let result = apply_patch_to_string(modified, diff, true).unwrap();
    assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn context_mismatch_error() {
    let original = vec![
        "wrong".to_string(),
        "line2".to_string(),
    ];
    let hunk = Hunk {
        old_start: 1,
        old_count: 2,
        new_start: 1,
        new_count: 2,
        lines: vec![
            DiffLine::Context("line1".to_string()),
            DiffLine::Context("line2".to_string()),
        ],
    };
    assert!(apply_hunk(&original, &hunk, false).is_err());
}
