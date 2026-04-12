use std::io::Cursor;

use clap::Parser;

use column::cli::ColumnConfig;
use column::ops::column;

fn parse(args: &[&str]) -> ColumnConfig {
    let mut full = vec!["column"];
    full.extend_from_slice(args);
    ColumnConfig::parse_from(full)
}

fn run(args: &[&str], input: &str) -> String {
    let config = parse(args);
    let mut input_cursor = Cursor::new(input.as_bytes().to_vec());
    let mut output = Vec::new();
    column(&mut input_cursor, &mut output, &config).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn default_columns() {
    let input = "alpha\nbeta\ngamma\ndelta\nepsilon\nzeta";
    let result = run(&["-c", "80"], input);
    // Should arrange into columns fitting 80 width
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    // With short words and 80 width, should use multiple columns
    assert!(lines.len() < 6, "should be fewer lines than input entries");
}

#[test]
fn table_mode() {
    let input = "name age city\nalice 30 nyc\nbob 25 la";
    let result = run(&["-t"], input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 3);
    // Columns should be aligned
    assert!(lines[0].contains("name"));
    assert!(lines[1].contains("alice"));
}

#[test]
fn table_with_separator() {
    let input = "name:age:city\nalice:30:nyc\nbob:25:la";
    let result = run(&["-t", "-s", ":"], input);
    let lines: Vec<&str> = result.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("name"));
    assert!(lines[1].contains("alice"));
}

#[test]
fn table_with_output_separator() {
    let input = "a b\nc d";
    let result = run(&["-t", "-o", " | "], input);
    assert!(result.contains(" | "), "should use custom output separator");
}

#[test]
fn json_output() {
    let input = "alice 30\nbob 25";
    let result = run(&["-J"], input);
    assert!(result.starts_with('['));
    assert!(result.contains("\"column1\""));
    assert!(result.contains("\"alice\""));
    assert!(result.contains("\"bob\""));
}

#[test]
fn json_with_column_names() {
    let input = "alice 30\nbob 25";
    let result = run(&["-J", "-N", "name,age"], input);
    assert!(result.contains("\"name\""));
    assert!(result.contains("\"age\""));
    assert!(result.contains("\"alice\""));
    assert!(result.contains("\"30\""));
}

#[test]
fn help_is_err() {
    assert!(ColumnConfig::try_parse_from(["column", "--help"]).is_err());
}

#[test]
fn version_is_err() {
    assert!(ColumnConfig::try_parse_from(["column", "--version"]).is_err());
}

#[test]
fn empty_input() {
    let result = run(&["-t"], "");
    assert_eq!(result, "");
}
