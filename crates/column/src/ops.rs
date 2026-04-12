use std::io::{self, BufRead, BufReader, Read, Write};

use crate::cli::ColumnConfig;

/// Columnate input data.
pub fn column(
    input: &mut dyn Read,
    output: &mut dyn Write,
    config: &ColumnConfig,
) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut lines: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if !line.is_empty() {
            lines.push(line);
        }
    }

    if lines.is_empty() {
        return Ok(());
    }

    if config.json {
        return output_json(&lines, output, config);
    }

    if config.table {
        return output_table(&lines, output, config);
    }

    output_columns(&lines, output, config)
}

fn split_row<'a>(line: &'a str, separator: &Option<String>) -> Vec<&'a str> {
    match separator {
        Some(sep) => line.split(sep.as_str()).collect(),
        None => line.split_whitespace().collect(),
    }
}

fn output_table(
    lines: &[String],
    output: &mut dyn Write,
    config: &ColumnConfig,
) -> io::Result<()> {
    // Parse all rows
    let rows: Vec<Vec<&str>> = lines.iter().map(|l| split_row(l, &config.separator)).collect();

    // Find max columns
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Compute column widths
    let mut col_widths = vec![0usize; num_cols];
    for row in &rows {
        for (j, cell) in row.iter().enumerate() {
            if cell.len() > col_widths[j] {
                col_widths[j] = cell.len();
            }
        }
    }

    // Parse right-align columns
    let right_cols = parse_right_align(&config.right_align, num_cols);

    // Determine column names
    let names: Option<Vec<&str>> = config
        .column_names
        .as_deref()
        .map(|n| n.split(',').collect());

    // If column names specified, print header
    if let Some(ref names) = names {
        for (j, name) in names.iter().enumerate() {
            if j > 0 {
                write!(output, "{}", config.output_separator)?;
            }
            let w = if j < col_widths.len() {
                col_widths[j].max(name.len())
            } else {
                name.len()
            };
            // Update col_widths to account for header
            if j < col_widths.len() {
                col_widths[j] = w;
            }
            if right_cols.contains(&j) {
                write!(output, "{:>width$}", name, width = w)?;
            } else {
                write!(output, "{:<width$}", name, width = w)?;
            }
        }
        writeln!(output)?;
    }

    // Print rows
    for row in &rows {
        for (j, cell) in row.iter().enumerate() {
            if j > 0 {
                write!(output, "{}", config.output_separator)?;
            }
            let w = if j < col_widths.len() {
                col_widths[j]
            } else {
                cell.len()
            };
            if j == row.len() - 1 {
                // Last column: no padding
                write!(output, "{cell}")?;
            } else if right_cols.contains(&j) {
                write!(output, "{:>width$}", cell, width = w)?;
            } else {
                write!(output, "{:<width$}", cell, width = w)?;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}

fn output_json(
    lines: &[String],
    output: &mut dyn Write,
    config: &ColumnConfig,
) -> io::Result<()> {
    let rows: Vec<Vec<&str>> = lines.iter().map(|l| split_row(l, &config.separator)).collect();
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Get column names
    let names: Vec<String> = if let Some(ref cn) = config.column_names {
        cn.split(',').map(|s| s.to_string()).collect()
    } else {
        (0..num_cols)
            .map(|i| format!("column{}", i + 1))
            .collect()
    };

    writeln!(output, "[")?;
    for (i, row) in rows.iter().enumerate() {
        write!(output, "  {{")?;
        for (j, cell) in row.iter().enumerate() {
            let name = if j < names.len() {
                &names[j]
            } else {
                "unknown"
            };
            if j > 0 {
                write!(output, ", ")?;
            }
            write!(output, "\"{}\": \"{}\"", escape_json(name), escape_json(cell))?;
        }
        if i + 1 < rows.len() {
            writeln!(output, "}},")?;
        } else {
            writeln!(output, "}}")?;
        }
    }
    writeln!(output, "]")?;

    Ok(())
}

fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

fn output_columns(
    lines: &[String],
    output: &mut dyn Write,
    config: &ColumnConfig,
) -> io::Result<()> {
    let term_width = config.width.unwrap_or(80);

    // Find the longest entry
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let col_width = max_len + 2; // padding

    if col_width == 0 || col_width > term_width {
        // One column mode
        for line in lines {
            writeln!(output, "{line}")?;
        }
        return Ok(());
    }

    let num_cols = (term_width / col_width).max(1);
    let num_rows = lines.len().div_ceil(num_cols);

    for row in 0..num_rows {
        for col in 0..num_cols {
            let idx = col * num_rows + row;
            if idx >= lines.len() {
                break;
            }
            if col + 1 < num_cols {
                // Check if there's a next item in this row
                let next_idx = (col + 1) * num_rows + row;
                if next_idx < lines.len() {
                    write!(output, "{:<width$}", lines[idx], width = col_width)?;
                } else {
                    write!(output, "{}", lines[idx])?;
                }
            } else {
                write!(output, "{}", lines[idx])?;
            }
        }
        writeln!(output)?;
    }

    Ok(())
}

fn parse_right_align(spec: &Option<String>, _num_cols: usize) -> Vec<usize> {
    let mut result = Vec::new();
    if let Some(s) = spec {
        for part in s.split(',') {
            if let Ok(n) = part.trim().parse::<usize>() {
                // 1-based to 0-based
                if n > 0 {
                    result.push(n - 1);
                }
            }
        }
    }
    result
}
