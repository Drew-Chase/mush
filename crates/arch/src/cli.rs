const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = "\
Usage: arch

Print machine architecture.

      --help     display this help and exit
      --version  output version information and exit";

pub struct ArchConfig;

impl ArchConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        for arg in args {
            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("arch {VERSION}");
                return None;
            }
        }
        Some(ArchConfig)
    }
}
