const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: rev [OPTION] [FILE]...
Reverse lines characterwise.

  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RevConfig {
    pub files: Vec<String>,
}

impl RevConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = RevConfig::default();
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

            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("rev {VERSION}");
                return None;
            }

            eprintln!("rev: invalid option '{arg}'");
            i += 1;
        }

        Some(config)
    }
}
