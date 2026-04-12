const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: xxd [OPTIONS] [FILE]
Make a hex dump of a file or stdin.

  -c COLS        number of octets per line (default 16)
  -g BYTES       group size in bytes (default 2)
  -l LEN         stop after LEN octets
  -s SEEK        start at SEEK bytes offset
  -u             use upper case hex letters
  -p             output in plain hex dump style
  -r             reverse: convert hex dump to binary
  -i             output in C include file style
  -b             binary digit dump
  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxdConfig {
    pub cols: usize,
    pub group_size: usize,
    pub length: Option<usize>,
    pub seek: usize,
    pub upper: bool,
    pub plain: bool,
    pub reverse: bool,
    pub include: bool,
    pub bits: bool,
    pub file: Option<String>,
}

impl Default for XxdConfig {
    fn default() -> Self {
        Self {
            cols: 16,
            group_size: 2,
            length: None,
            seek: 0,
            upper: false,
            plain: false,
            reverse: false,
            include: false,
            bits: false,
            file: None,
        }
    }
}

impl XxdConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = XxdConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("xxd {VERSION}");
                return None;
            }

            if arg == "-c" {
                i += 1;
                config.cols = args.get(i)?.parse().ok()?;
            } else if arg == "-g" {
                i += 1;
                config.group_size = args.get(i)?.parse().ok()?;
            } else if arg == "-l" {
                i += 1;
                config.length = Some(args.get(i)?.parse().ok()?);
            } else if arg == "-s" {
                i += 1;
                config.seek = args.get(i)?.parse().ok()?;
            } else if arg == "-u" {
                config.upper = true;
            } else if arg == "-p" {
                config.plain = true;
            } else if arg == "-r" {
                config.reverse = true;
            } else if arg == "-i" {
                config.include = true;
            } else if arg == "-b" {
                config.bits = true;
            } else if arg.starts_with('-') {
                eprintln!("xxd: invalid option '{arg}'");
            } else {
                config.file = Some(arg.clone());
            }
            i += 1;
        }

        Some(config)
    }
}
