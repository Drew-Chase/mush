const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: timeout [OPTION] DURATION COMMAND [ARG]...
  or:  timeout [OPTION]

Start COMMAND, and kill it if still running after DURATION.

DURATION is a number with optional suffix: 's' for seconds (the default),
'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an
integer.

  -s, --signal=SIGNAL    specify the signal to be sent on timeout;
                         SIGNAL may be a name like 'HUP' or a number;
                         see 'kill -l' for a list of signals (default: TERM)
  -k, --kill-after=DURATION
                         also send a KILL signal if COMMAND is still running
                         this long after the initial signal was sent
      --preserve-status  exit with the same status as COMMAND, even when the
                         command times out
  -v, --verbose          diagnose to stderr any signal sent upon timeout
      --help             display this help and exit
      --version          output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct TimeoutConfig {
    pub duration_secs: f64,
    pub signal: String,
    pub kill_after: Option<f64>,
    pub preserve_status: bool,
    pub verbose: bool,
    pub command: Vec<String>,
}

/// Parse a duration string like "5", "1m", "0.5s", "2h", "1d" into seconds.
pub fn parse_duration(s: &str) -> Option<f64> {
    let (num_str, multiplier) = if let Some(n) = s.strip_suffix('d') {
        (n, 86400.0)
    } else if let Some(n) = s.strip_suffix('h') {
        (n, 3600.0)
    } else if let Some(n) = s.strip_suffix('m') {
        (n, 60.0)
    } else if let Some(n) = s.strip_suffix('s') {
        (n, 1.0)
    } else {
        (s, 1.0)
    };

    num_str.parse::<f64>().ok().map(|v| v * multiplier)
}

impl TimeoutConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut signal = String::from("TERM");
        let mut kill_after: Option<f64> = None;
        let mut preserve_status = false;
        let mut verbose = false;
        let mut positionals: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("timeout {VERSION}");
                    return None;
                }
                "--preserve-status" => preserve_status = true,
                "-v" | "--verbose" => verbose = true,
                "-s" => {
                    i += 1;
                    if i < args.len() {
                        signal = args[i].clone();
                    } else {
                        eprintln!("timeout: option '-s' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with("--signal=") => {
                    signal = arg.strip_prefix("--signal=").unwrap().to_string();
                }
                "--signal" => {
                    i += 1;
                    if i < args.len() {
                        signal = args[i].clone();
                    } else {
                        eprintln!("timeout: option '--signal' requires an argument");
                        return None;
                    }
                }
                "-k" => {
                    i += 1;
                    if i < args.len() {
                        match parse_duration(&args[i]) {
                            Some(d) => kill_after = Some(d),
                            None => {
                                eprintln!("timeout: invalid duration '{}'", args[i]);
                                return None;
                            }
                        }
                    } else {
                        eprintln!("timeout: option '-k' requires an argument");
                        return None;
                    }
                }
                _ if arg.starts_with("--kill-after=") => {
                    let val = arg.strip_prefix("--kill-after=").unwrap();
                    match parse_duration(val) {
                        Some(d) => kill_after = Some(d),
                        None => {
                            eprintln!("timeout: invalid duration '{val}'");
                            return None;
                        }
                    }
                }
                "--kill-after" => {
                    i += 1;
                    if i < args.len() {
                        match parse_duration(&args[i]) {
                            Some(d) => kill_after = Some(d),
                            None => {
                                eprintln!("timeout: invalid duration '{}'", args[i]);
                                return None;
                            }
                        }
                    } else {
                        eprintln!("timeout: option '--kill-after' requires an argument");
                        return None;
                    }
                }
                _ => {
                    // First positional is duration, rest is command
                    positionals.extend(args[i..].iter().cloned());
                    break;
                }
            }
            i += 1;
        }

        if positionals.is_empty() {
            eprintln!("timeout: missing operand");
            return None;
        }

        let duration_secs = match parse_duration(&positionals[0]) {
            Some(d) => d,
            None => {
                eprintln!("timeout: invalid time interval '{}'", positionals[0]);
                return None;
            }
        };

        if positionals.len() < 2 {
            eprintln!("timeout: missing operand");
            return None;
        }

        let command = positionals[1..].to_vec();

        Some(TimeoutConfig {
            duration_secs,
            signal,
            kill_after,
            preserve_status,
            verbose,
            command,
        })
    }
}
