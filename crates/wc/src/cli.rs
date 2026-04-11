const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: wc [OPTION]... [FILE]...

Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified. A word is a non-zero-length sequence of
printable characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

  -c, --bytes            print the byte counts
  -m, --chars            print the character counts
  -l, --lines            print the newline counts
  -L, --max-line-length  print the maximum display width
  -w, --words            print the word counts
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WcConfig {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
    pub max_line_length: bool,
    pub files: Vec<String>,
}

impl WcConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = WcConfig::default();
        let mut any_flag = false;
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("wc {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--lines" => { config.lines = true; any_flag = true; }
                "--words" => { config.words = true; any_flag = true; }
                "--bytes" => { config.bytes = true; any_flag = true; }
                "--chars" => { config.chars = true; any_flag = true; }
                "--max-line-length" => { config.max_line_length = true; any_flag = true; }
                _ if arg.starts_with('-') && arg != "-" => {
                    for c in arg[1..].chars() {
                        match c {
                            'l' => { config.lines = true; any_flag = true; }
                            'w' => { config.words = true; any_flag = true; }
                            'c' => { config.bytes = true; any_flag = true; }
                            'm' => { config.chars = true; any_flag = true; }
                            'L' => { config.max_line_length = true; any_flag = true; }
                            _ => {
                                eprintln!("wc: invalid option -- '{c}'");
                                return None;
                            }
                        }
                    }
                }
                _ => break,
            }

            i += 1;
        }

        config.files = args[i..].to_vec();

        if !any_flag {
            config.lines = true;
            config.words = true;
            config.bytes = true;
        }

        Some(config)
    }
}
