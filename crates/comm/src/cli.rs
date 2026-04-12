const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: comm [OPTION]... FILE1 FILE2

Compare two sorted files line by line.

With no options, produce three-column output. Column one contains
lines unique to FILE1, column two contains lines unique to FILE2,
and column three contains lines common to both files.

  -1                    suppress column 1 (lines unique to FILE1)
  -2                    suppress column 2 (lines unique to FILE2)
  -3                    suppress column 3 (lines that appear in both)
  -i, --ignore-case     ignore differences in case when comparing
      --output-delimiter=STR  separate columns with STR
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommConfig {
    pub suppress1: bool,
    pub suppress2: bool,
    pub suppress3: bool,
    pub ignore_case: bool,
    pub output_delimiter: Option<String>,
    pub file1: String,
    pub file2: String,
}

impl CommConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = CommConfig::default();
        let mut positionals: Vec<String> = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("comm {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--ignore-case" => config.ignore_case = true,
                _ if arg.starts_with("--output-delimiter=") => {
                    config.output_delimiter =
                        Some(arg.strip_prefix("--output-delimiter=")?.to_string());
                }
                "--output-delimiter" => {
                    i += 1;
                    config.output_delimiter = Some(args.get(i)?.clone());
                }
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    for ch in arg[1..].chars() {
                        match ch {
                            '1' => config.suppress1 = true,
                            '2' => config.suppress2 = true,
                            '3' => config.suppress3 = true,
                            'i' => config.ignore_case = true,
                            _ => {
                                eprintln!("comm: invalid option -- '{ch}'");
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    positionals.push(arg.clone());
                }
            }

            i += 1;
        }

        positionals.extend(args[i..].iter().cloned());

        if positionals.len() != 2 {
            eprintln!("comm: requires exactly two file arguments");
            return None;
        }

        config.file1 = positionals[0].clone();
        config.file2 = positionals[1].clone();

        Some(config)
    }
}
