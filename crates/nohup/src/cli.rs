const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: nohup COMMAND [ARG]...

Run COMMAND, ignoring hangup signals.

If standard output is a terminal, redirect it to 'nohup.out'.

      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NohupConfig {
    pub command: Vec<String>,
}

impl NohupConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        if args.is_empty() {
            eprintln!("nohup: missing operand");
            return Some(NohupConfig::default());
        }

        match args[0].as_str() {
            "--help" => {
                println!("{HELP_TEXT}");
                None
            }
            "--version" => {
                println!("nohup {VERSION}");
                None
            }
            _ => {
                // Everything is the command + args
                Some(NohupConfig { command: args.to_vec() })
            }
        }
    }
}
