const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = "\
Usage: whoami

Print the current user name.

      --help     display this help and exit
      --version  output version information and exit";

pub struct WhoamiConfig;

impl WhoamiConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        for arg in args {
            if arg == "--help" { println!("{HELP_TEXT}"); return None; }
            if arg == "--version" { println!("whoami {VERSION}"); return None; }
        }
        Some(WhoamiConfig)
    }
}
