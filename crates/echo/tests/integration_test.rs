use std::process::Command;

fn echo(args: &[&str]) -> Vec<u8> {
    let output = Command::new(env!("CARGO_BIN_EXE_echo"))
        .args(args)
        .output()
        .expect("failed to run echo binary");
    assert!(output.status.success());
    output.stdout
}

fn echo_str(args: &[&str]) -> String {
    String::from_utf8(echo(args)).unwrap()
}

#[test]
fn no_args_prints_newline() {
    assert_eq!(echo(&[]), b"\n");
}

#[test]
fn simple_string() {
    assert_eq!(echo_str(&["hello"]), "hello\n");
}

#[test]
fn multiple_strings_space_separated() {
    assert_eq!(echo_str(&["hello", "world"]), "hello world\n");
}

#[test]
fn flag_n_suppresses_newline() {
    assert_eq!(echo_str(&["-n", "hello"]), "hello");
}

#[test]
fn flag_e_interprets_escapes() {
    assert_eq!(echo_str(&["-e", "hello\\nworld"]), "hello\nworld\n");
}

#[test]
fn flag_e_tab() {
    assert_eq!(echo_str(&["-e", "a\\tb"]), "a\tb\n");
}

#[test]
fn flag_e_backslash_c_stops() {
    assert_eq!(echo_str(&["-e", "hello\\c", "world"]), "hello");
}

#[test]
fn unknown_flag_is_literal() {
    assert_eq!(echo_str(&["-a"]), "-a\n");
}

#[test]
fn double_dash_is_literal() {
    assert_eq!(echo_str(&["--", "hello"]), "-- hello\n");
}

#[test]
fn empty_string_arg() {
    assert_eq!(echo_str(&[""]), "\n");
}

#[test]
fn help_flag() {
    let output = echo_str(&["--help"]);
    assert!(output.contains("Usage:"));
}

#[test]
fn version_flag() {
    let output = echo_str(&["--version"]);
    assert!(output.contains("echo"));
}
