const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: sort [OPTION]... [FILE]...

Write sorted concatenation of all FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -b, --ignore-leading-blanks  ignore leading blanks
  -d, --dictionary-order       consider only blanks and alphanumeric characters
  -f, --ignore-case            fold lower case to upper case characters
  -n, --numeric-sort           compare according to string numerical value
  -h, --human-numeric-sort     compare human readable numbers (e.g., 2K 1G)
  -r, --reverse                reverse the result of comparisons
  -k, --key=KEYDEF             sort via a key; KEYDEF gives location and type
  -t, --field-separator=SEP    use SEP instead of non-blank to blank transition
  -u, --unique                 with -c, check for strict ordering;
                                 without -c, output only the first of an equal run
  -s, --stable                 stabilize sort by disabling last-resort comparison
  -o, --output=FILE            write result to FILE instead of standard output
  -c, --check                  check for sorted input; do not sort
  -m, --merge                  merge already sorted files; do not sort
      --help     display this help and exit
      --version  output version information and exit

KEYDEF is F[,F] where F is a field number (origin 1). Use -t to specify a
field separator.";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SortKey {
    pub start_field: usize,
    pub end_field: Option<usize>,
}

impl SortKey {
    pub fn parse(s: &str) -> Option<Self> {
        if let Some((start, end)) = s.split_once(',') {
            let start_field = start.parse::<usize>().ok()?;
            let end_field = end.parse::<usize>().ok()?;
            if start_field == 0 || end_field == 0 {
                return None;
            }
            Some(SortKey { start_field, end_field: Some(end_field) })
        } else {
            let start_field = s.parse::<usize>().ok()?;
            if start_field == 0 {
                return None;
            }
            Some(SortKey { start_field, end_field: None })
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SortConfig {
    pub reverse: bool,
    pub numeric: bool,
    pub human_numeric: bool,
    pub ignore_case: bool,
    pub dictionary: bool,
    pub ignore_blanks: bool,
    pub key: Vec<SortKey>,
    pub separator: Option<char>,
    pub unique: bool,
    pub stable: bool,
    pub output: Option<String>,
    pub check: bool,
    pub merge: bool,
    pub files: Vec<String>,
}

impl SortConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = SortConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("sort {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            // Long options with = form
            if let Some(rest) = arg.strip_prefix("--key=") {
                match SortKey::parse(rest) {
                    Some(k) => config.key.push(k),
                    None => {
                        eprintln!("sort: invalid key specification: '{rest}'");
                        return None;
                    }
                }
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--field-separator=") {
                let mut chars = rest.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => config.separator = Some(c),
                    _ => {
                        eprintln!("sort: separator must be exactly one character");
                        return None;
                    }
                }
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--output=") {
                config.output = Some(rest.to_string());
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--reverse" => config.reverse = true,
                "--numeric-sort" => config.numeric = true,
                "--human-numeric-sort" => config.human_numeric = true,
                "--ignore-case" => config.ignore_case = true,
                "--dictionary-order" => config.dictionary = true,
                "--ignore-leading-blanks" => config.ignore_blanks = true,
                "--unique" => config.unique = true,
                "--stable" => config.stable = true,
                "--check" => config.check = true,
                "--merge" => config.merge = true,
                "--key" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("sort: option '--key' requires an argument");
                        return None;
                    }
                    match SortKey::parse(&args[i]) {
                        Some(k) => config.key.push(k),
                        None => {
                            eprintln!("sort: invalid key specification: '{}'", args[i]);
                            return None;
                        }
                    }
                }
                "--field-separator" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("sort: option '--field-separator' requires an argument");
                        return None;
                    }
                    let mut chars = args[i].chars();
                    match (chars.next(), chars.next()) {
                        (Some(c), None) => config.separator = Some(c),
                        _ => {
                            eprintln!("sort: separator must be exactly one character");
                            return None;
                        }
                    }
                }
                "--output" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("sort: option '--output' requires an argument");
                        return None;
                    }
                    config.output = Some(args[i].clone());
                }
                _ if arg.starts_with('-') && arg != "-" && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'r' => config.reverse = true,
                            'n' => config.numeric = true,
                            'h' => config.human_numeric = true,
                            'f' => config.ignore_case = true,
                            'd' => config.dictionary = true,
                            'b' => config.ignore_blanks = true,
                            'u' => config.unique = true,
                            's' => config.stable = true,
                            'c' => config.check = true,
                            'm' => config.merge = true,
                            'k' => {
                                // Rest of the short arg or next arg is the key
                                let keydef = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("sort: option requires an argument -- 'k'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                match SortKey::parse(&keydef) {
                                    Some(k) => config.key.push(k),
                                    None => {
                                        eprintln!("sort: invalid key specification: '{keydef}'");
                                        return None;
                                    }
                                }
                                // consumed rest of short flags
                                j = chars.len();
                                continue;
                            }
                            't' => {
                                let sep = if j + 1 < chars.len() {
                                    if j + 2 < chars.len() {
                                        eprintln!("sort: separator must be exactly one character");
                                        return None;
                                    }
                                    chars[j + 1]
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("sort: option requires an argument -- 't'");
                                        return None;
                                    }
                                    let mut c = args[i].chars();
                                    match (c.next(), c.next()) {
                                        (Some(ch), None) => ch,
                                        _ => {
                                            eprintln!("sort: separator must be exactly one character");
                                            return None;
                                        }
                                    }
                                };
                                config.separator = Some(sep);
                                j = chars.len();
                                continue;
                            }
                            'o' => {
                                let outfile = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("sort: option requires an argument -- 'o'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.output = Some(outfile);
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("sort: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("sort: unrecognized option '{arg}'");
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
