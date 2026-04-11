const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: sleep NUMBER[SUFFIX]...
  or:  sleep OPTION

Pause for NUMBER seconds.  SUFFIX may be 's' for seconds (the default),
'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an
integer.  Given two or more arguments, pause for the amount of time
specified by the sum of their values.

      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct SleepConfig {
    pub seconds: f64,
}

impl SleepConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        if args.len() == 1 {
            if args[0] == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if args[0] == "--version" {
                println!("sleep {VERSION}");
                return None;
            }
        }

        if args.is_empty() {
            eprintln!("sleep: missing operand");
            return Some(SleepConfig { seconds: 0.0 });
        }

        let mut total = 0.0;

        for arg in args {
            let (num_str, multiplier) = if let Some(n) = arg.strip_suffix('d') {
                (n, 86400.0)
            } else if let Some(n) = arg.strip_suffix('h') {
                (n, 3600.0)
            } else if let Some(n) = arg.strip_suffix('m') {
                (n, 60.0)
            } else if let Some(n) = arg.strip_suffix('s') {
                (n, 1.0)
            } else {
                (arg.as_str(), 1.0)
            };

            let value: f64 = match num_str.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("sleep: invalid time interval '{arg}'");
                    return Some(SleepConfig { seconds: 0.0 });
                }
            };

            total += value * multiplier;
        }

        Some(SleepConfig { seconds: total })
    }
}
