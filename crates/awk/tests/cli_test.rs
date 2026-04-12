use std::io::BufReader;

use clap::Parser;

use awk::cli::AwkConfig;
use awk::ops::run;

fn parse(args: &[&str]) -> AwkConfig {
    let mut full = vec!["awk"];
    full.extend_from_slice(args);
    AwkConfig::parse_from(full)
}

fn run_awk(program: &str, input: &str) -> String {
    let config = AwkConfig {
        positionals: vec![program.to_string()],
        ..Default::default()
    };
    let mut reader = BufReader::new(input.as_bytes());
    let mut output = Vec::new();
    run(program, &config, &mut reader, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

fn run_awk_config(config: &AwkConfig, input: &str) -> String {
    let program = config.program().unwrap_or_default().to_string();
    let mut reader = BufReader::new(input.as_bytes());
    let mut output = Vec::new();
    run(&program, config, &mut reader, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

// ---------------------------------------------------------------------------
// CLI parsing tests
// ---------------------------------------------------------------------------

#[test]
fn default_program_parsing() {
    let config = parse(&["{print $1}"]);
    assert_eq!(config.program(), Some("{print $1}"));
    assert!(config.files().is_empty());
}

#[test]
fn program_with_files() {
    let config = parse(&["{print $1}", "file1.txt", "file2.txt"]);
    assert_eq!(config.program(), Some("{print $1}"));
    assert_eq!(config.files(), &["file1.txt", "file2.txt"]);
}

#[test]
fn field_separator_short_separate() {
    let config = parse(&["-F", ":", "{print $1}"]);
    assert_eq!(config.field_separator, ":");
    assert_eq!(config.program(), Some("{print $1}"));
}

#[test]
fn field_separator_long() {
    let config = parse(&["--field-separator=:", "{print $1}"]);
    assert_eq!(config.field_separator, ":");
}

#[test]
fn field_separator_long_separate() {
    let config = parse(&["--field-separator", ":", "{print $1}"]);
    assert_eq!(config.field_separator, ":");
}

#[test]
fn variable_assignment_short() {
    let config = parse(&["-v", "OFS=,", "{print $1, $2}"]);
    assert_eq!(config.variables, vec!["OFS=,"]);
}

#[test]
fn variable_assignment_long() {
    let config = parse(&["--assign=OFS=,", "{print $1, $2}"]);
    assert_eq!(config.variables, vec!["OFS=,"]);
}

#[test]
fn variable_assignment_long_separate() {
    let config = parse(&["--assign", "OFS=,", "{print $1, $2}"]);
    assert_eq!(config.variables, vec!["OFS=,"]);
}

#[test]
fn multiple_variables() {
    let config = parse(&["-v", "A=1", "-v", "B=2", "{print}"]);
    assert_eq!(config.variables.len(), 2);
    assert_eq!(config.variables[0], "A=1");
    assert_eq!(config.variables[1], "B=2");
}

#[test]
fn program_file_short() {
    let config = parse(&["-f", "prog.awk", "data.txt"]);
    assert_eq!(config.program_file, Some("prog.awk".to_string()));
    assert_eq!(config.files(), &["data.txt"]);
}

#[test]
fn program_file_long() {
    let config = parse(&["--file=prog.awk", "data.txt"]);
    assert_eq!(config.program_file, Some("prog.awk".to_string()));
}

#[test]
fn program_file_long_separate() {
    let config = parse(&["--file", "prog.awk", "data.txt"]);
    assert_eq!(config.program_file, Some("prog.awk".to_string()));
}

#[test]
fn stdin_dash() {
    let config = parse(&["{print}", "-"]);
    assert_eq!(config.files(), &["-"]);
}

// ---------------------------------------------------------------------------
// Interpreter tests
// ---------------------------------------------------------------------------

#[test]
fn print_entire_line() {
    let out = run_awk("{print}", "hello world\n");
    assert_eq!(out, "hello world\n");
}

#[test]
fn print_first_field() {
    let out = run_awk("{print $1}", "hello world\n");
    assert_eq!(out, "hello\n");
}

#[test]
fn print_second_field() {
    let out = run_awk("{print $2}", "hello world\n");
    assert_eq!(out, "world\n");
}

#[test]
fn print_multiple_fields() {
    let out = run_awk("{print $1, $2}", "hello world\n");
    assert_eq!(out, "hello world\n");
}

#[test]
fn field_separator_colon() {
    let config = AwkConfig {
        positionals: vec!["{print $1}".to_string()],
        field_separator: ":".to_string(),
        ..Default::default()
    };
    let out = run_awk_config(&config, "root:x:0:0\n");
    assert_eq!(out, "root\n");
}

#[test]
fn custom_ofs() {
    let config = AwkConfig {
        positionals: vec!["{print $1, $2}".to_string()],
        variables: vec!["OFS=,".to_string()],
        ..Default::default()
    };
    let out = run_awk_config(&config, "hello world\n");
    assert_eq!(out, "hello,world\n");
}

#[test]
fn begin_block() {
    let out = run_awk("BEGIN {print \"header\"}", "");
    assert_eq!(out, "header\n");
}

#[test]
fn end_block() {
    let out = run_awk("END {print \"done\"}", "line1\nline2\n");
    assert_eq!(out, "done\n");
}

#[test]
fn begin_and_end() {
    let out = run_awk(
        "BEGIN {print \"start\"} {print} END {print \"end\"}",
        "middle\n",
    );
    assert_eq!(out, "start\nmiddle\nend\n");
}

#[test]
fn nr_variable() {
    let out = run_awk("{print NR, $0}", "a\nb\nc\n");
    assert_eq!(out, "1 a\n2 b\n3 c\n");
}

#[test]
fn nf_variable() {
    let out = run_awk("{print NF}", "a b c\nd e\nf\n");
    assert_eq!(out, "3\n2\n1\n");
}

#[test]
fn pattern_matching_regex() {
    let out = run_awk("/hello/ {print}", "hello world\ngoodbye world\nhello again\n");
    assert_eq!(out, "hello world\nhello again\n");
}

#[test]
fn pattern_matching_comparison() {
    let out = run_awk("$1 > 5 {print}", "3\n7\n1\n10\n");
    assert_eq!(out, "7\n10\n");
}

#[test]
fn arithmetic() {
    let out = run_awk("{print $1 + $2}", "3 4\n10 20\n");
    assert_eq!(out, "7\n30\n");
}

#[test]
fn string_concatenation() {
    let out = run_awk("{print $1 $2}", "hello world\n");
    assert_eq!(out, "helloworld\n");
}

#[test]
fn variable_assignment() {
    let out = run_awk("{x = $1 + $2; print x}", "3 4\n");
    assert_eq!(out, "7\n");
}

#[test]
fn if_statement() {
    let out = run_awk("{if ($1 > 5) print \"big\"; else print \"small\"}", "3\n7\n");
    assert_eq!(out, "small\nbig\n");
}

#[test]
fn while_loop() {
    let out = run_awk("BEGIN {i = 1; while (i <= 3) {print i; i++}}", "");
    assert_eq!(out, "1\n2\n3\n");
}

#[test]
fn for_loop() {
    let out = run_awk("BEGIN {for (i = 1; i <= 3; i++) print i}", "");
    assert_eq!(out, "1\n2\n3\n");
}

#[test]
fn length_function() {
    let out = run_awk("{print length($0)}", "hello\n");
    assert_eq!(out, "5\n");
}

#[test]
fn substr_function() {
    let out = run_awk("{print substr($0, 2, 3)}", "hello\n");
    assert_eq!(out, "ell\n");
}

#[test]
fn index_function() {
    let out = run_awk("{print index($0, \"lo\")}", "hello\n");
    assert_eq!(out, "4\n");
}

#[test]
fn tolower_function() {
    let out = run_awk("{print tolower($0)}", "HELLO\n");
    assert_eq!(out, "hello\n");
}

#[test]
fn toupper_function() {
    let out = run_awk("{print toupper($0)}", "hello\n");
    assert_eq!(out, "HELLO\n");
}

#[test]
fn next_statement() {
    let out = run_awk("{if ($1 == \"skip\") next; print}", "keep\nskip\nalso keep\n");
    assert_eq!(out, "keep\nalso keep\n");
}

#[test]
fn exit_statement() {
    let out = run_awk("{if (NR == 2) exit; print}", "a\nb\nc\n");
    assert_eq!(out, "a\n");
}

#[test]
fn printf_statement() {
    let out = run_awk("{printf \"%s=%d\\n\", $1, $2}", "x 42\n");
    assert_eq!(out, "x=42\n");
}

#[test]
fn compound_assignment() {
    let out = run_awk("BEGIN {x = 10; x += 5; print x}", "");
    assert_eq!(out, "15\n");
}

#[test]
fn pattern_without_action() {
    let out = run_awk("/yes/", "yes\nno\nyes again\n");
    assert_eq!(out, "yes\nyes again\n");
}

#[test]
fn multiple_rules() {
    let out = run_awk("/a/ {print \"has a\"} /b/ {print \"has b\"}", "ab\na\nb\nc\n");
    assert_eq!(out, "has a\nhas b\nhas a\nhas b\n");
}

#[test]
fn field_assignment() {
    let out = run_awk("{$2 = \"X\"; print}", "a b c\n");
    assert_eq!(out, "a X c\n");
}

#[test]
fn end_with_nr() {
    let out = run_awk("END {print NR}", "a\nb\nc\n");
    assert_eq!(out, "3\n");
}

#[test]
fn accumulator_pattern() {
    let out = run_awk("{sum += $1} END {print sum}", "10\n20\n30\n");
    assert_eq!(out, "60\n");
}

#[test]
fn gsub_function() {
    let out = run_awk("{gsub(/o/, \"0\"); print}", "foobar\n");
    assert_eq!(out, "f00bar\n");
}

#[test]
fn sub_function() {
    let out = run_awk("{sub(/o/, \"0\"); print}", "foobar\n");
    assert_eq!(out, "f0obar\n");
}

#[test]
fn sprintf_function() {
    let out = run_awk("{print sprintf(\"[%05d]\", $1)}", "42\n");
    assert_eq!(out, "[00042]\n");
}

#[test]
fn multiline_input() {
    let out = run_awk("{print NR \": \" $0}", "alpha\nbeta\ngamma\n");
    assert_eq!(out, "1: alpha\n2: beta\n3: gamma\n");
}

#[test]
fn empty_input() {
    let out = run_awk("{print}", "");
    assert_eq!(out, "");
}

#[test]
fn begin_only() {
    let out = run_awk("BEGIN {print 2 + 3}", "ignored\n");
    assert_eq!(out, "5\n");
}

#[test]
fn match_operator() {
    let out = run_awk("$0 ~ /^[0-9]/ {print}", "1abc\nabc\n2def\n");
    assert_eq!(out, "1abc\n2def\n");
}

#[test]
fn not_match_operator() {
    let out = run_awk("$0 !~ /^#/ {print}", "#comment\ndata\n#another\nmore\n");
    assert_eq!(out, "data\nmore\n");
}

#[test]
fn program_file_flag() {
    use std::io::Write;
    let dir = tempfile::tempdir().unwrap();
    let prog_path = dir.path().join("test.awk");
    let mut f = std::fs::File::create(&prog_path).unwrap();
    write!(f, "{{print $1}}").unwrap();
    drop(f);

    let config = AwkConfig {
        program_file: Some(prog_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let program = std::fs::read_to_string(prog_path).unwrap();
    let mut reader = BufReader::new("hello world\n".as_bytes());
    let mut output = Vec::new();
    run(&program, &config, &mut reader, &mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "hello\n");
}
