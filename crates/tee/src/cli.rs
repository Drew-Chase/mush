const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: tee [OPTION]... [FILE]...
Copy standard input to each FILE, and also to standard output.

  -a, --append    append to the given FILEs, do not overwrite
      --help      display this help and exit
      --version   output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeeConfig {
    pub append: bool,
    pub files: Vec<String>,
}

impl TeeConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = TeeConfig::default();
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
                println!("tee {VERSION}");
                return None;
            }
            if arg == "--append" {
                config.append = true;
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    'a' => config.append = true,
                    _ => {
                        eprintln!("tee: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
