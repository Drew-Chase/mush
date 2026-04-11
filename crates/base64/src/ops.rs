use data_encoding::BASE64;

pub fn encode(input: &[u8], wrap: usize) -> String {
    let encoded = BASE64.encode(input);
    if wrap == 0 {
        return encoded;
    }
    encoded
        .as_bytes()
        .chunks(wrap)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn decode(input: &str, ignore_garbage: bool) -> Result<Vec<u8>, String> {
    let cleaned: String = if ignore_garbage {
        input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=')
            .collect()
    } else {
        input.replace(['\n', '\r'], "")
    };
    BASE64.decode(cleaned.as_bytes()).map_err(|e| e.to_string())
}
