const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: dirname [OPTION] NAME...
Output each NAME with its last non-slash component and trailing slashes removed;
if NAME contains no /'s, output '.' (meaning the current directory).

  -z, --zero    end each output line with NUL, not newline
      --help    display this help and exit
      --version output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DirnameConfig {
    pub zero: bool,
    pub names: Vec<String>,
}

impl DirnameConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = DirnameConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.names.push(arg.clone());
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
                println!("dirname {VERSION}");
                return None;
            }
            if arg == "--zero" {
                config.zero = true;
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for c in &chars {
                match c {
                    'z' => config.zero = true,
                    _ => {
                        eprintln!("dirname: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
