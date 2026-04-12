const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: fold [OPTION]... [FILE]...

Wrap input lines in each FILE, writing to standard output.
With no FILE, or when FILE is -, read standard input.

  -b, --bytes           count bytes rather than columns
  -s, --spaces          break at spaces
  -w, --width=WIDTH     use WIDTH columns instead of 80
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct FoldConfig {
    pub width: usize,
    pub bytes: bool,
    pub spaces: bool,
    pub files: Vec<String>,
}

impl FoldConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut width: usize = 80;
        let mut bytes = false;
        let mut spaces = false;
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
                    println!("fold {VERSION}");
                    return None;
                }
                "--bytes" => bytes = true,
                "--spaces" => spaces = true,
                s if s.starts_with("--width=") => {
                    if let Ok(n) = s["--width=".len()..].parse() {
                        width = n;
                    } else {
                        eprintln!("fold: invalid width: '{}'", &s["--width=".len()..]);
                        return None;
                    }
                }
                "--width" => {
                    i += 1;
                    if i < args.len() {
                        if let Ok(n) = args[i].parse() {
                            width = n;
                        } else {
                            eprintln!("fold: invalid width: '{}'", args[i]);
                            return None;
                        }
                    } else {
                        eprintln!("fold: option '--width' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'b' => bytes = true,
                            's' => spaces = true,
                            'w' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    if let Ok(n) = rest.parse() {
                                        width = n;
                                    } else {
                                        eprintln!("fold: invalid width: '{rest}'");
                                        return None;
                                    }
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        if let Ok(n) = args[i].parse() {
                                            width = n;
                                        } else {
                                            eprintln!("fold: invalid width: '{}'", args[i]);
                                            return None;
                                        }
                                    } else {
                                        eprintln!("fold: option requires an argument -- 'w'");
                                        return None;
                                    }
                                }
                                break;
                            }
                            _ => {
                                eprintln!("fold: invalid option -- '{}'", chars[j]);
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

        Some(FoldConfig {
            width,
            bytes,
            spaces,
            files,
        })
    }
}
