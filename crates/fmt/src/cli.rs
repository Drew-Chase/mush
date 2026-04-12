const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: fmt [OPTION]... [FILE]...

Reformat each paragraph in the FILE(s), writing to standard output.
With no FILE, or when FILE is -, read standard input.

  -p, --prefix=STRING   reformat only lines beginning with STRING
  -s, --split-only      split long lines, but do not refill
  -u, --uniform-spacing one space between words, two after sentences
  -w, --width=WIDTH     maximum line width (default 75)
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct FmtConfig {
    pub width: usize,
    pub split_only: bool,
    pub uniform: bool,
    pub prefix: Option<String>,
    pub files: Vec<String>,
}

impl FmtConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut width: usize = 75;
        let mut split_only = false;
        let mut uniform = false;
        let mut prefix: Option<String> = None;
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
                    println!("fmt {VERSION}");
                    return None;
                }
                "--split-only" => split_only = true,
                "--uniform-spacing" => uniform = true,
                s if s.starts_with("--width=") => {
                    if let Ok(n) = s["--width=".len()..].parse() {
                        width = n;
                    } else {
                        eprintln!("fmt: invalid width: '{}'", &s["--width=".len()..]);
                        return None;
                    }
                }
                "--width" => {
                    i += 1;
                    if i < args.len() {
                        if let Ok(n) = args[i].parse() {
                            width = n;
                        } else {
                            eprintln!("fmt: invalid width: '{}'", args[i]);
                            return None;
                        }
                    } else {
                        eprintln!("fmt: option '--width' requires an argument");
                        return None;
                    }
                }
                s if s.starts_with("--prefix=") => {
                    prefix = Some(s["--prefix=".len()..].to_string());
                }
                "--prefix" => {
                    i += 1;
                    if i < args.len() {
                        prefix = Some(args[i].clone());
                    } else {
                        eprintln!("fmt: option '--prefix' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            's' => split_only = true,
                            'u' => uniform = true,
                            'w' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    if let Ok(n) = rest.parse() {
                                        width = n;
                                    } else {
                                        eprintln!("fmt: invalid width: '{rest}'");
                                        return None;
                                    }
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        if let Ok(n) = args[i].parse() {
                                            width = n;
                                        } else {
                                            eprintln!("fmt: invalid width: '{}'", args[i]);
                                            return None;
                                        }
                                    } else {
                                        eprintln!("fmt: option requires an argument -- 'w'");
                                        return None;
                                    }
                                }
                                break;
                            }
                            'p' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    prefix = Some(rest);
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        prefix = Some(args[i].clone());
                                    } else {
                                        eprintln!("fmt: option requires an argument -- 'p'");
                                        return None;
                                    }
                                }
                                break;
                            }
                            _ => {
                                eprintln!("fmt: invalid option -- '{}'", chars[j]);
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

        Some(FmtConfig {
            width,
            split_only,
            uniform,
            prefix,
            files,
        })
    }
}
