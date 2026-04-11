const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: stat [OPTION]... FILE...
Display file or file system status.

  -L, --dereference      follow links
  -c, --format=FORMAT    use the specified FORMAT instead of the default
  -t, --terse            print the information in terse form
      --help             display this help and exit
      --version          output version information and exit

The valid format sequences for files:
  %a   access rights in octal
  %b   number of blocks allocated
  %f   raw mode in hex
  %F   file type
  %G   group ID of owner
  %n   file name
  %s   total size, in bytes
  %U   user ID of owner
  %X   time of last access, seconds since Epoch
  %Y   time of last data modification, seconds since Epoch
  %Z   time of last status change, seconds since Epoch";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatConfig {
    pub dereference: bool,
    pub format: Option<String>,
    pub terse: bool,
    pub files: Vec<String>,
}

impl StatConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = StatConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.files.push(arg.clone());
                i += 1;
                continue;
            }

            if arg == "--" {
                parsing_flags = false;
                i += 1;
                continue;
            }

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("stat {VERSION}");
                return None;
            }
            if arg == "--dereference" {
                config.dereference = true;
                i += 1;
                continue;
            }
            if arg == "--terse" {
                config.terse = true;
                i += 1;
                continue;
            }
            if arg == "--format" {
                i += 1;
                if i < args.len() {
                    config.format = Some(args[i].clone());
                } else {
                    eprintln!("stat: option '--format' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(fmt) = arg.strip_prefix("--format=") {
                config.format = Some(fmt.to_string());
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'L' => config.dereference = true,
                    't' => config.terse = true,
                    'c' => {
                        // -c FORMAT (next arg or rest of current)
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.format = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.format = Some(args[i].clone());
                            } else {
                                eprintln!("stat: option '-c' requires an argument");
                                return None;
                            }
                        }
                        j = chars.len(); // consumed rest
                        continue;
                    }
                    _ => {
                        eprintln!("stat: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        if config.files.is_empty() {
            eprintln!("stat: missing operand");
            return None;
        }

        Some(config)
    }
}
