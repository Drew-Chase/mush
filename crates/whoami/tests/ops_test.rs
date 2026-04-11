use whoami::ops::current_username;

#[test]
fn returns_non_empty_string() {
    let name = current_username().unwrap();
    assert!(!name.is_empty());
}
