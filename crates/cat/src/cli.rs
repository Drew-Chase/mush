const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help               display this help and exit
      --version            output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatConfig {
    pub number: bool,
    pub number_nonblank: bool,
    pub squeeze_blank: bool,
    pub show_ends: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool,
    pub files: Vec<String>,
}

impl CatConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = CatConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.files.push(arg.clone());
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
                println!("cat {VERSION}");
                return None;
            }
            if arg == "--show-all" {
                config.show_nonprinting = true;
                config.show_ends = true;
                config.show_tabs = true;
                i += 1;
                continue;
            }
            if arg == "--number-nonblank" {
                config.number_nonblank = true;
                i += 1;
                continue;
            }
            if arg == "--show-ends" {
                config.show_ends = true;
                i += 1;
                continue;
            }
            if arg == "--number" {
                config.number = true;
                i += 1;
                continue;
            }
            if arg == "--squeeze-blank" {
                config.squeeze_blank = true;
                i += 1;
                continue;
            }
            if arg == "--show-tabs" {
                config.show_tabs = true;
                i += 1;
                continue;
            }
            if arg == "--show-nonprinting" {
                config.show_nonprinting = true;
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    'A' => {
                        config.show_nonprinting = true;
                        config.show_ends = true;
                        config.show_tabs = true;
                    }
                    'b' => config.number_nonblank = true,
                    'e' => {
                        config.show_nonprinting = true;
                        config.show_ends = true;
                    }
                    'E' => config.show_ends = true,
                    'n' => config.number = true,
                    's' => config.squeeze_blank = true,
                    't' => {
                        config.show_nonprinting = true;
                        config.show_tabs = true;
                    }
                    'T' => config.show_tabs = true,
                    'v' => config.show_nonprinting = true,
                    _ => {
                        eprintln!("cat: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
