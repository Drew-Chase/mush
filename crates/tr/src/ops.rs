use std::collections::HashSet;
use std::io::{self, Read, Write};

use crate::cli::TrConfig;

/// Expand a tr set specification into a list of characters.
///
/// Supports:
/// - Character classes: `[:upper:]`, `[:lower:]`, `[:digit:]`, `[:alpha:]`, `[:alnum:]`, `[:space:]`
/// - Ranges: `a-z`, `A-Z`, `0-9`
/// - Escape sequences: `\n`, `\t`, `\\`
pub fn expand_set(set: &str) -> Vec<char> {
    let mut result = Vec::new();
    let chars: Vec<char> = set.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for character classes [:class:]
        if i + 1 < chars.len()
            && chars[i] == '['
            && chars[i + 1] == ':'
            && let Some(end) = find_class_end(&chars, i + 2)
        {
            let class_name: String = chars[i + 2..end].iter().collect();
            expand_class(&class_name, &mut result);
            i = end + 2; // skip past :]
            continue;
        }

        // Check for escape sequences
        if chars[i] == '\\' && i + 1 < chars.len() {
            let escaped = match chars[i + 1] {
                'n' => '\n',
                't' => '\t',
                '\\' => '\\',
                other => other,
            };
            result.push(escaped);
            i += 2;
            continue;
        }

        // Check for ranges: a-z
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            let start = chars[i] as u32;
            let end = chars[i + 2] as u32;
            if start <= end {
                for code in start..=end {
                    if let Some(c) = char::from_u32(code) {
                        result.push(c);
                    }
                }
            }
            i += 3;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Find the position of the closing ':' in a character class like `[:upper:]`.
/// `start` points to the first character after `[:`.
/// Returns the position of `:` in `:]`.
fn find_class_end(chars: &[char], start: usize) -> Option<usize> {
    let mut j = start;
    while j + 1 < chars.len() {
        if chars[j] == ':' && chars[j + 1] == ']' {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn expand_class(name: &str, result: &mut Vec<char>) {
    match name {
        "upper" => result.extend('A'..='Z'),
        "lower" => result.extend('a'..='z'),
        "digit" => result.extend('0'..='9'),
        "alpha" => {
            result.extend('a'..='z');
            result.extend('A'..='Z');
        }
        "alnum" => {
            result.extend('a'..='z');
            result.extend('A'..='Z');
            result.extend('0'..='9');
        }
        "space" => {
            result.extend([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
        }
        _ => {}
    }
}

/// Translate, delete, or squeeze characters from input to output.
pub fn translate(
    input: &mut dyn Read,
    output: &mut dyn Write,
    config: &TrConfig,
) -> io::Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let set1_chars = expand_set(&config.set1);
    let set2_chars = config.set2.as_deref().map(expand_set).unwrap_or_default();

    let set1_set: HashSet<char> = set1_chars.iter().copied().collect();

    if config.delete {
        // Delete mode: remove characters in set1 (or complement)
        let mut result = String::new();
        for ch in buf.chars() {
            let in_set1 = set1_set.contains(&ch);
            let should_delete = if config.complement { !in_set1 } else { in_set1 };
            if !should_delete {
                result.push(ch);
            }
        }

        // If squeeze is also specified, squeeze set2 chars
        if config.squeeze && !set2_chars.is_empty() {
            let squeeze_set: HashSet<char> = set2_chars.iter().copied().collect();
            result = squeeze_string(&result, &squeeze_set);
        }

        output.write_all(result.as_bytes())?;
    } else if config.squeeze && config.set2.is_none() {
        // Squeeze-only mode: squeeze repeated chars in set1
        let squeeze_set: HashSet<char> = set1_chars.iter().copied().collect();
        let result = if config.complement {
            squeeze_complement(&buf, &squeeze_set)
        } else {
            squeeze_string(&buf, &squeeze_set)
        };
        output.write_all(result.as_bytes())?;
    } else if config.set2.is_some() {
        // Translate mode
        let mut s1 = set1_chars;
        let mut s2 = set2_chars;

        if config.truncate {
            s1.truncate(s2.len());
        } else if !s2.is_empty() && s2.len() < s1.len() {
            // Extend set2 with its last character to match set1 length
            let last = *s2.last().unwrap();
            s2.resize(s1.len(), last);
        }

        let mut result = String::new();
        for ch in buf.chars() {
            let in_set1 = s1.contains(&ch);
            if config.complement {
                if !in_set1 {
                    // Complement translate: chars NOT in set1 get mapped to last char of set2
                    if let Some(&replacement) = s2.last() {
                        result.push(replacement);
                    } else {
                        result.push(ch);
                    }
                } else {
                    result.push(ch);
                }
            } else if let Some(pos) = s1.iter().position(|&c| c == ch) {
                if pos < s2.len() {
                    result.push(s2[pos]);
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        // If squeeze is also specified, squeeze set2 chars
        if config.squeeze {
            let squeeze_set: HashSet<char> = s2.iter().copied().collect();
            result = squeeze_string(&result, &squeeze_set);
        }

        output.write_all(result.as_bytes())?;
    } else {
        // No operation, just pass through
        output.write_all(buf.as_bytes())?;
    }

    Ok(())
}

fn squeeze_string(s: &str, squeeze_set: &HashSet<char>) -> String {
    let mut result = String::new();
    let mut last_char: Option<char> = None;
    for ch in s.chars() {
        if squeeze_set.contains(&ch) && last_char == Some(ch) {
            continue;
        }
        result.push(ch);
        last_char = Some(ch);
    }
    result
}

fn squeeze_complement(s: &str, keep_set: &HashSet<char>) -> String {
    let mut result = String::new();
    let mut last_was_complement = false;
    let mut last_char: Option<char> = None;
    for ch in s.chars() {
        if !keep_set.contains(&ch) {
            // This is a complement char
            if last_was_complement && last_char == Some(ch) {
                continue;
            }
            last_was_complement = true;
        } else {
            last_was_complement = false;
        }
        result.push(ch);
        last_char = Some(ch);
    }
    result
}
