const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: column [OPTION]... [FILE]...

Columnate lists. With no FILE, or when FILE is -, read standard input.

  -c, --output-width=WIDTH  output width in number of characters
  -J, --json                use JSON output format for table
  -N, --table-columns=NAMES comma separated list of column names
  -o, --output-separator=SEP  output column separator (default \"  \")
  -R, --table-right=COLS    right-align columns (comma-separated indices)
  -s, --separator=SEP       specify input column separator
  -t, --table               create a table
      --help                display this help and exit
      --version             output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnConfig {
    pub table: bool,
    pub separator: Option<String>,
    pub output_separator: String,
    pub width: Option<usize>,
    pub column_names: Option<String>,
    pub right_align: Option<String>,
    pub json: bool,
    pub files: Vec<String>,
}

impl ColumnConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut table = false;
        let mut separator: Option<String> = None;
        let mut output_separator = "  ".to_string();
        let mut width: Option<usize> = None;
        let mut column_names: Option<String> = None;
        let mut right_align: Option<String> = None;
        let mut json = false;
        let mut files: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("column {VERSION}");
                    return None;
                }
                "--table" => table = true,
                "--json" => json = true,
                s if s.starts_with("--separator=") => {
                    separator = Some(s["--separator=".len()..].to_string());
                }
                "--separator" => {
                    i += 1;
                    if i < args.len() {
                        separator = Some(args[i].clone());
                    } else {
                        eprintln!("column: option '--separator' requires an argument");
                        return None;
                    }
                }
                s if s.starts_with("--output-separator=") => {
                    output_separator = s["--output-separator=".len()..].to_string();
                }
                "--output-separator" => {
                    i += 1;
                    if i < args.len() {
                        output_separator = args[i].clone();
                    } else {
                        eprintln!("column: option '--output-separator' requires an argument");
                        return None;
                    }
                }
                s if s.starts_with("--output-width=") => {
                    if let Ok(n) = s["--output-width=".len()..].parse() {
                        width = Some(n);
                    } else {
                        eprintln!(
                            "column: invalid width: '{}'",
                            &s["--output-width=".len()..]
                        );
                        return None;
                    }
                }
                "--output-width" => {
                    i += 1;
                    if i < args.len() {
                        if let Ok(n) = args[i].parse() {
                            width = Some(n);
                        } else {
                            eprintln!("column: invalid width: '{}'", args[i]);
                            return None;
                        }
                    } else {
                        eprintln!("column: option '--output-width' requires an argument");
                        return None;
                    }
                }
                s if s.starts_with("--table-columns=") => {
                    column_names = Some(s["--table-columns=".len()..].to_string());
                }
                "--table-columns" => {
                    i += 1;
                    if i < args.len() {
                        column_names = Some(args[i].clone());
                    } else {
                        eprintln!("column: option '--table-columns' requires an argument");
                        return None;
                    }
                }
                s if s.starts_with("--table-right=") => {
                    right_align = Some(s["--table-right=".len()..].to_string());
                }
                "--table-right" => {
                    i += 1;
                    if i < args.len() {
                        right_align = Some(args[i].clone());
                    } else {
                        eprintln!("column: option '--table-right' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            't' => table = true,
                            'J' => json = true,
                            's' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    separator = Some(rest);
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        separator = Some(args[i].clone());
                                    } else {
                                        eprintln!(
                                            "column: option requires an argument -- 's'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            'o' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    output_separator = rest;
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        output_separator = args[i].clone();
                                    } else {
                                        eprintln!(
                                            "column: option requires an argument -- 'o'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            'c' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    if let Ok(n) = rest.parse() {
                                        width = Some(n);
                                    } else {
                                        eprintln!("column: invalid width: '{rest}'");
                                        return None;
                                    }
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        if let Ok(n) = args[i].parse() {
                                            width = Some(n);
                                        } else {
                                            eprintln!(
                                                "column: invalid width: '{}'",
                                                args[i]
                                            );
                                            return None;
                                        }
                                    } else {
                                        eprintln!(
                                            "column: option requires an argument -- 'c'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            'N' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    column_names = Some(rest);
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        column_names = Some(args[i].clone());
                                    } else {
                                        eprintln!(
                                            "column: option requires an argument -- 'N'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            'R' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    right_align = Some(rest);
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        right_align = Some(args[i].clone());
                                    } else {
                                        eprintln!(
                                            "column: option requires an argument -- 'R'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            _ => {
                                eprintln!("column: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ => files.push(arg.clone()),
            }
            i += 1;
        }

        Some(ColumnConfig {
            table,
            separator,
            output_separator,
            width,
            column_names,
            right_align,
            json,
            files,
        })
    }
}
