const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: head [OPTION]... [FILE]...

Print the first 10 lines of each FILE to standard output.
With more than one FILE, precede each with a header giving the file name.

With no FILE, or when FILE is -, read standard input.

  -c, --bytes=[-]NUM    print the first NUM bytes of each file
  -n, --lines=[-]NUM    print the first NUM lines instead of first 10
  -q, --quiet, --silent never print headers giving file names
  -v, --verbose         always print headers giving file names
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadConfig {
    pub lines: usize,
    pub bytes: Option<usize>,
    pub quiet: bool,
    pub verbose: bool,
    pub files: Vec<String>,
}

impl Default for HeadConfig {
    fn default() -> Self {
        Self {
            lines: 10,
            bytes: None,
            quiet: false,
            verbose: false,
            files: Vec::new(),
        }
    }
}

impl HeadConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = HeadConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("head {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            if let Some(val) = arg.strip_prefix("--lines=") {
                config.lines = val.parse().unwrap_or(10);
                config.bytes = None;
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--bytes=") {
                config.bytes = Some(val.parse().unwrap_or(0));
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--lines" | "-n" => {
                    i += 1;
                    if i < args.len() {
                        config.lines = args[i].parse().unwrap_or(10);
                        config.bytes = None;
                    }
                }
                "--bytes" | "-c" => {
                    i += 1;
                    if i < args.len() {
                        config.bytes = Some(args[i].parse().unwrap_or(0));
                    }
                }
                "-q" | "--quiet" | "--silent" => {
                    config.quiet = true;
                    config.verbose = false;
                }
                "-v" | "--verbose" => {
                    config.verbose = true;
                    config.quiet = false;
                }
                _ if arg.starts_with('-') && arg != "-" => {
                    // Handle -n5, -c10 style
                    let flag = &arg[1..2];
                    let val = &arg[2..];
                    if flag == "n" && !val.is_empty() {
                        config.lines = val.parse().unwrap_or(10);
                        config.bytes = None;
                    } else if flag == "c" && !val.is_empty() {
                        config.bytes = Some(val.parse().unwrap_or(0));
                    } else {
                        eprintln!("head: invalid option -- '{}'", &arg[1..]);
                        return None;
                    }
                }
                _ => break,
            }

            i += 1;
        }

        config.files = args[i..].to_vec();
        Some(config)
    }
}
