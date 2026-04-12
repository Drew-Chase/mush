use std::cmp::Ordering;

use crate::cli::{SortConfig, SortKey};

/// Extract the field(s) from a line based on a key specification and separator.
fn extract_key(line: &str, key: &SortKey, separator: Option<char>) -> String {
    let fields: Vec<&str> = match separator {
        Some(sep) => line.split(sep).collect(),
        None => line.split_whitespace().collect(),
    };

    let start = key.start_field.saturating_sub(1);

    match key.end_field {
        Some(end) => {
            let end_idx = end.min(fields.len());
            if start >= fields.len() {
                String::new()
            } else {
                fields[start..end_idx].join(" ")
            }
        }
        None => {
            if start < fields.len() {
                fields[start].to_string()
            } else {
                String::new()
            }
        }
    }
}

/// Parse a leading numeric value from a string for numeric comparison.
fn parse_numeric(s: &str) -> f64 {
    let trimmed = s.trim();
    // Try to parse as much of the leading portion as possible
    let mut end = 0;
    let bytes = trimmed.as_bytes();
    if end < bytes.len() && (bytes[end] == b'-' || bytes[end] == b'+') {
        end += 1;
    }
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b'.' {
        end += 1;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
    }
    if end == 0 {
        return 0.0;
    }
    trimmed[..end].parse::<f64>().unwrap_or(0.0)
}

/// Parse a human-readable numeric value (e.g., "2K", "1.5G").
fn parse_human_numeric(s: &str) -> f64 {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return 0.0;
    }

    let last = trimmed.bytes().last().unwrap();
    let (num_part, multiplier) = match last {
        b'K' | b'k' => (&trimmed[..trimmed.len() - 1], 1_000.0),
        b'M' | b'm' => (&trimmed[..trimmed.len() - 1], 1_000_000.0),
        b'G' | b'g' => (&trimmed[..trimmed.len() - 1], 1_000_000_000.0),
        b'T' | b't' => (&trimmed[..trimmed.len() - 1], 1_000_000_000_000.0),
        b'P' | b'p' => (&trimmed[..trimmed.len() - 1], 1_000_000_000_000_000.0),
        b'E' | b'e' => (&trimmed[..trimmed.len() - 1], 1_000_000_000_000_000_000.0),
        _ => (trimmed, 1.0),
    };

    parse_numeric(num_part) * multiplier
}

/// Filter a string to only blanks and alphanumeric characters (dictionary order).
fn dictionary_filter(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_ascii_whitespace())
        .collect()
}

/// Strip leading blanks from a string.
fn strip_leading_blanks(s: &str) -> &str {
    s.trim_start_matches([' ', '\t'])
}

/// Build a comparison key string from a line according to config.
fn comparison_value(line: &str, config: &SortConfig) -> String {
    let mut val = if !config.key.is_empty() {
        config
            .key
            .iter()
            .map(|k| extract_key(line, k, config.separator))
            .collect::<Vec<_>>()
            .join("\0")
    } else {
        line.to_string()
    };

    if config.ignore_blanks {
        val = strip_leading_blanks(&val).to_string();
    }
    if config.dictionary {
        val = dictionary_filter(&val);
    }
    if config.ignore_case {
        val = val.to_uppercase();
    }

    val
}

/// Compare two lines according to the sort configuration.
fn compare_lines(a: &str, b: &str, config: &SortConfig) -> Ordering {
    let ord = if config.human_numeric {
        let va = if !config.key.is_empty() {
            config
                .key
                .iter()
                .map(|k| extract_key(a, k, config.separator))
                .collect::<Vec<_>>()
                .join("\0")
        } else {
            a.to_string()
        };
        let vb = if !config.key.is_empty() {
            config
                .key
                .iter()
                .map(|k| extract_key(b, k, config.separator))
                .collect::<Vec<_>>()
                .join("\0")
        } else {
            b.to_string()
        };
        let na = parse_human_numeric(&va);
        let nb = parse_human_numeric(&vb);
        na.partial_cmp(&nb).unwrap_or(Ordering::Equal)
    } else if config.numeric {
        let va = if !config.key.is_empty() {
            config
                .key
                .iter()
                .map(|k| extract_key(a, k, config.separator))
                .collect::<Vec<_>>()
                .join("\0")
        } else {
            a.to_string()
        };
        let vb = if !config.key.is_empty() {
            config
                .key
                .iter()
                .map(|k| extract_key(b, k, config.separator))
                .collect::<Vec<_>>()
                .join("\0")
        } else {
            b.to_string()
        };
        let na = parse_numeric(&va);
        let nb = parse_numeric(&vb);
        na.partial_cmp(&nb).unwrap_or(Ordering::Equal)
    } else {
        let va = comparison_value(a, config);
        let vb = comparison_value(b, config);
        va.cmp(&vb)
    };

    if config.reverse { ord.reverse() } else { ord }
}

/// Sort lines in place according to the configuration.
pub fn sort_lines(lines: &mut Vec<String>, config: &SortConfig) {
    lines.sort_by(|a, b| compare_lines(a, b, config));

    if config.unique {
        lines.dedup_by(|a, b| compare_lines(a, b, config) == Ordering::Equal);
    }
}

/// Check whether lines are already sorted according to the configuration.
pub fn check_sorted(lines: &[String], config: &SortConfig) -> bool {
    lines.windows(2).all(|pair| {
        let ord = compare_lines(&pair[0], &pair[1], config);
        if config.unique {
            ord == Ordering::Less
        } else {
            ord != Ordering::Greater
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric() {
        assert_eq!(parse_numeric("42"), 42.0);
        assert_eq!(parse_numeric("  -3.14"), -std::f64::consts::PI);
        assert_eq!(parse_numeric("abc"), 0.0);
        assert_eq!(parse_numeric("10abc"), 10.0);
    }

    #[test]
    fn test_parse_human_numeric() {
        assert_eq!(parse_human_numeric("1K"), 1000.0);
        assert_eq!(parse_human_numeric("2M"), 2_000_000.0);
        assert_eq!(parse_human_numeric("3G"), 3_000_000_000.0);
        assert_eq!(parse_human_numeric("42"), 42.0);
    }

    #[test]
    fn test_dictionary_filter() {
        assert_eq!(dictionary_filter("hello, world!"), "hello world");
        assert_eq!(dictionary_filter("abc 123"), "abc 123");
    }

    #[test]
    fn test_extract_key() {
        assert_eq!(extract_key("a b c", &SortKey { start_field: 2, end_field: None }, None), "b");
        assert_eq!(
            extract_key("a:b:c", &SortKey { start_field: 1, end_field: Some(2) }, Some(':')),
            "a b"
        );
    }

    #[test]
    fn test_sort_basic() {
        let config = SortConfig::default();
        let mut lines = vec!["banana".into(), "apple".into(), "cherry".into()];
        sort_lines(&mut lines, &config);
        assert_eq!(lines, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_reverse() {
        let config = SortConfig { reverse: true, ..Default::default() };
        let mut lines = vec!["banana".into(), "apple".into(), "cherry".into()];
        sort_lines(&mut lines, &config);
        assert_eq!(lines, vec!["cherry", "banana", "apple"]);
    }

    #[test]
    fn test_sort_numeric() {
        let config = SortConfig { numeric: true, ..Default::default() };
        let mut lines = vec!["10".into(), "2".into(), "1".into(), "20".into()];
        sort_lines(&mut lines, &config);
        assert_eq!(lines, vec!["1", "2", "10", "20"]);
    }

    #[test]
    fn test_sort_unique() {
        let config = SortConfig { unique: true, ..Default::default() };
        let mut lines = vec!["a".into(), "b".into(), "a".into(), "b".into(), "c".into()];
        sort_lines(&mut lines, &config);
        assert_eq!(lines, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_check_sorted_true() {
        let config = SortConfig::default();
        let lines: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        assert!(check_sorted(&lines, &config));
    }

    #[test]
    fn test_check_sorted_false() {
        let config = SortConfig::default();
        let lines: Vec<String> = vec!["c".into(), "a".into(), "b".into()];
        assert!(!check_sorted(&lines, &config));
    }
}
