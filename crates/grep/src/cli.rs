const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: grep [OPTION]... PATTERN [FILE]...

Search for PATTERN in each FILE or standard input.

  -E, --extended-regexp    PATTERN is an extended regular expression
  -F, --fixed-strings      PATTERN is a set of newline-separated strings
  -i, --ignore-case        ignore case distinctions in patterns and data
  -v, --invert-match       select non-matching lines
  -w, --word-regexp        match only whole words
  -x, --line-regexp        match only whole lines
  -c, --count              print only a count of selected lines per FILE
  -l, --files-with-matches print only names of FILEs with selected lines
  -L, --files-without-match print only names of FILEs with no selected lines
  -n, --line-number        prefix each output line with line number
  -H, --with-filename      print file name with output lines
  -h, --no-filename        suppress the file name prefix on output
  -o, --only-matching      show only nonempty parts of lines that match
  -q, --quiet, --silent    suppress all normal output
  -r, -R, --recursive      search directories recursively
  -A, --after-context NUM  print NUM lines of trailing context
  -B, --before-context NUM print NUM lines of leading context
  -C, --context NUM        print NUM lines of output context
  -m, --max-count NUM      stop after NUM selected lines
      --color[=WHEN]       use markers to highlight the matching strings;
                             WHEN is 'always', 'never', or 'auto'
      --include GLOB       search only files that match GLOB
      --exclude GLOB       skip files that match GLOB
      --exclude-dir DIR    skip directories that match DIR
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GrepConfig {
    pub pattern: String,
    pub files: Vec<String>,
    pub ignore_case: bool,
    pub invert: bool,
    pub word_regexp: bool,
    pub line_regexp: bool,
    pub count: bool,
    pub files_with_matches: bool,
    pub files_without_match: bool,
    pub line_number: bool,
    pub with_filename: bool,
    pub no_filename: bool,
    pub only_matching: bool,
    pub quiet: bool,
    pub recursive: bool,
    pub after_context: usize,
    pub before_context: usize,
    pub context: usize,
    pub max_count: Option<usize>,
    pub color: bool,
    pub fixed_strings: bool,
    pub extended_regexp: bool,
    pub include_glob: Vec<String>,
    pub exclude_glob: Vec<String>,
    pub exclude_dir: Vec<String>,
}

impl GrepConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = GrepConfig::default();
        let mut i = 0;
        let mut pattern_set = false;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("grep {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            // Long options with = form
            if let Some(rest) = arg.strip_prefix("--color=") {
                config.color = match rest {
                    "always" => true,
                    "never" => false,
                    "auto" => atty_stdout(),
                    _ => {
                        eprintln!("grep: invalid argument '{rest}' for '--color'");
                        return None;
                    }
                };
                i += 1;
                continue;
            }
            if arg == "--color" {
                config.color = atty_stdout();
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--after-context=") {
                config.after_context = parse_num_arg("--after-context", rest)?;
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--before-context=") {
                config.before_context = parse_num_arg("--before-context", rest)?;
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--context=") {
                config.context = parse_num_arg("--context", rest)?;
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--max-count=") {
                config.max_count = Some(parse_num_arg("--max-count", rest)?);
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--include=") {
                config.include_glob.push(rest.to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--exclude=") {
                config.exclude_glob.push(rest.to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--exclude-dir=") {
                config.exclude_dir.push(rest.to_string());
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--extended-regexp" => config.extended_regexp = true,
                "--fixed-strings" => config.fixed_strings = true,
                "--ignore-case" => config.ignore_case = true,
                "--invert-match" => config.invert = true,
                "--word-regexp" => config.word_regexp = true,
                "--line-regexp" => config.line_regexp = true,
                "--count" => config.count = true,
                "--files-with-matches" => config.files_with_matches = true,
                "--files-without-match" => config.files_without_match = true,
                "--line-number" => config.line_number = true,
                "--with-filename" => config.with_filename = true,
                "--no-filename" => config.no_filename = true,
                "--only-matching" => config.only_matching = true,
                "--quiet" | "--silent" => config.quiet = true,
                "--recursive" => config.recursive = true,
                "--after-context" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--after-context' requires an argument");
                        return None;
                    }
                    config.after_context = parse_num_arg("--after-context", &args[i])?;
                }
                "--before-context" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--before-context' requires an argument");
                        return None;
                    }
                    config.before_context = parse_num_arg("--before-context", &args[i])?;
                }
                "--context" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--context' requires an argument");
                        return None;
                    }
                    config.context = parse_num_arg("--context", &args[i])?;
                }
                "--max-count" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--max-count' requires an argument");
                        return None;
                    }
                    config.max_count = Some(parse_num_arg("--max-count", &args[i])?);
                }
                "--include" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--include' requires an argument");
                        return None;
                    }
                    config.include_glob.push(args[i].clone());
                }
                "--exclude" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--exclude' requires an argument");
                        return None;
                    }
                    config.exclude_glob.push(args[i].clone());
                }
                "--exclude-dir" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("grep: option '--exclude-dir' requires an argument");
                        return None;
                    }
                    config.exclude_dir.push(args[i].clone());
                }
                _ if arg.starts_with('-') && arg != "-" && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'E' => config.extended_regexp = true,
                            'F' => config.fixed_strings = true,
                            'i' => config.ignore_case = true,
                            'v' => config.invert = true,
                            'w' => config.word_regexp = true,
                            'x' => config.line_regexp = true,
                            'c' => config.count = true,
                            'l' => config.files_with_matches = true,
                            'L' => config.files_without_match = true,
                            'n' => config.line_number = true,
                            'H' => config.with_filename = true,
                            'h' => config.no_filename = true,
                            'o' => config.only_matching = true,
                            'q' => config.quiet = true,
                            'r' | 'R' => config.recursive = true,
                            'A' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("grep: option requires an argument -- 'A'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.after_context = parse_num_arg("-A", &val)?;
                                j = chars.len();
                                continue;
                            }
                            'B' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("grep: option requires an argument -- 'B'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.before_context = parse_num_arg("-B", &val)?;
                                j = chars.len();
                                continue;
                            }
                            'C' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("grep: option requires an argument -- 'C'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.context = parse_num_arg("-C", &val)?;
                                j = chars.len();
                                continue;
                            }
                            'm' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("grep: option requires an argument -- 'm'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.max_count = Some(parse_num_arg("-m", &val)?);
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("grep: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("grep: unrecognized option '{arg}'");
                    return None;
                }
                _ => break,
            }

            i += 1;
        }

        // First non-flag arg is the pattern
        if i < args.len() && !pattern_set {
            config.pattern = args[i].clone();
            pattern_set = true;
            i += 1;
        }

        if !pattern_set {
            eprintln!("grep: missing pattern");
            return None;
        }

        // Remaining args are files
        config.files = args[i..].to_vec();

        Some(config)
    }
}

fn parse_num_arg(flag: &str, val: &str) -> Option<usize> {
    match val.parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("grep: invalid argument '{val}' for '{flag}'");
            None
        }
    }
}

fn atty_stdout() -> bool {
    false
}
