const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: xargs [OPTION]... [COMMAND [INITIAL-ARGS]]

Build and execute command lines from standard input.

  -0, --null              input items are terminated by NUL, not whitespace
  -d, --delimiter CHAR    input items are terminated by CHAR
  -n, --max-args N        use at most N arguments per command line
  -I, --replace STR       replace STR in COMMAND with input items
  -L, --max-lines N       use at most N input lines per command line
  -P, --max-procs N       run at most N processes at a time (default 1)
  -t, --verbose           print commands to stderr before executing
  -p, --interactive       prompt the user before executing each command
  -r, --no-run-if-empty   if there are no input items, do not run the command
  -s, --max-chars N       limit length of command line to N chars
      --help              display this help and exit
      --version           output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XargsConfig {
    pub null: bool,
    pub delimiter: Option<char>,
    pub max_args: Option<usize>,
    pub replace: Option<String>,
    pub max_lines: Option<usize>,
    pub max_procs: usize,
    pub verbose: bool,
    pub interactive: bool,
    pub no_run_if_empty: bool,
    pub max_chars: Option<usize>,
    pub command: Vec<String>,
}

impl XargsConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = XargsConfig {
            max_procs: 1,
            ..Default::default()
        };
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("xargs {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            // Long options with = form
            if let Some(rest) = arg.strip_prefix("--delimiter=") {
                if let Some(ch) = rest.chars().next() {
                    config.delimiter = Some(ch);
                } else {
                    eprintln!("xargs: option '--delimiter' requires a non-empty argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--max-args=") {
                config.max_args = Some(parse_num_arg("--max-args", rest)?);
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--replace=") {
                config.replace = Some(rest.to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--max-lines=") {
                config.max_lines = Some(parse_num_arg("--max-lines", rest)?);
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--max-procs=") {
                config.max_procs = parse_num_arg("--max-procs", rest)?;
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--max-chars=") {
                config.max_chars = Some(parse_num_arg("--max-chars", rest)?);
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--null" => config.null = true,
                "--delimiter" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--delimiter' requires an argument");
                        return None;
                    }
                    if let Some(ch) = args[i].chars().next() {
                        config.delimiter = Some(ch);
                    } else {
                        eprintln!("xargs: option '--delimiter' requires a non-empty argument");
                        return None;
                    }
                }
                "--max-args" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--max-args' requires an argument");
                        return None;
                    }
                    config.max_args = Some(parse_num_arg("--max-args", &args[i])?);
                }
                "--replace" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--replace' requires an argument");
                        return None;
                    }
                    config.replace = Some(args[i].clone());
                }
                "--max-lines" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--max-lines' requires an argument");
                        return None;
                    }
                    config.max_lines = Some(parse_num_arg("--max-lines", &args[i])?);
                }
                "--max-procs" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--max-procs' requires an argument");
                        return None;
                    }
                    config.max_procs = parse_num_arg("--max-procs", &args[i])?;
                }
                "--verbose" => config.verbose = true,
                "--interactive" => config.interactive = true,
                "--no-run-if-empty" => config.no_run_if_empty = true,
                "--max-chars" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("xargs: option '--max-chars' requires an argument");
                        return None;
                    }
                    config.max_chars = Some(parse_num_arg("--max-chars", &args[i])?);
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg != "-" => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            '0' => config.null = true,
                            'd' => {
                                if j + 1 < chars.len() {
                                    config.delimiter = Some(chars[j + 1]);
                                    j = chars.len();
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 'd'");
                                        return None;
                                    }
                                    if let Some(ch) = args[i].chars().next() {
                                        config.delimiter = Some(ch);
                                    } else {
                                        eprintln!("xargs: option '-d' requires a non-empty argument");
                                        return None;
                                    }
                                    j += 1;
                                    continue;
                                }
                            }
                            'n' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 'n'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.max_args = Some(parse_num_arg("-n", &val)?);
                                j = chars.len();
                                continue;
                            }
                            'I' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 'I'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.replace = Some(val);
                                j = chars.len();
                                continue;
                            }
                            'L' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 'L'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.max_lines = Some(parse_num_arg("-L", &val)?);
                                j = chars.len();
                                continue;
                            }
                            'P' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 'P'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.max_procs = parse_num_arg("-P", &val)?;
                                j = chars.len();
                                continue;
                            }
                            's' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("xargs: option requires an argument -- 's'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.max_chars = Some(parse_num_arg("-s", &val)?);
                                j = chars.len();
                                continue;
                            }
                            't' => config.verbose = true,
                            'p' => config.interactive = true,
                            'r' => config.no_run_if_empty = true,
                            _ => {
                                eprintln!("xargs: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("xargs: unrecognized option '{arg}'");
                    return None;
                }
                _ => break,
            }

            i += 1;
        }

        // Remaining args are the command
        config.command = args[i..].to_vec();

        // Default command is "echo"
        if config.command.is_empty() {
            config.command = vec!["echo".to_string()];
        }

        Some(config)
    }
}

fn parse_num_arg(flag: &str, val: &str) -> Option<usize> {
    match val.parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("xargs: invalid argument '{val}' for '{flag}'");
            None
        }
    }
}
