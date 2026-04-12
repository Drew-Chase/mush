const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: strings [OPTION]... [FILE]...

Print the printable character sequences that are at least N characters
long (default 4) from each FILE.

With no FILE, or when FILE is -, read standard input.

  -a, --all             scan the whole file (default)
  -n, --bytes N         print sequences of at least N characters
  -t, --radix CHAR      print the offset with radix CHAR (o, x, or d)
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringsConfig {
    pub min_length: usize,
    pub all: bool,
    pub radix: Option<char>,
    pub files: Vec<String>,
}

impl Default for StringsConfig {
    fn default() -> Self {
        Self {
            min_length: 4,
            all: false,
            radix: None,
            files: Vec::new(),
        }
    }
}

impl StringsConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = StringsConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("strings {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--all" => config.all = true,
                "--bytes" => {
                    i += 1;
                    config.min_length = args.get(i)?.parse().ok()?;
                }
                "--radix" => {
                    i += 1;
                    let val = args.get(i)?;
                    let ch = val.chars().next()?;
                    if matches!(ch, 'o' | 'x' | 'd') {
                        config.radix = Some(ch);
                    } else {
                        eprintln!("strings: invalid radix '{ch}'");
                        return None;
                    }
                }
                _ if arg.starts_with("--bytes=") => {
                    config.min_length = arg.strip_prefix("--bytes=")?.parse().ok()?;
                }
                _ if arg.starts_with("--radix=") => {
                    let val = arg.strip_prefix("--radix=")?;
                    let ch = val.chars().next()?;
                    if matches!(ch, 'o' | 'x' | 'd') {
                        config.radix = Some(ch);
                    } else {
                        eprintln!("strings: invalid radix '{ch}'");
                        return None;
                    }
                }
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'a' => config.all = true,
                            'n' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.min_length = rest.parse().ok()?;
                                } else {
                                    i += 1;
                                    config.min_length = args.get(i)?.parse().ok()?;
                                }
                                j = chars.len();
                                continue;
                            }
                            't' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                let val = if !rest.is_empty() {
                                    rest
                                } else {
                                    i += 1;
                                    args.get(i)?.clone()
                                };
                                let ch = val.chars().next()?;
                                if matches!(ch, 'o' | 'x' | 'd') {
                                    config.radix = Some(ch);
                                } else {
                                    eprintln!("strings: invalid radix '{ch}'");
                                    return None;
                                }
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("strings: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ => {
                    config.files.push(arg.clone());
                }
            }

            i += 1;
        }

        config.files.extend(args[i..].iter().cloned());

        Some(config)
    }
}
