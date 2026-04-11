const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: chmod [OPTION]... MODE[,MODE]... FILE...
  or:  chmod [OPTION]... --reference=RFILE FILE...
Change the mode of each FILE to MODE.

  -R, --recursive        change files and directories recursively
  -v, --verbose          output a diagnostic for every file processed
  -c, --changes          like verbose but report only when a change is made
  -f, --silent, --quiet  suppress most error messages
      --help             display this help and exit
      --version          output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChmodConfig {
    pub recursive: bool,
    pub verbose: bool,
    pub changes: bool,
    pub quiet: bool,
    pub mode: String,
    pub files: Vec<String>,
}

impl ChmodConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = ChmodConfig::default();
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
                println!("chmod {VERSION}");
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

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    'R' => config.recursive = true,
                    'v' => config.verbose = true,
                    'c' => config.changes = true,
                    'f' => config.quiet = true,
                    _ => {
                        eprintln!("chmod: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        // First positional is mode, rest are files
        if positional.is_empty() {
            eprintln!("chmod: missing operand");
            return None;
        }

        config.mode = positional.remove(0);

        if positional.is_empty() {
            eprintln!("chmod: missing operand after '{}'", config.mode);
            return None;
        }

        config.files = positional;
        Some(config)
    }
}
