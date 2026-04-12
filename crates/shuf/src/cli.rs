const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: shuf [OPTION]... [FILE]
Write a random permutation of the input lines.

  -e             treat each ARG as an input line
  -i LO-HI      treat each number LO through HI as an input line
  -n COUNT       output at most COUNT lines
  -r             output lines can be repeated (with -n)
  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ShufConfig {
    pub echo_args: Vec<String>,
    pub echo_mode: bool,
    pub range: Option<(u64, u64)>,
    pub head_count: Option<usize>,
    pub repeat: bool,
    pub file: Option<String>,
}

impl ShufConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = ShufConfig::default();
        let mut i = 0;
        let mut positional = Vec::new();

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("shuf {VERSION}");
                return None;
            }

            if arg == "-e" {
                config.echo_mode = true;
            } else if arg == "-i" {
                i += 1;
                let range_str = args.get(i)?;
                let (lo, hi) = range_str.split_once('-')?;
                config.range = Some((lo.parse().ok()?, hi.parse().ok()?));
            } else if arg == "-n" {
                i += 1;
                config.head_count = Some(args.get(i)?.parse().ok()?);
            } else if arg == "-r" {
                config.repeat = true;
            } else if arg.starts_with('-') && arg != "-" {
                eprintln!("shuf: invalid option '{arg}'");
            } else {
                positional.push(arg.clone());
            }
            i += 1;
        }

        if config.echo_mode {
            config.echo_args = positional;
        } else {
            config.file = positional.into_iter().next();
        }

        Some(config)
    }
}
