const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: bc [OPTIONS] [FILE]...
An arbitrary precision calculator language.

  -l             define the standard math library
  -V, --version  output version information and exit
  -h, --help     display this help and exit

Operators: + - * / % ^ ( )
Special variable: scale (number of decimal digits)
Math library (-l): s(x) c(x) a(x) l(x) e(x) sqrt(x)";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BcConfig {
    pub math_lib: bool,
    pub files: Vec<String>,
}

impl BcConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = BcConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("bc {VERSION}");
                return None;
            }

            if arg == "-l" {
                config.math_lib = true;
            } else if arg.starts_with('-') {
                eprintln!("bc: invalid option '{arg}'");
            } else {
                config.files.push(arg.clone());
            }
            i += 1;
        }

        Some(config)
    }
}
