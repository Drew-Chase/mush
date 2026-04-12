use std::io::{self, BufRead, Read, Write};

use crate::cli::JoinConfig;

struct Line {
    raw: String,
    fields: Vec<String>,
}

impl Line {
    fn new(raw: String, sep: Option<char>) -> Self {
        let fields: Vec<String> = match sep {
            Some(c) => raw.split(c).map(|s| s.to_string()).collect(),
            None => raw.split_whitespace().map(|s| s.to_string()).collect(),
        };
        Self { raw, fields }
    }

    fn field(&self, idx: usize) -> Option<&str> {
        if idx == 0 {
            return None;
        }
        self.fields.get(idx - 1).map(|s| s.as_str())
    }
}

fn key_matches(a: &str, b: &str, ignore_case: bool) -> std::cmp::Ordering {
    if ignore_case {
        a.to_lowercase().cmp(&b.to_lowercase())
    } else {
        a.cmp(b)
    }
}

fn format_output(
    line1: Option<&Line>,
    line2: Option<&Line>,
    config: &JoinConfig,
    output: &mut dyn Write,
) -> io::Result<()> {
    let sep = match config.separator {
        Some(c) => c.to_string(),
        None => " ".to_string(),
    };
    let empty = config.empty.as_deref().unwrap_or("");

    if let Some(ref fmt) = config.format {
        let specs: Vec<&str> = fmt.split([',', ' ']).filter(|s| !s.is_empty()).collect();
        let mut parts: Vec<String> = Vec::new();

        for spec in &specs {
            if *spec == "0" {
                // Join field
                let key = line1
                    .and_then(|l| l.field(config.field1))
                    .or_else(|| line2.and_then(|l| l.field(config.field2)))
                    .unwrap_or(empty);
                parts.push(key.to_string());
            } else if let Some(rest) = spec.strip_prefix("1.") {
                let idx: usize = rest.parse().unwrap_or(0);
                let val = line1
                    .and_then(|l| l.field(idx))
                    .unwrap_or(empty);
                parts.push(val.to_string());
            } else if let Some(rest) = spec.strip_prefix("2.") {
                let idx: usize = rest.parse().unwrap_or(0);
                let val = line2
                    .and_then(|l| l.field(idx))
                    .unwrap_or(empty);
                parts.push(val.to_string());
            }
        }

        writeln!(output, "{}", parts.join(&sep))?;
    } else {
        // Default: join field, then remaining fields from file1, then remaining fields from file2
        let mut parts: Vec<String> = Vec::new();

        // Join field value
        let key = line1
            .and_then(|l| l.field(config.field1))
            .or_else(|| line2.and_then(|l| l.field(config.field2)))
            .unwrap_or(empty);
        parts.push(key.to_string());

        // Remaining fields from file1
        if let Some(l1) = line1 {
            for (idx, f) in l1.fields.iter().enumerate() {
                if idx + 1 != config.field1 {
                    parts.push(f.clone());
                }
            }
        }

        // Remaining fields from file2
        if let Some(l2) = line2 {
            for (idx, f) in l2.fields.iter().enumerate() {
                if idx + 1 != config.field2 {
                    parts.push(f.clone());
                }
            }
        }

        writeln!(output, "{}", parts.join(&sep))?;
    }

    Ok(())
}

fn output_unpaired(
    line: &Line,
    file_num: u8,
    config: &JoinConfig,
    output: &mut dyn Write,
) -> io::Result<()> {
    if config.format.is_some() {
        if file_num == 1 {
            format_output(Some(line), None, config, output)?;
        } else {
            format_output(None, Some(line), config, output)?;
        }
    } else {
        writeln!(output, "{}", line.raw)?;
    }
    Ok(())
}

pub fn join(
    input1: &mut dyn Read,
    input2: &mut dyn Read,
    output: &mut dyn Write,
    config: &JoinConfig,
) -> io::Result<()> {
    let reader1 = io::BufReader::new(input1);
    let reader2 = io::BufReader::new(input2);

    let lines1: Vec<Line> = reader1
        .lines()
        .collect::<io::Result<Vec<_>>>()?
        .into_iter()
        .map(|s| Line::new(s, config.separator))
        .collect();

    let lines2: Vec<Line> = reader2
        .lines()
        .collect::<io::Result<Vec<_>>>()?
        .into_iter()
        .map(|s| Line::new(s, config.separator))
        .collect();

    let suppress_paired = config.only_unpaired1 || config.only_unpaired2;
    let show_unpaired1 = config.unpaired1 || config.only_unpaired1;
    let show_unpaired2 = config.unpaired2 || config.only_unpaired2;

    let mut i = 0;
    let mut j = 0;

    while i < lines1.len() && j < lines2.len() {
        let key1 = lines1[i].field(config.field1).unwrap_or("");
        let key2 = lines2[j].field(config.field2).unwrap_or("");

        match key_matches(key1, key2, config.ignore_case) {
            std::cmp::Ordering::Less => {
                if show_unpaired1 {
                    output_unpaired(&lines1[i], 1, config, output)?;
                }
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                if show_unpaired2 {
                    output_unpaired(&lines2[j], 2, config, output)?;
                }
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                // Find all lines in file1 and file2 with this key
                let mut i_end = i + 1;
                while i_end < lines1.len() {
                    let k = lines1[i_end].field(config.field1).unwrap_or("");
                    if key_matches(k, key1, config.ignore_case) != std::cmp::Ordering::Equal {
                        break;
                    }
                    i_end += 1;
                }
                let mut j_end = j + 1;
                while j_end < lines2.len() {
                    let k = lines2[j_end].field(config.field2).unwrap_or("");
                    if key_matches(k, key2, config.ignore_case) != std::cmp::Ordering::Equal {
                        break;
                    }
                    j_end += 1;
                }

                if !suppress_paired {
                    for l1 in &lines1[i..i_end] {
                        for l2 in &lines2[j..j_end] {
                            format_output(
                                Some(l1),
                                Some(l2),
                                config,
                                output,
                            )?;
                        }
                    }
                }

                i = i_end;
                j = j_end;
            }
        }
    }

    // Remaining unpairable lines
    while i < lines1.len() {
        if show_unpaired1 {
            output_unpaired(&lines1[i], 1, config, output)?;
        }
        i += 1;
    }

    while j < lines2.len() {
        if show_unpaired2 {
            output_unpaired(&lines2[j], 2, config, output)?;
        }
        j += 1;
    }

    Ok(())
}
