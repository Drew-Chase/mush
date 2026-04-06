use echo::escape::process_escapes;

fn escaped(input: &str) -> String {
    let (output, _) = process_escapes(input);
    output
}

fn stops(input: &str) -> bool {
    let (_, stop) = process_escapes(input);
    stop
}

#[test]
fn no_escapes() {
    assert_eq!(escaped("hello world"), "hello world");
}

#[test]
fn empty_string() {
    assert_eq!(escaped(""), "");
    assert!(!stops(""));
}

#[test]
fn backslash_backslash() {
    assert_eq!(escaped("path\\\\file"), "path\\file");
}

#[test]
fn backslash_a_bel() {
    assert_eq!(escaped("\\a"), "\x07");
}

#[test]
fn backslash_b_backspace() {
    assert_eq!(escaped("\\b"), "\x08");
}

#[test]
fn backslash_e_escape() {
    assert_eq!(escaped("\\e"), "\x1B");
}

#[test]
fn backslash_f_formfeed() {
    assert_eq!(escaped("\\f"), "\x0C");
}

#[test]
fn backslash_n_newline() {
    assert_eq!(escaped("hello\\nworld"), "hello\nworld");
}

#[test]
fn backslash_r_carriage_return() {
    assert_eq!(escaped("\\r"), "\r");
}

#[test]
fn backslash_t_tab() {
    assert_eq!(escaped("col1\\tcol2"), "col1\tcol2");
}

#[test]
fn backslash_v_vertical_tab() {
    assert_eq!(escaped("\\v"), "\x0B");
}

#[test]
fn backslash_c_stops_output() {
    let (output, stop) = process_escapes("hello\\cworld");
    assert_eq!(output, "hello");
    assert!(stop);
}

#[test]
fn backslash_c_at_end() {
    let (output, stop) = process_escapes("hello\\c");
    assert_eq!(output, "hello");
    assert!(stop);
}

#[test]
fn backslash_c_middle_of_text() {
    let (output, stop) = process_escapes("a\\cb\\nc");
    assert_eq!(output, "a");
    assert!(stop);
}

#[test]
fn octal_zero() {
    assert_eq!(escaped("\\0"), "\0");
}

#[test]
fn octal_101_is_a() {
    assert_eq!(escaped("\\0101"), "A");
}

#[test]
fn octal_012_is_newline() {
    assert_eq!(escaped("\\012"), "\n");
}

#[test]
fn octal_max_377() {
    let result = escaped("\\0377");
    assert_eq!(result, "\u{FF}");
}

#[test]
fn octal_partial_one_digit() {
    assert_eq!(escaped("\\07"), "\x07");
}

#[test]
fn hex_41_is_a() {
    assert_eq!(escaped("\\x41"), "A");
}

#[test]
fn hex_0a_is_newline() {
    assert_eq!(escaped("\\x0a"), "\n");
}

#[test]
fn hex_uppercase() {
    assert_eq!(escaped("\\x4F"), "O");
}

#[test]
fn hex_single_digit() {
    assert_eq!(escaped("\\x9"), "\x09");
}

#[test]
fn hex_no_digits_literal() {
    assert_eq!(escaped("\\x"), "\\x");
}

#[test]
fn unknown_escape_literal() {
    assert_eq!(escaped("\\z"), "\\z");
}

#[test]
fn trailing_backslash() {
    assert_eq!(escaped("hello\\"), "hello\\");
}

#[test]
fn multiple_escapes() {
    assert_eq!(escaped("\\t\\n\\t"), "\t\n\t");
}

#[test]
fn mixed_text_and_escapes() {
    assert_eq!(escaped("A\\tB\\nC"), "A\tB\nC");
}
