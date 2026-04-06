pub fn process_escapes(input: &str) -> (String, bool) {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }

        match chars.next() {
            None => output.push('\\'),
            Some('\\') => output.push('\\'),
            Some('a') => output.push('\x07'),
            Some('b') => output.push('\x08'),
            Some('c') => return (output, true),
            Some('e') => output.push('\x1B'),
            Some('f') => output.push('\x0C'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('v') => output.push('\x0B'),
            Some('0') => {
                let mut value: u8 = 0;
                for _ in 0..3 {
                    match chars.peek() {
                        Some(&d) if ('0'..='7').contains(&d) => {
                            value = value.wrapping_mul(8).wrapping_add(d as u8 - b'0');
                            chars.next();
                        }
                        _ => break,
                    }
                }
                output.push(value as char);
            }
            Some('x') => {
                let mut digits = String::new();
                for _ in 0..2 {
                    match chars.peek() {
                        Some(&d) if d.is_ascii_hexdigit() => {
                            digits.push(d);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if digits.is_empty() {
                    output.push('\\');
                    output.push('x');
                } else {
                    let value = u8::from_str_radix(&digits, 16).unwrap();
                    output.push(value as char);
                }
            }
            Some(other) => {
                output.push('\\');
                output.push(other);
            }
        }
    }

    (output, false)
}
