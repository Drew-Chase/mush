const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: du [OPTION]... [FILE]...

Summarize device usage of the set of FILEs, recursively for directories.

  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)
  -s, --summarize       display only a total for each argument
  -a, --all             write counts for all files, not just directories
  -c, --total           produce a grand total
  -d, --max-depth N     print the total for a directory only if it is N or
                        fewer levels below the command line argument
      --apparent-size   print apparent sizes rather than device usage
  -b, --bytes           equivalent to --apparent-size, print size in bytes
  -k                    like --block-size=1K
  -m                    like --block-size=1M
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DuConfig {
    pub human_readable: bool,
    pub summarize: bool,
    pub all: bool,
    pub total: bool,
    pub max_depth: Option<usize>,
    pub apparent_size: bool,
    pub bytes: bool,
    pub kilobytes: bool,
    pub megabytes: bool,
    pub files: Vec<String>,
}

impl DuConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = DuConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("du {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--human-readable" => config.human_readable = true,
                "--summarize" => config.summarize = true,
                "--all" => config.all = true,
                "--total" => config.total = true,
                "--apparent-size" => config.apparent_size = true,
                "--bytes" => {
                    config.bytes = true;
                    config.apparent_size = true;
                }
                "--max-depth" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("du: option '--max-depth' requires an argument");
                        return None;
                    }
                    match args[i].parse::<usize>() {
                        Ok(n) => config.max_depth = Some(n),
                        Err(_) => {
                            eprintln!("du: invalid maximum depth '{}'", args[i]);
                            return None;
                        }
                    }
                }
                _ if arg.starts_with("--max-depth=") => {
                    let val = &arg["--max-depth=".len()..];
                    match val.parse::<usize>() {
                        Ok(n) => config.max_depth = Some(n),
                        Err(_) => {
                            eprintln!("du: invalid maximum depth '{val}'");
                            return None;
                        }
                    }
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg != "-" => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'h' => config.human_readable = true,
                            's' => config.summarize = true,
                            'a' => config.all = true,
                            'c' => config.total = true,
                            'b' => {
                                config.bytes = true;
                                config.apparent_size = true;
                            }
                            'k' => config.kilobytes = true,
                            'm' => config.megabytes = true,
                            'd' => {
                                // next chars or next arg
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    match rest.parse::<usize>() {
                                        Ok(n) => config.max_depth = Some(n),
                                        Err(_) => {
                                            eprintln!("du: invalid maximum depth '{rest}'");
                                            return None;
                                        }
                                    }
                                    j = chars.len(); // consumed all remaining
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("du: option requires an argument -- 'd'");
                                        return None;
                                    }
                                    match args[i].parse::<usize>() {
                                        Ok(n) => config.max_depth = Some(n),
                                        Err(_) => {
                                            eprintln!(
                                                "du: invalid maximum depth '{}'",
                                                args[i]
                                            );
                                            return None;
                                        }
                                    }
                                }
                            }
                            _ => {
                                eprintln!("du: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with('-') && arg.starts_with("--") => {
                    eprintln!("du: unrecognized option '{arg}'");
                    return None;
                }
                _ => break,
            }

            i += 1;
        }

        config.files = args[i..].to_vec();

        Some(config)
    }
}
