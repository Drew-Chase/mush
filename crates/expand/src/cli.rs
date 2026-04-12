const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: expand [OPTION]... [FILE]...

Convert tabs in each FILE to spaces, writing to standard output.
With no FILE, or when FILE is -, read standard input.

  -i, --initial         do not convert tabs after non blanks
  -t, --tabs=NUMBER     have tabs NUMBER characters apart, not 8
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct ExpandConfig {
    pub tab_width: usize,
    pub initial_only: bool,
    pub files: Vec<String>,
}

impl ExpandConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut tab_width: usize = 8;
        let mut initial_only = false;
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
                    println!("expand {VERSION}");
                    return None;
                }
                "--initial" => initial_only = true,
                s if s.starts_with("--tabs=") => {
                    if let Ok(n) = s["--tabs=".len()..].parse() {
                        tab_width = n;
                    } else {
                        eprintln!("expand: invalid tab size: '{}'", &s["--tabs=".len()..]);
                        return None;
                    }
                }
                "--tabs" => {
                    i += 1;
                    if i < args.len() {
                        if let Ok(n) = args[i].parse() {
                            tab_width = n;
                        } else {
                            eprintln!("expand: invalid tab size: '{}'", args[i]);
                            return None;
                        }
                    } else {
                        eprintln!("expand: option '--tabs' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'i' => initial_only = true,
                            't' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    if let Ok(n) = rest.parse() {
                                        tab_width = n;
                                    } else {
                                        eprintln!("expand: invalid tab size: '{rest}'");
                                        return None;
                                    }
                                } else {
                                    i += 1;
                                    if i < args.len() {
                                        if let Ok(n) = args[i].parse() {
                                            tab_width = n;
                                        } else {
                                            eprintln!("expand: invalid tab size: '{}'", args[i]);
                                            return None;
                                        }
                                    } else {
                                        eprintln!("expand: option requires an argument -- 't'");
                                        return None;
                                    }
                                }
                                break;
                            }
                            _ => {
                                eprintln!("expand: invalid option -- '{}'", chars[j]);
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

        Some(ExpandConfig {
            tab_width,
            initial_only,
            files,
        })
    }
}
