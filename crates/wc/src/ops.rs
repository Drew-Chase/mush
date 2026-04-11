use std::io::Read;

use crate::cli::WcConfig;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WcCounts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub max_line_length: usize,
}

impl WcCounts {
    pub fn add(&mut self, other: &WcCounts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
        if other.max_line_length > self.max_line_length {
            self.max_line_length = other.max_line_length;
        }
    }
}

pub fn count(input: &mut dyn Read, _config: &WcConfig) -> WcCounts {
    let mut buf = Vec::new();
    let _ = input.read_to_end(&mut buf);

    let mut counts = WcCounts { bytes: buf.len(), ..Default::default() };

    let mut in_word = false;
    let mut current_line_len: usize = 0;

    for &b in &buf {
        if b == b'\n' {
            counts.lines += 1;
            if current_line_len > counts.max_line_length {
                counts.max_line_length = current_line_len;
            }
            current_line_len = 0;
        } else {
            current_line_len += 1;
        }

        let is_whitespace = matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
        if is_whitespace {
            if in_word {
                counts.words += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }
    if in_word {
        counts.words += 1;
    }
    // Handle last line without trailing newline
    if current_line_len > counts.max_line_length {
        counts.max_line_length = current_line_len;
    }

    // Count chars via UTF-8 decoding
    let text = String::from_utf8_lossy(&buf);
    counts.chars = text.chars().count();

    counts
}

pub fn format_counts(counts: &WcCounts, config: &WcConfig, filename: Option<&str>) -> String {
    let mut parts: Vec<String> = Vec::new();

    if config.lines {
        parts.push(format!("{:>8}", counts.lines));
    }
    if config.words {
        parts.push(format!("{:>8}", counts.words));
    }
    if config.bytes {
        parts.push(format!("{:>8}", counts.bytes));
    }
    if config.chars {
        parts.push(format!("{:>8}", counts.chars));
    }
    if config.max_line_length {
        parts.push(format!("{:>8}", counts.max_line_length));
    }

    let mut result = parts.join("");
    if let Some(name) = filename {
        result.push(' ');
        result.push_str(name);
    }
    result
}
