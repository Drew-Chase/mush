const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: chown [OPTION]... OWNER[:GROUP] FILE...
  or:  chown [OPTION]... --reference=RFILE FILE...
Change the owner and/or group of each FILE to OWNER and/or GROUP.

  -R, --recursive      operate on files and directories recursively
  -v, --verbose        output a diagnostic for every file processed
  -c, --changes        like verbose but report only when a change is made
  -f, --silent, --quiet suppress most error messages
  -h, --no-dereference  affect symbolic links instead of referenced file
      --reference=RFILE  use RFILE's owner and group
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChownConfig {
    pub recursive: bool,
    pub verbose: bool,
    pub changes: bool,
    pub quiet: bool,
    pub no_deref: bool,
    pub reference: Option<String>,
    pub owner_group: String,
    pub files: Vec<String>,
}

impl ChownConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = ChownConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;
        let mut positional: Vec<String> = Vec::new();

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                positional.push(arg.clone());
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
                println!("chown {VERSION}");
                return None;
            }
            if arg == "--recursive" {
                config.recursive = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if arg == "--changes" {
                config.changes = true;
                i += 1;
                continue;
            }
            if arg == "--silent" || arg == "--quiet" {
                config.quiet = true;
                i += 1;
                continue;
            }
            if arg == "--no-dereference" {
                config.no_deref = true;
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--reference=") {
                config.reference = Some(val.to_string());
                i += 1;
                continue;
            }
            if arg == "--reference" {
                i += 1;
                if i < args.len() {
                    config.reference = Some(args[i].clone());
                } else {
                    eprintln!("chown: option '--reference' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    'R' => config.recursive = true,
                    'v' => config.verbose = true,
                    'c' => config.changes = true,
                    'f' => config.quiet = true,
                    'h' => config.no_deref = true,
                    _ => {
                        eprintln!("chown: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        if config.reference.is_none() {
            if positional.is_empty() {
                eprintln!("chown: missing operand");
                return None;
            }
            config.owner_group = positional.remove(0);
        }

        if positional.is_empty() {
            eprintln!("chown: missing operand");
            return None;
        }

        config.files = positional;
        Some(config)
    }
}
