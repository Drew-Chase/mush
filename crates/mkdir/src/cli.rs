const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.

  -m, --mode=MODE  set file mode (as in chmod), not a=rwx - umask
  -p, --parents    no error if existing, make parent directories as needed
  -v, --verbose    print a message for each created directory
      --help       display this help and exit
      --version    output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MkdirConfig {
    pub parents: bool,
    pub verbose: bool,
    pub mode: Option<u32>,
    pub directories: Vec<String>,
}

impl MkdirConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = MkdirConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.directories.push(arg.clone());
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
                println!("mkdir {VERSION}");
                return None;
            }
            if arg == "--parents" {
                config.parents = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if let Some(mode_str) = arg.strip_prefix("--mode=") {
                config.mode = parse_mode(mode_str);
                i += 1;
                continue;
            }
            if arg == "--mode" {
                i += 1;
                if i < args.len() {
                    config.mode = parse_mode(&args[i]);
                }
                i += 1;
                continue;
            }

            // Short flags: -p, -v, -m, or combined like -pv, -pm
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'p' => config.parents = true,
                    'v' => config.verbose = true,
                    'm' => {
                        // Mode value: rest of this arg or next arg
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.mode = parse_mode(&rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.mode = parse_mode(&args[i]);
                            }
                        }
                        // 'm' consumes the rest of the short flag group
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("mkdir: invalid option -- '{}'", chars[j]);
                        config.directories.push(arg.clone());
                        j = chars.len();
                        continue;
                    }
                }
                j += 1;
            }
            i += 1;
        }

        Some(config)
    }
}

fn parse_mode(s: &str) -> Option<u32> {
    u32::from_str_radix(s, 8).ok()
}
