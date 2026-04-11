const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: realpath [OPTION]... FILE...
Print the resolved absolute file name; all but the last component must exist.

  -e, --canonicalize-existing  all components of the path must exist
  -m, --canonicalize-missing   no path components need exist or be a directory
  -s, --strip, --no-symlinks   don't expand symlinks
  -q, --quiet                  suppress most error messages
  -z, --zero                   end each output line with NUL, not newline
      --help                   display this help and exit
      --version                output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RealpathConfig {
    pub canonicalize_existing: bool,
    pub canonicalize_missing: bool,
    pub no_symlinks: bool,
    pub quiet: bool,
    pub zero: bool,
    pub files: Vec<String>,
}

impl RealpathConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = RealpathConfig::default();
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
                println!("realpath {VERSION}");
                return None;
            }

            match arg.as_str() {
                "--canonicalize-existing" => config.canonicalize_existing = true,
                "--canonicalize-missing" => config.canonicalize_missing = true,
                "--strip" | "--no-symlinks" => config.no_symlinks = true,
                "--quiet" => config.quiet = true,
                "--zero" => config.zero = true,
                _ if arg.starts_with("--") => {
                    eprintln!("realpath: unrecognized option '{arg}'");
                }
                _ => {
                    // Short flags
                    for ch in arg[1..].chars() {
                        match ch {
                            'e' => config.canonicalize_existing = true,
                            'm' => config.canonicalize_missing = true,
                            's' => config.no_symlinks = true,
                            'q' => config.quiet = true,
                            'z' => config.zero = true,
                            _ => eprintln!("realpath: invalid option -- '{ch}'"),
                        }
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
