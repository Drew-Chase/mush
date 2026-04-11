use std::io::{self, BufRead, Read, Write};

use crate::cli::UniqConfig;

/// Extract the comparison key from a line according to skip_fields, skip_chars, and check_chars.
fn comparison_key(line: &str, config: &UniqConfig) -> String {
    let mut s = line;

    // Skip fields: a field is a run of blanks followed by a run of non-blanks
    if config.skip_fields > 0 {
        let mut remaining = s;
        for _ in 0..config.skip_fields {
            // Skip leading blanks
            remaining = remaining.trim_start_matches([' ', '\t']);
            // Skip non-blanks
            let end = remaining
                .find([' ', '\t'])
                .unwrap_or(remaining.len());
            remaining = &remaining[end..];
        }
        s = remaining;
    }

    // Skip chars
    if config.skip_chars > 0 {
        let skip = config.skip_chars.min(s.len());
        s = &s[skip..];
    }

    // Check chars
    let s = if let Some(n) = config.check_chars {
        let end = n.min(s.len());
        &s[..end]
    } else {
        s
    };

    if config.ignore_case {
        s.to_lowercase()
    } else {
        s.to_string()
    }
}

pub fn uniq(input: &mut dyn Read, output: &mut dyn Write, config: &UniqConfig) -> io::Result<()> {
    let reader = io::BufReader::new(input);
    let lines: Vec<String> = reader.lines().collect::<io::Result<Vec<_>>>()?;

    if lines.is_empty() {
        return Ok(());
    }

    // Group adjacent identical lines
    let mut groups: Vec<(usize, &str)> = Vec::new(); // (count, representative line)
    let mut current_key = comparison_key(&lines[0], config);
    let mut count: usize = 1;
    let mut representative = 0usize;

    for i in 1..lines.len() {
        let key = comparison_key(&lines[i], config);
        if key == current_key {
            count += 1;
        } else {
            groups.push((count, &lines[representative]));
            current_key = key;
            count = 1;
            representative = i;
        }
    }
    groups.push((count, &lines[representative]));

    if config.all_repeated {
        // -D: print every line that belongs to a duplicate group
        let mut group_idx = 0;
        let mut line_idx = 0;
        while group_idx < groups.len() {
            let (cnt, _) = groups[group_idx];
            if cnt > 1 {
                for j in 0..cnt {
                    if config.count {
                        writeln!(output, "{:>7} {}", cnt, &lines[line_idx + j])?;
                    } else {
                        writeln!(output, "{}", &lines[line_idx + j])?;
                    }
                }
            }
            line_idx += cnt;
            group_idx += 1;
        }
    } else if config.repeated {
        // -d: print one copy of each duplicated group
        for &(cnt, line) in &groups {
            if cnt > 1 {
                if config.count {
                    writeln!(output, "{cnt:>7} {line}")?;
                } else {
                    writeln!(output, "{line}")?;
                }
            }
        }
    } else if config.unique {
        // -u: print only lines that are NOT repeated
        for &(cnt, line) in &groups {
            if cnt == 1 {
                if config.count {
                    writeln!(output, "{cnt:>7} {line}")?;
                } else {
                    writeln!(output, "{line}")?;
                }
            }
        }
    } else {
        // Default: collapse adjacent duplicates
        for &(cnt, line) in &groups {
            if config.count {
                writeln!(output, "{cnt:>7} {line}")?;
            } else {
                writeln!(output, "{line}")?;
            }
        }
    }

    Ok(())
}
