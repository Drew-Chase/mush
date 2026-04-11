const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: find [PATH...] [EXPRESSION]

Search for files in a directory hierarchy.

Options:
  -maxdepth N        Descend at most N levels
  -mindepth N        Do not apply tests at levels less than N
  -name PATTERN      Base of file name matches shell glob PATTERN
  -iname PATTERN     Like -name but case-insensitive
  -type TYPE         File type: f (file), d (directory), l (symlink)
  -size SPEC         File size (+N, -N, or N with suffix c/k/M/G)
  -empty             File is empty and is a regular file or directory
  -newer FILE        File was modified more recently than FILE
  -path PATTERN      File path matches shell glob PATTERN
  -regex PATTERN     File path matches regular expression PATTERN
  -mtime N           File was modified N*24 hours ago (+N, -N, or N)
  -mmin N            File was modified N minutes ago (+N, -N, or N)
  -perm MODE         File permission bits are exactly MODE (octal)
  -not EXPR          Negate the following expression
  ! EXPR             Same as -not
  -o                 OR: combines with previous expression
  -a                 AND: combines with previous expression (implicit)
  -print             Print full file name (default action)
  -print0            Print full file name followed by NUL
  -delete            Delete files
  -exec CMD {} ;     Execute CMD with {} replaced by file path
  -exec CMD {} +     Execute CMD with all paths appended
  --help             Display this help and exit
  --version          Output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    Name(String),
    IName(String),
    Type(FileType),
    Size(SizeSpec),
    Empty,
    Newer(String),
    Path(String),
    Regex(String),
    Not(Box<Predicate>),
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
    MaxDepth(usize),
    MinDepth(usize),
    Mtime(i64),
    Mmin(i64),
    Perm(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SizeSpec {
    pub cmp: Cmp,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmp {
    Exact,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Print,
    Print0,
    Delete,
    Exec(Vec<String>),
    ExecPlus(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindConfig {
    pub paths: Vec<String>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
    pub max_depth: Option<usize>,
    pub min_depth: Option<usize>,
}

impl Default for FindConfig {
    fn default() -> Self {
        Self {
            paths: vec![".".to_string()],
            predicates: Vec::new(),
            actions: vec![Action::Print],
            max_depth: None,
            min_depth: None,
        }
    }
}

impl FindConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = FindConfig {
            paths: Vec::new(),
            predicates: Vec::new(),
            actions: Vec::new(),
            max_depth: None,
            min_depth: None,
        };

        if args.is_empty() {
            config.paths.push(".".to_string());
            config.actions.push(Action::Print);
            return Some(config);
        }

        let mut i = 0;

        // Check for --help / --version first
        for arg in args {
            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("find {VERSION}");
                return None;
            }
        }

        // Collect leading paths: args that don't start with '-' and aren't '!'
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with('-') || arg == "!" || arg == "(" || arg == ")" {
                break;
            }
            config.paths.push(arg.clone());
            i += 1;
        }

        if config.paths.is_empty() {
            config.paths.push(".".to_string());
        }

        // Parse predicates and actions
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "-maxdepth" => {
                    i += 1;
                    let n = parse_usize_arg("-maxdepth", args.get(i)?)?;
                    config.max_depth = Some(n);
                    config.predicates.push(Predicate::MaxDepth(n));
                }
                "-mindepth" => {
                    i += 1;
                    let n = parse_usize_arg("-mindepth", args.get(i)?)?;
                    config.min_depth = Some(n);
                    config.predicates.push(Predicate::MinDepth(n));
                }
                "-name" => {
                    i += 1;
                    let pat = args.get(i)?.clone();
                    config.predicates.push(Predicate::Name(pat));
                }
                "-iname" => {
                    i += 1;
                    let pat = args.get(i)?.clone();
                    config.predicates.push(Predicate::IName(pat));
                }
                "-type" => {
                    i += 1;
                    let t = args.get(i)?;
                    let ft = match t.as_str() {
                        "f" => FileType::File,
                        "d" => FileType::Dir,
                        "l" => FileType::Symlink,
                        _ => {
                            eprintln!("find: unknown type '{t}'");
                            return None;
                        }
                    };
                    config.predicates.push(Predicate::Type(ft));
                }
                "-size" => {
                    i += 1;
                    let spec = parse_size_spec(args.get(i)?)?;
                    config.predicates.push(Predicate::Size(spec));
                }
                "-empty" => {
                    config.predicates.push(Predicate::Empty);
                }
                "-newer" => {
                    i += 1;
                    let file = args.get(i)?.clone();
                    config.predicates.push(Predicate::Newer(file));
                }
                "-path" => {
                    i += 1;
                    let pat = args.get(i)?.clone();
                    config.predicates.push(Predicate::Path(pat));
                }
                "-regex" => {
                    i += 1;
                    let pat = args.get(i)?.clone();
                    config.predicates.push(Predicate::Regex(pat));
                }
                "-mtime" => {
                    i += 1;
                    let n = parse_signed_arg("-mtime", args.get(i)?)?;
                    config.predicates.push(Predicate::Mtime(n));
                }
                "-mmin" => {
                    i += 1;
                    let n = parse_signed_arg("-mmin", args.get(i)?)?;
                    config.predicates.push(Predicate::Mmin(n));
                }
                "-perm" => {
                    i += 1;
                    let mode = parse_perm_arg(args.get(i)?)?;
                    config.predicates.push(Predicate::Perm(mode));
                }
                "-not" | "!" => {
                    // Parse the next predicate and wrap in Not
                    i += 1;
                    if i >= args.len() {
                        eprintln!("find: expected expression after '-not'");
                        return None;
                    }
                    let inner = parse_single_predicate(args, &mut i)?;
                    config.predicates.push(Predicate::Not(Box::new(inner)));
                    i += 1;
                    continue;
                }
                "-o" => {
                    // OR: combine last predicate with next one
                    // We'll handle this as a marker; for simplicity, store in predicates list
                    // and handle in evaluation
                    if config.predicates.is_empty() {
                        eprintln!("find: expected expression before '-o'");
                        return None;
                    }
                    i += 1;
                    if i >= args.len() {
                        eprintln!("find: expected expression after '-o'");
                        return None;
                    }
                    let left = config.predicates.pop().unwrap();
                    let right = parse_single_predicate(args, &mut i)?;
                    config.predicates.push(Predicate::Or(Box::new(left), Box::new(right)));
                    i += 1;
                    continue;
                }
                "-a" => {
                    // AND is implicit, just skip
                }
                "-print" => {
                    config.actions.push(Action::Print);
                }
                "-print0" => {
                    config.actions.push(Action::Print0);
                }
                "-delete" => {
                    config.actions.push(Action::Delete);
                }
                "-exec" => {
                    i += 1;
                    let mut cmd_parts = Vec::new();
                    let mut found_terminator = false;
                    while i < args.len() {
                        if args[i] == ";" {
                            found_terminator = true;
                            break;
                        }
                        if args[i] == "+" {
                            // ExecPlus mode
                            config.actions.push(Action::ExecPlus(cmd_parts));
                            cmd_parts = Vec::new();
                            found_terminator = true;
                            break;
                        }
                        cmd_parts.push(args[i].clone());
                        i += 1;
                    }
                    if !found_terminator {
                        eprintln!("find: missing argument to '-exec'");
                        return None;
                    }
                    if !cmd_parts.is_empty() {
                        config.actions.push(Action::Exec(cmd_parts));
                    }
                }
                _ => {
                    eprintln!("find: unknown predicate '{arg}'");
                    return None;
                }
            }

            i += 1;
        }

        if config.actions.is_empty() {
            config.actions.push(Action::Print);
        }

        Some(config)
    }
}

fn parse_single_predicate(args: &[String], i: &mut usize) -> Option<Predicate> {
    let arg = &args[*i];
    match arg.as_str() {
        "-name" => {
            *i += 1;
            Some(Predicate::Name(args.get(*i)?.clone()))
        }
        "-iname" => {
            *i += 1;
            Some(Predicate::IName(args.get(*i)?.clone()))
        }
        "-type" => {
            *i += 1;
            let t = args.get(*i)?;
            let ft = match t.as_str() {
                "f" => FileType::File,
                "d" => FileType::Dir,
                "l" => FileType::Symlink,
                _ => {
                    eprintln!("find: unknown type '{t}'");
                    return None;
                }
            };
            Some(Predicate::Type(ft))
        }
        "-size" => {
            *i += 1;
            let spec = parse_size_spec(args.get(*i)?)?;
            Some(Predicate::Size(spec))
        }
        "-empty" => Some(Predicate::Empty),
        "-newer" => {
            *i += 1;
            Some(Predicate::Newer(args.get(*i)?.clone()))
        }
        "-path" => {
            *i += 1;
            Some(Predicate::Path(args.get(*i)?.clone()))
        }
        "-regex" => {
            *i += 1;
            Some(Predicate::Regex(args.get(*i)?.clone()))
        }
        "-mtime" => {
            *i += 1;
            let n = parse_signed_arg("-mtime", args.get(*i)?)?;
            Some(Predicate::Mtime(n))
        }
        "-mmin" => {
            *i += 1;
            let n = parse_signed_arg("-mmin", args.get(*i)?)?;
            Some(Predicate::Mmin(n))
        }
        "-perm" => {
            *i += 1;
            let mode = parse_perm_arg(args.get(*i)?)?;
            Some(Predicate::Perm(mode))
        }
        _ => {
            eprintln!("find: unknown predicate '{arg}'");
            None
        }
    }
}

fn parse_usize_arg(flag: &str, val: &str) -> Option<usize> {
    match val.parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("find: invalid argument '{val}' for '{flag}'");
            None
        }
    }
}

fn parse_signed_arg(flag: &str, val: &str) -> Option<i64> {
    // Supports +N, -N, or N
    let s = val.trim();
    match s.parse::<i64>() {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("find: invalid argument '{val}' for '{flag}'");
            None
        }
    }
}

fn parse_perm_arg(val: &str) -> Option<u32> {
    match u32::from_str_radix(val, 8) {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("find: invalid permission mode '{val}'");
            None
        }
    }
}

fn parse_size_spec(val: &str) -> Option<SizeSpec> {
    let s = val.trim();
    if s.is_empty() {
        eprintln!("find: invalid size specification");
        return None;
    }

    let (cmp, rest) = if let Some(stripped) = s.strip_prefix('+') {
        (Cmp::GreaterThan, stripped)
    } else if let Some(stripped) = s.strip_prefix('-') {
        (Cmp::LessThan, stripped)
    } else {
        (Cmp::Exact, s)
    };

    // Parse number and optional suffix
    let (num_str, multiplier) = if let Some(n) = rest.strip_suffix('c') {
        (n, 1u64)
    } else if let Some(n) = rest.strip_suffix('k') {
        (n, 1024)
    } else if let Some(n) = rest.strip_suffix('M') {
        (n, 1024 * 1024)
    } else if let Some(n) = rest.strip_suffix('G') {
        (n, 1024 * 1024 * 1024)
    } else {
        // Default: 512-byte blocks
        (rest, 512)
    };

    match num_str.parse::<u64>() {
        Ok(n) => Some(SizeSpec {
            cmp,
            bytes: n * multiplier,
        }),
        Err(_) => {
            eprintln!("find: invalid size '{val}'");
            None
        }
    }
}
