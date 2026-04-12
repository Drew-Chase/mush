use clap::Parser;

const SIGNALS: &[(i32, &str)] = &[
    (1, "HUP"),
    (2, "INT"),
    (3, "QUIT"),
    (9, "KILL"),
    (10, "USR1"),
    (12, "USR2"),
    (15, "TERM"),
    (18, "CONT"),
    (19, "STOP"),
];

fn parse_signal_name(name: &str) -> Option<i32> {
    let upper = name.to_ascii_uppercase();
    let upper = upper.strip_prefix("SIG").unwrap_or(&upper);
    for &(num, sig_name) in SIGNALS {
        if sig_name == upper {
            return Some(num);
        }
    }
    None
}

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "kill",
    about = "Send a signal to a process.",
    version,
    disable_help_flag = true,
    allow_hyphen_values = true
)]
pub struct KillConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Specify the signal to send
    #[arg(short = 's', long, default_value_t = 15)]
    pub signal: i32,

    /// List signal names, or convert signal number to name
    #[arg(short = 'l', long = "list")]
    pub list: bool,

    /// List signal names in a table
    #[arg(short = 'L', long = "table")]
    pub table: bool,

    /// Process IDs
    #[arg(skip)]
    pub pids: Vec<u32>,
}

impl KillConfig {
    /// Custom parsing to handle -9, -KILL, -SIGTERM style signal args
    /// that clap cannot parse natively.
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut signal: i32 = 15;
        let mut signal_set = false;
        let mut list = false;
        let mut table = false;
        let mut pids = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                let _ = Self::try_parse_from(["kill", "--help"]);
                return None;
            }
            if arg == "--version" {
                let _ = Self::try_parse_from(["kill", "--version"]);
                return None;
            }
            if arg == "--list" || arg == "-l" {
                list = true;
                i += 1;
                continue;
            }
            if arg == "--table" || arg == "-L" {
                table = true;
                i += 1;
                continue;
            }
            if arg == "-s" {
                i += 1;
                if i < args.len() {
                    let sig_arg = &args[i];
                    if let Ok(num) = sig_arg.parse::<i32>() {
                        signal = num;
                        signal_set = true;
                    } else if let Some(num) = parse_signal_name(sig_arg) {
                        signal = num;
                        signal_set = true;
                    } else {
                        eprintln!("kill: unknown signal '{sig_arg}'");
                    }
                } else {
                    eprintln!("kill: option '-s' requires an argument");
                }
                i += 1;
                continue;
            }

            // Handle -SIGNAL (e.g., -9, -KILL, -SIGTERM, -TERM)
            if arg.starts_with('-') && arg.len() > 1 {
                let sig_str = &arg[1..];
                if let Ok(num) = sig_str.parse::<i32>() {
                    signal = num;
                    signal_set = true;
                    i += 1;
                    continue;
                }
                if let Some(num) = parse_signal_name(sig_str) {
                    signal = num;
                    signal_set = true;
                    i += 1;
                    continue;
                }
                // Unknown flag
                eprintln!("kill: unknown option '{arg}'");
                i += 1;
                continue;
            }

            // Positional: PID
            if let Ok(pid) = arg.parse::<u32>() {
                pids.push(pid);
            } else {
                eprintln!("kill: invalid PID '{arg}'");
            }
            i += 1;
        }

        let _ = signal_set;
        Some(KillConfig {
            help: None,
            signal,
            list,
            table,
            pids,
        })
    }
}
