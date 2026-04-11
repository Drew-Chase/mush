const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: cut OPTION... [FILE]...

Print selected parts of lines from each FILE to standard output.

With no FILE, or when FILE is -, read standard input.

Mandatory arguments to long options are mandatory for short options too.
  -b, --bytes=LIST        select only these bytes
  -c, --characters=LIST   select only these characters
  -d, --delimiter=DELIM   use DELIM instead of TAB for field delimiter
  -f, --fields=LIST       select only these fields
  -s, --only-delimited    do not print lines not containing delimiters
      --complement         complement the set of selected bytes, characters
                             or fields
      --output-delimiter=STRING  use STRING as the output delimiter
      --help     display this help and exit
      --version  output version information and exit

Use one, and only one of -b, -c or -f. Each LIST is made up of one range,
or many ranges separated by commas.

Each range is one of:
  N     N'th byte, character or field, counted from 1
  N-    from N'th byte, character or field, to end of line
  N-M   from N'th to M'th (included) byte, character or field
  -M    from first to M'th (included) byte, character or field";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    Single(usize),
    FromTo(usize, usize),
    From(usize),
    To(usize),
}

impl Range {
    /// Check whether a 1-based index is included in this range.
    pub fn contains(&self, idx: usize) -> bool {
        match self {
            Range::Single(n) => idx == *n,
            Range::FromTo(a, b) => idx >= *a && idx <= *b,
            Range::From(a) => idx >= *a,
            Range::To(b) => idx >= 1 && idx <= *b,
        }
    }
}

/// Parse a range specification like "1,3-5,7-,-3"
pub fn parse_ranges(spec: &str) -> Option<Vec<Range>> {
    let mut ranges = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return None;
        }
        if let Some(dash_pos) = part.find('-') {
            let left = &part[..dash_pos];
            let right = &part[dash_pos + 1..];
            if left.is_empty() && right.is_empty() {
                return None;
            } else if left.is_empty() {
                let b: usize = right.parse().ok()?;
                ranges.push(Range::To(b));
            } else if right.is_empty() {
                let a: usize = left.parse().ok()?;
                ranges.push(Range::From(a));
            } else {
                let a: usize = left.parse().ok()?;
                let b: usize = right.parse().ok()?;
                ranges.push(Range::FromTo(a, b));
            }
        } else {
            let n: usize = part.parse().ok()?;
            ranges.push(Range::Single(n));
        }
    }
    if ranges.is_empty() { None } else { Some(ranges) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CutMode {
    Bytes(Vec<Range>),
    Characters(Vec<Range>),
    Fields(Vec<Range>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutConfig {
    pub mode: CutMode,
    pub delimiter: char,
    pub output_delimiter: Option<String>,
    pub only_delimited: bool,
    pub complement: bool,
    pub files: Vec<String>,
}

impl CutConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut mode: Option<CutMode> = None;
        let mut delimiter: char = '\t';
        let mut output_delimiter: Option<String> = None;
        let mut only_delimited = false;
        let mut complement = false;
        let mut files: Vec<String> = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("cut {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--complement" => complement = true,
                "--only-delimited" => only_delimited = true,
                _ if arg.starts_with("--bytes=") => {
                    let spec = arg.strip_prefix("--bytes=")?;
                    mode = Some(CutMode::Bytes(parse_ranges(spec)?));
                }
                _ if arg.starts_with("--characters=") => {
                    let spec = arg.strip_prefix("--characters=")?;
                    mode = Some(CutMode::Characters(parse_ranges(spec)?));
                }
                _ if arg.starts_with("--fields=") => {
                    let spec = arg.strip_prefix("--fields=")?;
                    mode = Some(CutMode::Fields(parse_ranges(spec)?));
                }
                _ if arg.starts_with("--delimiter=") => {
                    let val = arg.strip_prefix("--delimiter=")?;
                    delimiter = val.chars().next()?;
                }
                _ if arg.starts_with("--output-delimiter=") => {
                    let val = arg.strip_prefix("--output-delimiter=")?.to_string();
                    output_delimiter = Some(val);
                }
                "--bytes" | "-b" => {
                    i += 1;
                    mode = Some(CutMode::Bytes(parse_ranges(args.get(i)?)?));
                }
                "--characters" | "-c" => {
                    i += 1;
                    mode = Some(CutMode::Characters(parse_ranges(args.get(i)?)?));
                }
                "--fields" | "-f" => {
                    i += 1;
                    mode = Some(CutMode::Fields(parse_ranges(args.get(i)?)?));
                }
                "--delimiter" | "-d" => {
                    i += 1;
                    delimiter = args.get(i)?.chars().next()?;
                }
                "-s" => only_delimited = true,
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    // Handle combined short options like -f1, -d:, -b1-3
                    let flag = arg.as_bytes()[1] as char;
                    let rest = &arg[2..];
                    match flag {
                        'b' => {
                            if rest.is_empty() {
                                i += 1;
                                mode = Some(CutMode::Bytes(parse_ranges(args.get(i)?)?));
                            } else {
                                mode = Some(CutMode::Bytes(parse_ranges(rest)?));
                            }
                        }
                        'c' => {
                            if rest.is_empty() {
                                i += 1;
                                mode = Some(CutMode::Characters(parse_ranges(args.get(i)?)?));
                            } else {
                                mode = Some(CutMode::Characters(parse_ranges(rest)?));
                            }
                        }
                        'f' => {
                            if rest.is_empty() {
                                i += 1;
                                mode = Some(CutMode::Fields(parse_ranges(args.get(i)?)?));
                            } else {
                                mode = Some(CutMode::Fields(parse_ranges(rest)?));
                            }
                        }
                        'd' => {
                            if rest.is_empty() {
                                i += 1;
                                delimiter = args.get(i)?.chars().next()?;
                            } else {
                                delimiter = rest.chars().next()?;
                            }
                        }
                        's' => only_delimited = true,
                        _ => {
                            eprintln!("cut: invalid option -- '{flag}'");
                            return None;
                        }
                    }
                }
                _ => {
                    files.push(arg.clone());
                }
            }

            i += 1;
        }

        // Remaining args after --
        files.extend(args[i..].iter().cloned());

        let mode = mode.or_else(|| {
            eprintln!("cut: you must specify a list of bytes, characters, or fields");
            None
        })?;

        Some(CutConfig {
            mode,
            delimiter,
            output_delimiter,
            only_delimited,
            complement,
            files,
        })
    }
}
