const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: pkill [OPTIONS] PATTERN
Signal processes by name pattern.

  -s, --signal SIG   signal to send (default: TERM)
  -f, --full         match against full command line
  -i, --ignore-case  case-insensitive matching
  -x, --exact        require exact match of process name
  -u, --euid USER    match only processes owned by USER
  -n, --newest       select most recently started
  -o, --oldest       select least recently started
      --help         display this help and exit
      --version      output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct PkillConfig {
    pub signal: String,
    pub full: bool,
    pub ignore_case: bool,
    pub exact: bool,
    pub user_filter: Option<String>,
    pub newest: bool,
    pub oldest: bool,
    pub pattern: String,
}

impl Default for PkillConfig {
    fn default() -> Self {
        Self {
            signal: "TERM".to_string(),
            full: false,
            ignore_case: false,
            exact: false,
            user_filter: None,
            newest: false,
            oldest: false,
            pattern: String::new(),
        }
    }
}

impl PkillConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = PkillConfig::default();
        let mut i = 0;
        let mut positional = Vec::new();

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("pkill {VERSION}");
                    return None;
                }
                "-s" | "--signal" => {
                    i += 1;
                    if i < args.len() {
                        config.signal = args[i].clone();
                    } else {
                        eprintln!("pkill: option '{arg}' requires an argument");
                    }
                }
                "-f" | "--full" => config.full = true,
                "-i" | "--ignore-case" => config.ignore_case = true,
                "-x" | "--exact" => config.exact = true,
                "-n" | "--newest" => config.newest = true,
                "-o" | "--oldest" => config.oldest = true,
                "-u" | "--euid" => {
                    i += 1;
                    if i < args.len() {
                        config.user_filter = Some(args[i].clone());
                    } else {
                        eprintln!("pkill: option '{arg}' requires an argument");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("pkill: unknown option '{arg}'");
                    } else {
                        positional.push(arg.clone());
                    }
                }
            }
            i += 1;
        }

        if positional.is_empty() {
            eprintln!("pkill: no matching criteria specified");
            eprintln!("Try 'pkill --help' for more information.");
            return Some(config);
        }

        config.pattern = positional[0].clone();
        Some(config)
    }
}
