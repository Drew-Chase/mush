const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: unexpand [OPTION]... [FILE]...

Convert blanks in each FILE to tabs, writing to standard output.
With no FILE, or when FILE is -, read standard input.

  -a, --all             convert all blanks, instead of just initial blanks
      --first-only      convert only leading sequences of blanks
  -t, --tabs=NUMBER     have tabs NUMBER characters apart, not 8
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpandConfig {
    pub all: bool,
    pub tab_width: usize,
    pub first_only: bool,
    pub files: Vec<String>,
}

impl UnexpandConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut all = false;
        let mut tab_width: usize = 8;
        let mut first_only = false;
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
                    println!("unexpand {VERSION}");
                    return None;
                }
                "--all" => all = true,
                "--first-only" => first_only = true,
                s if s.starts_with("--tabs=") => {
                    if let Ok(n) = s["--tabs=".len()..].parse() {
                        tab_width = n;
                    } else {
                        eprintln!("unexpand: invalid tab size: '{}'", &s["--tabs=".len()..]);
                        return None;
                    }
                }
                "--tabs" => {
                    i += 1;
                    if i < args.len() {
                        if let Ok(n) = args[i].parse() {
                            tab_width = n;
                        } else {
                            eprintln!("unexpand: invalid tab size: '{}'", args[i]);
                            return None;
                        }
                    } else {
                        eprintln!("unexpand: option '--tabs' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'a' => all = true,
                            't' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    if let Ok(n) = rest.parse() {
                                        tab_width = n;
                                    } else {
                                        eprintln!("unexpand: invalid tab size: '{rest}'");
                                        return None;
                                    }
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        if let Ok(n) = args[i].parse() {
                                            tab_width = n;
                                        } else {
                                            eprintln!(
                                                "unexpand: invalid tab size: '{}'",
                                                args[i]
                                            );
                                            return None;
                                        }
                                    } else {
                                        eprintln!(
                                            "unexpand: option requires an argument -- 't'"
                                        );
                                        return None;
                                    }
                                }
                                break;
                            }
                            _ => {
                                eprintln!("unexpand: invalid option -- '{}'", chars[j]);
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

        Some(UnexpandConfig {
            all,
            tab_width,
            first_only,
            files,
        })
    }
}
