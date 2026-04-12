const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: mktemp [OPTION]... [TEMPLATE]
Create a temporary file or directory, safely, and print its name.
TEMPLATE must contain at least 3 consecutive 'X's in last component.
If TEMPLATE is not specified, use tmp.XXXXXXXXXX.

  -d, --directory      create a directory, not a file
  -u, --dry-run        do not create anything; merely print a name
  -q, --quiet          suppress diagnostics about file/dir-creation failure
  -p, --tmpdir DIR     interpret TEMPLATE relative to DIR
      --suffix SUFFIX  append SUFFIX to TEMPLATE
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MktempConfig {
    pub directory: bool,
    pub dry_run: bool,
    pub quiet: bool,
    pub tmpdir: Option<String>,
    pub suffix: Option<String>,
    pub template: Option<String>,
}

impl MktempConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = MktempConfig::default();
        let mut i = 0;
        let mut positional: Vec<String> = Vec::new();

        while i < args.len() {
            let arg = &args[i];

            if !arg.starts_with('-') || arg == "-" {
                positional.push(arg.clone());
                i += 1;
                continue;
            }

            if arg == "--" {
                i += 1;
                while i < args.len() {
                    positional.push(args[i].clone());
                    i += 1;
                }
                break;
            }

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("mktemp {VERSION}");
                return None;
            }
            if arg == "--directory" {
                config.directory = true;
                i += 1;
                continue;
            }
            if arg == "--dry-run" {
                config.dry_run = true;
                i += 1;
                continue;
            }
            if arg == "--quiet" {
                config.quiet = true;
                i += 1;
                continue;
            }
            if arg == "--tmpdir" || arg == "-p" {
                i += 1;
                if i < args.len() {
                    config.tmpdir = Some(args[i].clone());
                } else {
                    eprintln!("mktemp: option '{arg}' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--tmpdir=") {
                config.tmpdir = Some(val.to_string());
                i += 1;
                continue;
            }
            if arg == "--suffix" {
                i += 1;
                if i < args.len() {
                    config.suffix = Some(args[i].clone());
                } else {
                    eprintln!("mktemp: option '--suffix' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--suffix=") {
                config.suffix = Some(val.to_string());
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'd' => config.directory = true,
                    'u' => config.dry_run = true,
                    'q' => config.quiet = true,
                    'p' => {
                        // Rest of chars or next arg is the directory
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.tmpdir = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.tmpdir = Some(args[i].clone());
                            } else {
                                eprintln!("mktemp: option '-p' requires an argument");
                                return None;
                            }
                        }
                        j = chars.len(); // consumed
                        continue;
                    }
                    _ => {
                        eprintln!("mktemp: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        config.template = positional.into_iter().next();
        Some(config)
    }
}
