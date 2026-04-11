const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: readlink [OPTION]... FILE...
Print value of a symbolic link or canonical file name.

  -f, --canonicalize            canonicalize by following every symlink in
                                every component of the given name recursively;
                                all but the last component must exist
  -e, --canonicalize-existing   canonicalize by following every symlink in
                                every component of the given name recursively,
                                all components must exist
  -m, --canonicalize-missing    canonicalize by following every symlink in
                                every component of the given name recursively,
                                without requirements on components existence
  -n, --no-newline              do not output the trailing delimiter
  -z, --zero                    end each output line with NUL, not newline
      --help                    display this help and exit
      --version                 output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadlinkConfig {
    pub canonicalize: bool,
    pub canonicalize_existing: bool,
    pub canonicalize_missing: bool,
    pub no_newline: bool,
    pub zero: bool,
    pub files: Vec<String>,
}

impl ReadlinkConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = ReadlinkConfig::default();
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
                println!("readlink {VERSION}");
                return None;
            }

            match arg.as_str() {
                "--canonicalize" => config.canonicalize = true,
                "--canonicalize-existing" => config.canonicalize_existing = true,
                "--canonicalize-missing" => config.canonicalize_missing = true,
                "--no-newline" => config.no_newline = true,
                "--zero" => config.zero = true,
                _ if arg.starts_with("--") => {
                    eprintln!("readlink: unrecognized option '{arg}'");
                }
                _ => {
                    // Short flags
                    for ch in arg[1..].chars() {
                        match ch {
                            'f' => config.canonicalize = true,
                            'e' => config.canonicalize_existing = true,
                            'm' => config.canonicalize_missing = true,
                            'n' => config.no_newline = true,
                            'z' => config.zero = true,
                            _ => eprintln!("readlink: invalid option -- '{ch}'"),
                        }
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
