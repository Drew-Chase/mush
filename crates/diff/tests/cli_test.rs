use clap::Parser;

use diff::cli::DiffConfig;
use diff::ops::{compute_diff, format_normal, format_side_by_side, format_unified, DiffLine};

fn parse(args: &[&str]) -> DiffConfig {
    let mut full = vec!["diff"];
    full.extend_from_slice(args);
    DiffConfig::parse_from(full)
}

// --- CLI parsing tests ---

#[test]
fn defaults() {
    let config = parse(&["file1.txt", "file2.txt"]);
    assert_eq!(config.file1, "file1.txt");
    assert_eq!(config.file2, "file2.txt");
    assert!(config.unified.is_none());
    assert!(config.context.is_none());
    assert!(!config.side_by_side);
    assert_eq!(config.width, 130);
    assert!(!config.ignore_case);
    assert!(!config.ignore_space_change);
    assert!(!config.ignore_all_space);
    assert!(!config.ignore_blank_lines);
    assert!(!config.recursive);
    assert!(!config.brief);
    assert!(!config.report_identical);
    assert!(!config.color);
}

#[test]
fn flag_u_no_num() {
    let config = parse(&["-u", "a", "b"]);
    assert_eq!(config.unified, Some(3));
}

#[test]
fn flag_unified_long() {
    let config = parse(&["--unified", "a", "b"]);
    assert_eq!(config.unified, Some(3));
}

#[test]
fn flag_unified_long_equals() {
    let config = parse(&["--unified=5", "a", "b"]);
    assert_eq!(config.unified, Some(5));
}

#[test]
fn flag_c_no_num() {
    let config = parse(&["-c", "a", "b"]);
    assert_eq!(config.context, Some(3));
}

#[test]
fn flag_context_long_equals() {
    let config = parse(&["--context=7", "a", "b"]);
    assert_eq!(config.context, Some(7));
}

#[test]
fn flag_y() {
    let config = parse(&["-y", "a", "b"]);
    assert!(config.side_by_side);
}

#[test]
fn flag_side_by_side_long() {
    let config = parse(&["--side-by-side", "a", "b"]);
    assert!(config.side_by_side);
}

#[test]
fn flag_width_short() {
    let config = parse(&["-W", "80", "a", "b"]);
    assert_eq!(config.width, 80);
}

#[test]
fn flag_width_long() {
    let config = parse(&["--width", "80", "a", "b"]);
    assert_eq!(config.width, 80);
}

#[test]
fn flag_width_long_equals() {
    let config = parse(&["--width=80", "a", "b"]);
    assert_eq!(config.width, 80);
}

#[test]
fn flag_i() {
    let config = parse(&["-i", "a", "b"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_ignore_case_long() {
    let config = parse(&["--ignore-case", "a", "b"]);
    assert!(config.ignore_case);
}

#[test]
fn flag_b() {
    let config = parse(&["-b", "a", "b_file"]);
    assert!(config.ignore_space_change);
}

#[test]
fn flag_ignore_space_change_long() {
    let config = parse(&["--ignore-space-change", "a", "b"]);
    assert!(config.ignore_space_change);
}

#[test]
fn flag_w() {
    let config = parse(&["-w", "a", "b"]);
    assert!(config.ignore_all_space);
}

#[test]
fn flag_ignore_all_space_long() {
    let config = parse(&["--ignore-all-space", "a", "b"]);
    assert!(config.ignore_all_space);
}

#[test]
fn flag_capital_b() {
    let config = parse(&["-B", "a", "b"]);
    assert!(config.ignore_blank_lines);
}

#[test]
fn flag_ignore_blank_lines_long() {
    let config = parse(&["--ignore-blank-lines", "a", "b"]);
    assert!(config.ignore_blank_lines);
}

#[test]
fn flag_r() {
    let config = parse(&["-r", "a", "b"]);
    assert!(config.recursive);
}

#[test]
fn flag_recursive_long() {
    let config = parse(&["--recursive", "a", "b"]);
    assert!(config.recursive);
}

#[test]
fn flag_q() {
    let config = parse(&["-q", "a", "b"]);
    assert!(config.brief);
}

#[test]
fn flag_brief_long() {
    let config = parse(&["--brief", "a", "b"]);
    assert!(config.brief);
}

#[test]
fn flag_s() {
    let config = parse(&["-s", "a", "b"]);
    assert!(config.report_identical);
}

#[test]
fn flag_report_identical_long() {
    let config = parse(&["--report-identical-files", "a", "b"]);
    assert!(config.report_identical);
}

#[test]
fn flag_color() {
    let config = parse(&["--color", "a", "b"]);
    assert!(config.color);
}

// --- Diff computation tests ---

#[test]
fn identical_files() {
    let lines = vec!["a", "b", "c"];
    let config = DiffConfig::default();
    let hunks = compute_diff(&lines, &lines, &config);
    assert!(hunks.is_empty());
}

#[test]
fn simple_addition() {
    let lines1 = vec!["a", "c"];
    let lines2 = vec!["a", "b", "c"];
    let config = DiffConfig::default();
    let hunks = compute_diff(&lines1, &lines2, &config);
    assert_eq!(hunks.len(), 1);
    assert!(hunks[0].changes.iter().any(|c| matches!(c, DiffLine::Added(s) if s == "b")));
}

#[test]
fn simple_removal() {
    let lines1 = vec!["a", "b", "c"];
    let lines2 = vec!["a", "c"];
    let config = DiffConfig::default();
    let hunks = compute_diff(&lines1, &lines2, &config);
    assert_eq!(hunks.len(), 1);
    assert!(hunks[0].changes.iter().any(|c| matches!(c, DiffLine::Removed(s) if s == "b")));
}

#[test]
fn ignore_case_diff() {
    let lines1 = vec!["Hello", "World"];
    let lines2 = vec!["hello", "world"];
    let config = DiffConfig {
        ignore_case: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    assert!(hunks.is_empty());
}

#[test]
fn ignore_space_change_diff() {
    let lines1 = vec!["a  b", "c"];
    let lines2 = vec!["a b", "c"];
    let config = DiffConfig {
        ignore_space_change: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    assert!(hunks.is_empty());
}

#[test]
fn ignore_all_space_diff() {
    let lines1 = vec!["a b c"];
    let lines2 = vec!["abc"];
    let config = DiffConfig {
        ignore_all_space: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    assert!(hunks.is_empty());
}

// --- Format tests ---

#[test]
fn unified_format() {
    let lines1 = vec!["a", "b", "c"];
    let lines2 = vec!["a", "x", "c"];
    let config = DiffConfig {
        unified: Some(3),
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    let output = format_unified(&hunks, "file1", "file2", false);
    assert!(output.iter().any(|l| l.starts_with("--- a/")));
    assert!(output.iter().any(|l| l.starts_with("+++ b/")));
    assert!(output.iter().any(|l| l.starts_with("@@")));
    assert!(output.iter().any(|l| l == "-b"));
    assert!(output.iter().any(|l| l == "+x"));
}

#[test]
fn normal_format() {
    let lines1 = vec!["a", "b", "c"];
    let lines2 = vec!["a", "x", "c"];
    let config = DiffConfig::default();
    let hunks = compute_diff(&lines1, &lines2, &config);
    let output = format_normal(&hunks);
    assert!(output.iter().any(|l| l.contains('c') && l.contains('2')));
    assert!(output.iter().any(|l| l == "< b"));
    assert!(output.iter().any(|l| l == "> x"));
}

#[test]
fn side_by_side_format() {
    let lines1 = vec!["a", "b", "c"];
    let lines2 = vec!["a", "x", "c"];
    let config = DiffConfig {
        side_by_side: true,
        width: 80,
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    let output = format_side_by_side(&hunks, &lines1, &lines2, 80);
    assert!(!output.is_empty());
    // Should have a change marker
    assert!(output.iter().any(|l| l.contains(" | ")));
}

#[test]
fn color_unified_format() {
    let lines1 = vec!["a"];
    let lines2 = vec!["b"];
    let config = DiffConfig {
        unified: Some(3),
        color: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines1, &lines2, &config);
    let output = format_unified(&hunks, "f1", "f2", true);
    assert!(output.iter().any(|l| l.contains("\x1b[31m")));
    assert!(output.iter().any(|l| l.contains("\x1b[32m")));
    assert!(output.iter().any(|l| l.contains("\x1b[36m")));
}

#[test]
fn brief_identical() {
    let lines = vec!["a", "b"];
    let config = DiffConfig {
        brief: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines, &lines, &config);
    assert!(hunks.is_empty());
}

#[test]
fn report_identical_files() {
    let lines = vec!["a", "b"];
    let config = DiffConfig {
        report_identical: true,
        ..Default::default()
    };
    let hunks = compute_diff(&lines, &lines, &config);
    assert!(hunks.is_empty());
}
