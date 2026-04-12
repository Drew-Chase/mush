const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: paste [OPTION]... [FILE]...

Write lines consisting of the sequentially corresponding lines from
each FILE, separated by TABs, to standard output.

With no FILE, or when FILE is -, read standard input.

  -d, --delimiters LIST  reuse characters from LIST instead of TABs
  -s, --serial           paste one file at a time instead of in parallel
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasteConfig {
    pub delimiters: String,
    pub serial: bool,
    pub files: Vec<String>,
}

impl Default for PasteConfig {
    fn default() -> Self {
        Self {
            delimiters: "\t".to_string(),
            serial: false,
            files: Vec::new(),
        }
    }
}

impl PasteConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = PasteConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("paste {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--serial" => config.serial = true,
                "--delimiters" => {
                    i += 1;
                    config.delimiters = args.get(i)?.clone();
                }
                _ if arg.starts_with("--delimiters=") => {
                    config.delimiters = arg.strip_prefix("--delimiters=")?.to_string();
                }
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            's' => config.serial = true,
                            'd' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.delimiters = rest;
                                } else {
                                    i += 1;
                                    config.delimiters = args.get(i)?.clone();
                                }
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("paste: invalid option -- '{}'", chars[j]);
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

        // Remaining args after -- are positionals
        config.files.extend(args[i..].iter().cloned());

        Some(config)
    }
}
