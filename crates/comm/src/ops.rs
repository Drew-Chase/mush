use std::io::{self, BufRead, Read, Write};

use crate::cli::CommConfig;

pub fn comm(
    input1: &mut dyn Read,
    input2: &mut dyn Read,
    output: &mut dyn Write,
    config: &CommConfig,
) -> io::Result<()> {
    let reader1 = io::BufReader::new(input1);
    let reader2 = io::BufReader::new(input2);

    let lines1: Vec<String> = reader1.lines().collect::<io::Result<Vec<_>>>()?;
    let lines2: Vec<String> = reader2.lines().collect::<io::Result<Vec<_>>>()?;

    let delim = config
        .output_delimiter
        .as_deref()
        .unwrap_or("\t");

    let mut i = 0;
    let mut j = 0;

    while i < lines1.len() && j < lines2.len() {
        let cmp = if config.ignore_case {
            lines1[i]
                .to_lowercase()
                .cmp(&lines2[j].to_lowercase())
        } else {
            lines1[i].cmp(&lines2[j])
        };

        match cmp {
            std::cmp::Ordering::Less => {
                // Line only in file1
                if !config.suppress1 {
                    writeln!(output, "{}", &lines1[i])?;
                }
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                // Line only in file2
                if !config.suppress2 {
                    let prefix = if config.suppress1 { "" } else { delim };
                    writeln!(output, "{prefix}{}", &lines2[j])?;
                }
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                // Line in both
                if !config.suppress3 {
                    let mut prefix = String::new();
                    if !config.suppress1 {
                        prefix.push_str(delim);
                    }
                    if !config.suppress2 {
                        prefix.push_str(delim);
                    }
                    writeln!(output, "{prefix}{}", &lines1[i])?;
                }
                i += 1;
                j += 1;
            }
        }
    }

    // Remaining lines from file1
    while i < lines1.len() {
        if !config.suppress1 {
            writeln!(output, "{}", &lines1[i])?;
        }
        i += 1;
    }

    // Remaining lines from file2
    while j < lines2.len() {
        if !config.suppress2 {
            let prefix = if config.suppress1 { "" } else { delim };
            writeln!(output, "{prefix}{}", &lines2[j])?;
        }
        j += 1;
    }

    Ok(())
}
