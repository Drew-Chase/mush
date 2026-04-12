use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::FmtConfig;

/// Reformat paragraphs in input to the specified width.
pub fn fmt(input: &mut dyn Read, output: &mut dyn Write, config: &FmtConfig) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut lines: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        lines.push(line_result?);
    }

    let paragraphs = split_paragraphs(&lines);

    let mut first = true;
    for paragraph in &paragraphs {
        if !first {
            writeln!(output)?;
        }
        first = false;

        if paragraph.is_empty() {
            continue;
        }

        // Check if prefix filtering applies
        if let Some(ref pfx) = config.prefix {
            if !paragraph.iter().all(|l| l.starts_with(pfx.as_str())) {
                // Not all lines match prefix; output as-is
                for line in paragraph {
                    writeln!(output, "{line}")?;
                }
                continue;
            }
            // Strip prefix, format, then re-add
            let stripped: Vec<String> = paragraph
                .iter()
                .map(|l| l[pfx.len()..].to_string())
                .collect();
            let formatted = format_paragraph(&stripped, config);
            for line in &formatted {
                writeln!(output, "{pfx}{line}")?;
            }
            continue;
        }

        let formatted = format_paragraph(paragraph, config);
        for line in &formatted {
            writeln!(output, "{line}")?;
        }
    }

    Ok(())
}

fn split_paragraphs(lines: &[String]) -> Vec<Vec<String>> {
    let mut paragraphs: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(current.clone());
                current.clear();
            }
            // Add empty paragraph marker
            paragraphs.push(Vec::new());
        } else {
            current.push(line.clone());
        }
    }

    if !current.is_empty() {
        paragraphs.push(current);
    }

    paragraphs
}

fn format_paragraph(lines: &[String], config: &FmtConfig) -> Vec<String> {
    if config.split_only {
        // Only split long lines, don't join
        let mut result = Vec::new();
        for line in lines {
            if line.len() <= config.width {
                result.push(line.clone());
            } else {
                split_long_line(line, config.width, &mut result);
            }
        }
        return result;
    }

    // Collect all words from the paragraph
    let mut words: Vec<String> = Vec::new();
    for line in lines {
        let line_words: Vec<&str> = line.split_whitespace().collect();
        for w in line_words {
            words.push(w.to_string());
        }
    }

    if words.is_empty() {
        return vec![String::new()];
    }

    if config.uniform {
        // Normalize: single space between words, two after sentence-ending punctuation
        return reflow_uniform(&words, config.width);
    }

    reflow_words(&words, config.width)
}

fn reflow_words(words: &[String], width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            result.push(current_line.clone());
            current_line.clear();
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}

fn reflow_uniform(words: &[String], width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else {
            // Two spaces after sentence-ending punctuation
            let spaces = if ends_sentence(&current_line) {
                "  "
            } else {
                " "
            };
            if current_line.len() + spaces.len() + word.len() <= width {
                current_line.push_str(spaces);
                current_line.push_str(word);
            } else {
                result.push(current_line.clone());
                current_line.clear();
                current_line.push_str(word);
            }
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}

fn ends_sentence(s: &str) -> bool {
    s.ends_with('.') || s.ends_with('!') || s.ends_with('?')
}

fn split_long_line(line: &str, width: usize, result: &mut Vec<String>) {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut current = String::new();

    // Preserve leading whitespace
    let leading: String = line.chars().take_while(|c| c.is_whitespace()).collect();

    for word in &words {
        if current.is_empty() {
            current.push_str(&leading);
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            result.push(current.clone());
            current.clear();
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }
}
