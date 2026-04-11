const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = "\
Usage: yes [STRING]...

Repeatedly output a line with all specified STRING(s), or 'y'.

      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YesConfig {
    pub string: String,
}

impl Default for YesConfig {
    fn default() -> Self {
        Self {
            string: "y".to_string(),
        }
    }
}

impl YesConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        if args.len() == 1 {
            if args[0] == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if args[0] == "--version" {
                println!("yes {VERSION}");
                return None;
            }
        }

        if args.is_empty() {
            Some(YesConfig::default())
        } else {
            Some(YesConfig {
                string: args.join(" "),
            })
        }
    }
}
