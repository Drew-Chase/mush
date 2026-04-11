const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: ln [OPTION]... TARGET LINK_NAME
  or:  ln [OPTION]... TARGET... DIRECTORY

Create a link to TARGET with the name LINK_NAME, or create links to each
TARGET in DIRECTORY.

  -s, --symbolic         make symbolic links instead of hard links
  -f, --force            remove existing destination files
  -i, --interactive      prompt whether to remove destinations
  -n, --no-dereference   treat LINK_NAME as a normal file if it is a
                           symbolic link to a directory
  -v, --verbose          print name of each linked file
      --help             display this help and exit
      --version          output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LnConfig {
    pub symbolic: bool,
    pub force: bool,
    pub interactive: bool,
    pub verbose: bool,
    pub no_deref: bool,
    pub targets: Vec<String>,
}

impl LnConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = LnConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.targets.push(arg.clone());
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
                println!("ln {VERSION}");
                return None;
            }
            if arg == "--symbolic" {
                config.symbolic = true;
                i += 1;
                continue;
            }
            if arg == "--force" {
                config.force = true;
                i += 1;
                continue;
            }
            if arg == "--interactive" {
                config.interactive = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if arg == "--no-dereference" {
                config.no_deref = true;
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    's' => config.symbolic = true,
                    'f' => config.force = true,
                    'i' => config.interactive = true,
                    'v' => config.verbose = true,
                    'n' => config.no_deref = true,
                    _ => {
                        eprintln!("ln: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
