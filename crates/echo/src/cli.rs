const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: echo [SHORT-OPTION]... [STRING]...
  or:  echo LONG-OPTION

Echo the STRING(s) to standard output.

  -n             do not output the trailing newline
  -e             enable interpretation of backslash escapes
  -E             disable interpretation of backslash escapes (default)
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EchoConfig {
    pub no_newline: bool,
    pub interpret_escapes: bool,
    pub args: Vec<String>,
}

impl EchoConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        if args.len() == 1 {
            if args[0] == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if args[0] == "--version" {
                println!("echo {VERSION}");
                return None;
            }
        }

        let mut config = EchoConfig::default();
        let mut positional_start = 0;

        for arg in args {
            if !arg.starts_with('-') || arg == "-" {
                break;
            }

            let flag_chars = &arg[1..];
            if flag_chars.is_empty() || !flag_chars.chars().all(|c| matches!(c, 'n' | 'e' | 'E')) {
                break;
            }

            for c in flag_chars.chars() {
                match c {
                    'n' => config.no_newline = true,
                    'e' => config.interpret_escapes = true,
                    'E' => config.interpret_escapes = false,
                    _ => unreachable!(),
                }
            }

            positional_start += 1;
        }

        config.args = args[positional_start..].to_vec();
        Some(config)
    }
}
