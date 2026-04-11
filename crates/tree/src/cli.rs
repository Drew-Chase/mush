const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: tree [OPTIONS] [DIRECTORY]...

List contents of directories in a tree-like format.

  -a, --all          include hidden files
  -d, --dirs-only    list directories only
  -f, --full-path    print the full path prefix for each file
  -L, --level DEPTH  descend only DEPTH directories deep
  -I, --exclude PAT  exclude files matching pattern
  -P, --pattern PAT  list only files matching pattern
  -s, --size         print the size of each file
  -h, --human-readable  print size in human readable format
  -D, --date         print the date of last modification
      --dirsfirst    list directories before files
      --noreport     omit the file and directory report at end
  -C, --color        turn colorization on always
  -J, --json         output as JSON
      --help         display this help and exit
      --version      output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreeConfig {
    pub all: bool,
    pub dirs_only: bool,
    pub full_path: bool,
    pub level: Option<usize>,
    pub exclude: Option<String>,
    pub pattern: Option<String>,
    pub show_size: bool,
    pub human_readable: bool,
    pub show_date: bool,
    pub dirs_first: bool,
    pub no_report: bool,
    pub color: bool,
    pub json: bool,
    pub paths: Vec<String>,
}

impl TreeConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = TreeConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("tree {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--all" => config.all = true,
                "--dirs-only" => config.dirs_only = true,
                "--full-path" => config.full_path = true,
                "--dirsfirst" => config.dirs_first = true,
                "--noreport" => config.no_report = true,
                "--color" => config.color = true,
                "--json" => config.json = true,
                "--size" => config.show_size = true,
                "--human-readable" => config.human_readable = true,
                "--date" => config.show_date = true,
                "--level" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("tree: option '--level' requires an argument");
                        return None;
                    }
                    match args[i].parse::<usize>() {
                        Ok(n) => config.level = Some(n),
                        Err(_) => {
                            eprintln!("tree: invalid level '{}'", args[i]);
                            return None;
                        }
                    }
                }
                "--exclude" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("tree: option '--exclude' requires an argument");
                        return None;
                    }
                    config.exclude = Some(args[i].clone());
                }
                "--pattern" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("tree: option '--pattern' requires an argument");
                        return None;
                    }
                    config.pattern = Some(args[i].clone());
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'a' => config.all = true,
                            'd' => config.dirs_only = true,
                            'f' => config.full_path = true,
                            's' => config.show_size = true,
                            'h' => config.human_readable = true,
                            'D' => config.show_date = true,
                            'C' => config.color = true,
                            'J' => config.json = true,
                            'L' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    match rest.parse::<usize>() {
                                        Ok(n) => config.level = Some(n),
                                        Err(_) => {
                                            eprintln!("tree: invalid level '{rest}'");
                                            return None;
                                        }
                                    }
                                    j = chars.len();
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!(
                                            "tree: option requires an argument -- 'L'"
                                        );
                                        return None;
                                    }
                                    match args[i].parse::<usize>() {
                                        Ok(n) => config.level = Some(n),
                                        Err(_) => {
                                            eprintln!(
                                                "tree: invalid level '{}'",
                                                args[i]
                                            );
                                            return None;
                                        }
                                    }
                                }
                            }
                            'I' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.exclude = Some(rest);
                                    j = chars.len();
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!(
                                            "tree: option requires an argument -- 'I'"
                                        );
                                        return None;
                                    }
                                    config.exclude = Some(args[i].clone());
                                }
                            }
                            'P' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.pattern = Some(rest);
                                    j = chars.len();
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!(
                                            "tree: option requires an argument -- 'P'"
                                        );
                                        return None;
                                    }
                                    config.pattern = Some(args[i].clone());
                                }
                            }
                            _ => {
                                eprintln!("tree: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("tree: unrecognized option '{arg}'");
                    return None;
                }
                _ => break,
            }

            i += 1;
        }

        config.paths = args[i..].to_vec();

        Some(config)
    }
}
