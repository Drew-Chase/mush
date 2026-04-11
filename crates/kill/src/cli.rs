const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: kill [-s SIGNAL | -SIGNAL] PID...
       kill -l [SIGNAL]
       kill -L
Send a signal to a process.

  -s SIGNAL        specify the signal to send
  -l, --list       list signal names, or convert signal number to name
  -L, --table      list signal names in a table
      --help       display this help and exit
      --version    output version information and exit

Signals: HUP(1) INT(2) QUIT(3) KILL(9) USR1(10) USR2(12) TERM(15) CONT(18) STOP(19)";

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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KillConfig {
    pub signal: i32,
    pub list: bool,
    pub table: bool,
    pub pids: Vec<u32>,
}

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

impl KillConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = KillConfig {
            signal: 15, // default SIGTERM
            ..Default::default()
        };
        let mut signal_set = false;
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("kill {VERSION}");
                return None;
            }
            if arg == "--list" || arg == "-l" {
                config.list = true;
                i += 1;
                continue;
            }
            if arg == "--table" || arg == "-L" {
                config.table = true;
                i += 1;
                continue;
            }
            if arg == "-s" {
                i += 1;
                if i < args.len() {
                    let sig_arg = &args[i];
                    if let Ok(num) = sig_arg.parse::<i32>() {
                        config.signal = num;
                        signal_set = true;
                    } else if let Some(num) = parse_signal_name(sig_arg) {
                        config.signal = num;
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
                    config.signal = num;
                    signal_set = true;
                    i += 1;
                    continue;
                }
                if let Some(num) = parse_signal_name(sig_str) {
                    config.signal = num;
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
                config.pids.push(pid);
            } else {
                eprintln!("kill: invalid PID '{arg}'");
            }
            i += 1;
        }

        let _ = signal_set;
        Some(config)
    }
}
