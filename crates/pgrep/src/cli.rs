const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: pgrep [OPTIONS] PATTERN
Look up processes by name pattern.

  -l, --list-name    list PID and process name
  -a, --list-full    list PID and full command line
  -c, --count        display count of matching processes
  -d, --delimiter D  set output delimiter (default: newline)
  -f, --full         match against full command line
  -i, --ignore-case  case-insensitive matching
  -x, --exact        require exact match of process name
  -u, --euid USER    match only processes owned by USER
  -n, --newest       select most recently started
  -o, --oldest       select least recently started
      --help         display this help and exit
      --version      output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct PgrepConfig {
    pub list_name: bool,
    pub list_full: bool,
    pub count: bool,
    pub delimiter: String,
    pub full: bool,
    pub ignore_case: bool,
    pub exact: bool,
    pub user_filter: Option<String>,
    pub newest: bool,
    pub oldest: bool,
    pub pattern: String,
}

impl Default for PgrepConfig {
    fn default() -> Self {
        Self {
            list_name: false,
            list_full: false,
            count: false,
            delimiter: "\n".to_string(),
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

impl PgrepConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = PgrepConfig::default();
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
                    println!("pgrep {VERSION}");
                    return None;
                }
                "-l" | "--list-name" => config.list_name = true,
                "-a" | "--list-full" => config.list_full = true,
                "-c" | "--count" => config.count = true,
                "-f" | "--full" => config.full = true,
                "-i" | "--ignore-case" => config.ignore_case = true,
                "-x" | "--exact" => config.exact = true,
                "-n" | "--newest" => config.newest = true,
                "-o" | "--oldest" => config.oldest = true,
                "-d" | "--delimiter" => {
                    i += 1;
                    if i < args.len() {
                        config.delimiter = args[i].clone();
                    } else {
                        eprintln!("pgrep: option '{arg}' requires an argument");
                    }
                }
                "-u" | "--euid" => {
                    i += 1;
                    if i < args.len() {
                        config.user_filter = Some(args[i].clone());
                    } else {
                        eprintln!("pgrep: option '{arg}' requires an argument");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("pgrep: unknown option '{arg}'");
                    } else {
                        positional.push(arg.clone());
                    }
                }
            }
            i += 1;
        }

        if positional.is_empty() {
            eprintln!("pgrep: no matching criteria specified");
            eprintln!("Try 'pgrep --help' for more information.");
            return Some(config);
        }

        config.pattern = positional[0].clone();
        Some(config)
    }
}
