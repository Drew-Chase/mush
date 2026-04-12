const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: reset [OPTION]
Reset the terminal to a sane state.

  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResetConfig;

impl ResetConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        for arg in args {
            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("reset {VERSION}");
                return None;
            }
            eprintln!("reset: invalid option '{arg}'");
        }
        Some(ResetConfig)
    }
}
