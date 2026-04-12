const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: join [OPTION]... FILE1 FILE2

For each pair of input lines with identical join fields, write a line to
standard output. The default join field is the first, delimited by blanks.

  -1 FIELD          join on this FIELD of file 1
  -2 FIELD          join on this FIELD of file 2
  -a FILENUM        also print unpairable lines from file FILENUM
  -v FILENUM        like -a FILENUM, but suppress joined output lines
  -e STRING         replace missing input fields with STRING
  -o FORMAT         obey FORMAT while constructing output line
  -t CHAR           use CHAR as input and output field separator
  -i, --ignore-case ignore differences in case when comparing fields
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinConfig {
    pub field1: usize,
    pub field2: usize,
    pub separator: Option<char>,
    pub unpaired1: bool,
    pub unpaired2: bool,
    pub only_unpaired1: bool,
    pub only_unpaired2: bool,
    pub empty: Option<String>,
    pub format: Option<String>,
    pub ignore_case: bool,
    pub file1: String,
    pub file2: String,
}

impl Default for JoinConfig {
    fn default() -> Self {
        Self {
            field1: 1,
            field2: 1,
            separator: None,
            unpaired1: false,
            unpaired2: false,
            only_unpaired1: false,
            only_unpaired2: false,
            empty: None,
            format: None,
            ignore_case: false,
            file1: String::new(),
            file2: String::new(),
        }
    }
}

impl JoinConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = JoinConfig::default();
        let mut positionals: Vec<String> = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("join {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--ignore-case" => config.ignore_case = true,
                "-1" => {
                    i += 1;
                    config.field1 = args.get(i)?.parse().ok()?;
                }
                "-2" => {
                    i += 1;
                    config.field2 = args.get(i)?.parse().ok()?;
                }
                "-a" => {
                    i += 1;
                    match args.get(i)?.as_str() {
                        "1" => config.unpaired1 = true,
                        "2" => config.unpaired2 = true,
                        _ => {
                            eprintln!("join: invalid file number for -a");
                            return None;
                        }
                    }
                }
                "-v" => {
                    i += 1;
                    match args.get(i)?.as_str() {
                        "1" => config.only_unpaired1 = true,
                        "2" => config.only_unpaired2 = true,
                        _ => {
                            eprintln!("join: invalid file number for -v");
                            return None;
                        }
                    }
                }
                "-e" => {
                    i += 1;
                    config.empty = Some(args.get(i)?.clone());
                }
                "-o" => {
                    i += 1;
                    config.format = Some(args.get(i)?.clone());
                }
                "-t" => {
                    i += 1;
                    let val = args.get(i)?;
                    config.separator = val.chars().next();
                }
                "-i" => config.ignore_case = true,
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    // Not a recognized short option, treat as positional (could be negative number file)
                    positionals.push(arg.clone());
                }
                _ => {
                    positionals.push(arg.clone());
                }
            }

            i += 1;
        }

        positionals.extend(args[i..].iter().cloned());

        if positionals.len() != 2 {
            eprintln!("join: requires exactly two file arguments");
            return None;
        }

        config.file1 = positionals[0].clone();
        config.file2 = positionals[1].clone();

        Some(config)
    }
}
