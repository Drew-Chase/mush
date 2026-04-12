use std::io::Cursor;

use clap::Parser;

use xargs::cli::XargsConfig;
use xargs::ops::{build_commands, read_items};

fn parse(args: &[&str]) -> XargsConfig {
    let mut full = vec!["xargs"];
    full.extend_from_slice(args);
    let mut config = XargsConfig::parse_from(full);
    // Default command is "echo"
    if config.command.is_empty() {
        config.command = vec!["echo".to_string()];
    }
    config
}

// --- CLI parsing tests ---

#[test]
fn defaults() {
    let config = parse(&[]);
    assert!(!config.null);
    assert!(config.delimiter.is_none());
    assert!(config.max_args.is_none());
    assert!(config.replace.is_none());
    assert!(config.max_lines.is_none());
    assert_eq!(config.max_procs, 1);
    assert!(!config.verbose);
    assert!(!config.interactive);
    assert!(!config.no_run_if_empty);
    assert!(config.max_chars.is_none());
    assert_eq!(config.command, vec!["echo"]);
}

#[test]
fn flag_0() {
    let config = parse(&["-0"]);
    assert!(config.null);
}

#[test]
fn flag_null_long() {
    let config = parse(&["--null"]);
    assert!(config.null);
}

#[test]
fn flag_d() {
    let config = parse(&["-d", ","]);
    assert_eq!(config.delimiter, Some(','));
}

#[test]
fn flag_delimiter_long() {
    let config = parse(&["--delimiter", ","]);
    assert_eq!(config.delimiter, Some(','));
}

#[test]
fn flag_delimiter_long_equals() {
    let config = parse(&["--delimiter=,"]);
    assert_eq!(config.delimiter, Some(','));
}

#[test]
fn flag_n() {
    let config = parse(&["-n", "3"]);
    assert_eq!(config.max_args, Some(3));
}

#[test]
fn flag_max_args_long() {
    let config = parse(&["--max-args", "5"]);
    assert_eq!(config.max_args, Some(5));
}

#[test]
fn flag_max_args_long_equals() {
    let config = parse(&["--max-args=5"]);
    assert_eq!(config.max_args, Some(5));
}

#[test]
fn flag_i_replace() {
    let config = parse(&["-I", "{}"]);
    assert_eq!(config.replace, Some("{}".to_string()));
}

#[test]
fn flag_replace_long() {
    let config = parse(&["--replace", "{}"]);
    assert_eq!(config.replace, Some("{}".to_string()));
}

#[test]
fn flag_replace_long_equals() {
    let config = parse(&["--replace={}"]);
    assert_eq!(config.replace, Some("{}".to_string()));
}

#[test]
fn flag_l() {
    let config = parse(&["-L", "2"]);
    assert_eq!(config.max_lines, Some(2));
}

#[test]
fn flag_max_lines_long() {
    let config = parse(&["--max-lines", "2"]);
    assert_eq!(config.max_lines, Some(2));
}

#[test]
fn flag_p_capital() {
    let config = parse(&["-P", "4"]);
    assert_eq!(config.max_procs, 4);
}

#[test]
fn flag_max_procs_long() {
    let config = parse(&["--max-procs", "8"]);
    assert_eq!(config.max_procs, 8);
}

#[test]
fn flag_max_procs_long_equals() {
    let config = parse(&["--max-procs=8"]);
    assert_eq!(config.max_procs, 8);
}

#[test]
fn flag_t() {
    let config = parse(&["-t"]);
    assert!(config.verbose);
}

#[test]
fn flag_verbose_long() {
    let config = parse(&["--verbose"]);
    assert!(config.verbose);
}

#[test]
fn flag_p_interactive() {
    let config = parse(&["-p"]);
    assert!(config.interactive);
}

#[test]
fn flag_interactive_long() {
    let config = parse(&["--interactive"]);
    assert!(config.interactive);
}

#[test]
fn flag_r() {
    let config = parse(&["-r"]);
    assert!(config.no_run_if_empty);
}

#[test]
fn flag_no_run_if_empty_long() {
    let config = parse(&["--no-run-if-empty"]);
    assert!(config.no_run_if_empty);
}

#[test]
fn flag_s() {
    let config = parse(&["-s", "1000"]);
    assert_eq!(config.max_chars, Some(1000));
}

#[test]
fn flag_max_chars_long() {
    let config = parse(&["--max-chars", "500"]);
    assert_eq!(config.max_chars, Some(500));
}

#[test]
fn flag_max_chars_long_equals() {
    let config = parse(&["--max-chars=500"]);
    assert_eq!(config.max_chars, Some(500));
}

#[test]
fn custom_command() {
    let config = parse(&["ls", "-la"]);
    assert_eq!(config.command, vec!["ls", "-la"]);
}

#[test]
fn default_echo() {
    let config = parse(&[]);
    assert_eq!(config.command, vec!["echo"]);
}

#[test]
fn combined_flags() {
    let config = parse(&["-0tr"]);
    assert!(config.null);
    assert!(config.verbose);
    assert!(config.no_run_if_empty);
}

// --- read_items tests ---

#[test]
fn read_whitespace_items() {
    let config = XargsConfig::default();
    let input = "hello  world\nfoo  bar\n";
    let mut cursor = Cursor::new(input.as_bytes());
    let items = read_items(&mut cursor, &config);
    assert_eq!(items, vec!["hello", "world", "foo", "bar"]);
}

#[test]
fn read_null_items() {
    let config = XargsConfig {
        null: true,
        ..Default::default()
    };
    let input = b"hello\0world\0foo\0";
    let mut cursor = Cursor::new(input.as_ref());
    let items = read_items(&mut cursor, &config);
    assert_eq!(items, vec!["hello", "world", "foo"]);
}

#[test]
fn read_custom_delimiter() {
    let config = XargsConfig {
        delimiter: Some(','),
        ..Default::default()
    };
    let input = "a,b,c";
    let mut cursor = Cursor::new(input.as_bytes());
    let items = read_items(&mut cursor, &config);
    assert_eq!(items, vec!["a", "b", "c"]);
}

// --- build_commands tests ---

#[test]
fn build_default_single_command() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        ..Default::default()
    };
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let cmds = build_commands(&items, &config);
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0], vec!["echo", "a", "b", "c"]);
}

#[test]
fn build_max_args() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        max_args: Some(2),
        ..Default::default()
    };
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let cmds = build_commands(&items, &config);
    assert_eq!(cmds.len(), 2);
    assert_eq!(cmds[0], vec!["echo", "a", "b"]);
    assert_eq!(cmds[1], vec!["echo", "c"]);
}

#[test]
fn build_replace() {
    let config = XargsConfig {
        command: vec!["cp".to_string(), "{}".to_string(), "/dest/{}".to_string()],
        replace: Some("{}".to_string()),
        ..Default::default()
    };
    let items = vec!["file1".to_string(), "file2".to_string()];
    let cmds = build_commands(&items, &config);
    assert_eq!(cmds.len(), 2);
    assert_eq!(cmds[0], vec!["cp", "file1", "/dest/file1"]);
    assert_eq!(cmds[1], vec!["cp", "file2", "/dest/file2"]);
}

#[test]
fn build_max_lines() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        max_lines: Some(1),
        ..Default::default()
    };
    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let cmds = build_commands(&items, &config);
    assert_eq!(cmds.len(), 3);
    assert_eq!(cmds[0], vec!["echo", "a"]);
    assert_eq!(cmds[1], vec!["echo", "b"]);
    assert_eq!(cmds[2], vec!["echo", "c"]);
}

#[test]
fn build_max_chars() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        max_chars: Some(10),
        ..Default::default()
    };
    let items = vec!["hello".to_string(), "world".to_string(), "foo".to_string()];
    let cmds = build_commands(&items, &config);
    assert_eq!(cmds.len(), 1);
    // "echo hello" = 10 chars, "world" would make it 16, so truncated
    assert_eq!(cmds[0], vec!["echo", "hello"]);
}

#[test]
fn build_empty_items() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        ..Default::default()
    };
    let items: Vec<String> = Vec::new();
    let cmds = build_commands(&items, &config);
    assert!(cmds.is_empty());
}

#[test]
fn build_no_run_if_empty() {
    let config = XargsConfig {
        command: vec!["echo".to_string()],
        no_run_if_empty: true,
        ..Default::default()
    };
    let items: Vec<String> = Vec::new();
    let cmds = build_commands(&items, &config);
    assert!(cmds.is_empty());
}
