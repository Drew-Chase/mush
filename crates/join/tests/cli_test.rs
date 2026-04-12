use std::io::Cursor;

use join::cli::JoinConfig;
use join::ops;

fn parse(args: &[&str]) -> JoinConfig {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    JoinConfig::from_args(&owned).expect("should not be --help/--version")
}

fn run_join(input1: &str, input2: &str, args: &[&str]) -> String {
    let mut full_args: Vec<&str> = args.to_vec();
    full_args.push("file1");
    full_args.push("file2");
    let config = parse(&full_args);

    let mut inp1 = Cursor::new(input1.as_bytes().to_vec());
    let mut inp2 = Cursor::new(input2.as_bytes().to_vec());
    let mut out = Vec::new();
    ops::join(&mut inp1, &mut inp2, &mut out, &config).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn default_join() {
    let result = run_join("a 1\nb 2\nc 3\n", "a x\nb y\n", &[]);
    assert_eq!(result, "a 1 x\nb 2 y\n");
}

#[test]
fn join_field1_2() {
    let result = run_join("1 a\n2 b\n", "a x\nb y\n", &["-1", "2"]);
    assert_eq!(result, "a 1 x\nb 2 y\n");
}

#[test]
fn join_field2_2() {
    let result = run_join("a 1\nb 2\n", "x a\ny b\n", &["-2", "2"]);
    assert_eq!(result, "a 1 x\nb 2 y\n");
}

#[test]
fn join_with_separator() {
    let result = run_join("a:1\nb:2\n", "a:x\nb:y\n", &["-t", ":"]);
    assert_eq!(result, "a:1:x\nb:2:y\n");
}

#[test]
fn join_unpaired1() {
    let result = run_join("a 1\nb 2\nc 3\n", "a x\nc z\n", &["-a", "1"]);
    assert_eq!(result, "a 1 x\nb 2\nc 3 z\n");
}

#[test]
fn join_only_unpaired1() {
    let result = run_join("a 1\nb 2\nc 3\n", "a x\nc z\n", &["-v", "1"]);
    assert_eq!(result, "b 2\n");
}

#[test]
fn join_empty_replacement() {
    let result = run_join(
        "a 1\nb 2\nc 3\n",
        "a x\nc z\n",
        &["-a", "1", "-e", "EMPTY", "-o", "0,1.2,2.2"],
    );
    assert_eq!(result, "a 1 x\nb 2 EMPTY\nc 3 z\n");
}

#[test]
fn join_ignore_case() {
    let result = run_join("A 1\nB 2\n", "a x\nb y\n", &["-i"]);
    assert_eq!(result, "A 1 x\nB 2 y\n");
}
