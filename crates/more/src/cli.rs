const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: more [OPTION]... [FILE]...

A filter for paging through text one screenful at a time.

With no FILE, or when FILE is -, read standard input.

  -s             squeeze multiple adjacent blank lines into one
  -n NUM         lines per screenful (default: terminal height - 1)
  +NUM           start displaying at line number NUM
      --help     display this help and exit
      --version  output version information and exit

Interactive commands:
  SPACE          display next page
  ENTER          display next line
  q              quit
  h              display help";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MoreConfig {
    pub squeeze: bool,
    pub lines_per_screen: Option<usize>,
    pub start_line: Option<usize>,
    pub files: Vec<String>,
}

impl MoreConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = MoreConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("more {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "-s" => {
                    config.squeeze = true;
                }
                "-n" => {
                    i += 1;
                    if i < args.len() {
                        config.lines_per_screen = args[i].parse().ok();
                    }
                }
                _ if arg.starts_with('+') => {
                    if let Ok(n) = arg[1..].parse::<usize>() {
                        config.start_line = Some(n);
                    }
                }
                _ if arg.starts_with('-') && arg.len() > 1 && arg != "-" => {
                    // Handle -sNUM or -nNUM style
                    let rest = &arg[1..];
                    if rest == "s" {
                        config.squeeze = true;
                    } else if let Some(val) = rest.strip_prefix('n') {
                        if !val.is_empty() {
                            config.lines_per_screen = val.parse().ok();
                        }
                    } else {
                        eprintln!("more: invalid option -- '{rest}'");
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
