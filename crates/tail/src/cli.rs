const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: tail [OPTION]... [FILE]...
Print the last 10 lines of each FILE to standard output.
With more than one FILE, precede each with a header giving the file name.

With no FILE, or when FILE is -, read standard input.

  -n, --lines=NUM          output the last NUM lines
  -c, --bytes=NUM          output the last NUM bytes
  -f, --follow             output appended data as the file grows
  -q, --quiet, --silent    never output headers giving file names
  -v, --verbose            always output headers giving file names
      --help               display this help and exit
      --version            output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TailConfig {
    pub lines: usize,
    pub bytes: Option<usize>,
    pub follow: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub files: Vec<String>,
}

impl TailConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = TailConfig {
            lines: 10,
            ..Default::default()
        };
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
                println!("tail {VERSION}");
                return None;
            }
            if arg == "--follow" {
                config.follow = true;
                i += 1;
                continue;
            }
            if arg == "--quiet" || arg == "--silent" {
                config.quiet = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--lines=") {
                if let Ok(n) = val.parse::<usize>() {
                    config.lines = n;
                }
                i += 1;
                continue;
            }
            if arg == "--lines" {
                i += 1;
                if i < args.len()
                    && let Ok(n) = args[i].parse::<usize>()
                {
                    config.lines = n;
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--bytes=") {
                if let Ok(n) = val.parse::<usize>() {
                    config.bytes = Some(n);
                }
                i += 1;
                continue;
            }
            if arg == "--bytes" {
                i += 1;
                if i < args.len()
                    && let Ok(n) = args[i].parse::<usize>()
                {
                    config.bytes = Some(n);
                }
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'f' => config.follow = true,
                    'q' => config.quiet = true,
                    'v' => config.verbose = true,
                    'n' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            if let Ok(n) = rest.parse::<usize>() {
                                config.lines = n;
                            }
                        } else {
                            i += 1;
                            if i < args.len()
                                && let Ok(n) = args[i].parse::<usize>()
                            {
                                config.lines = n;
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    'c' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            if let Ok(n) = rest.parse::<usize>() {
                                config.bytes = Some(n);
                            }
                        } else {
                            i += 1;
                            if i < args.len()
                                && let Ok(n) = args[i].parse::<usize>()
                            {
                                config.bytes = Some(n);
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("tail: invalid option -- '{}'", chars[j]);
                        break;
                    }
                }
                j += 1;
            }
            i += 1;
        }

        Some(config)
    }
}
