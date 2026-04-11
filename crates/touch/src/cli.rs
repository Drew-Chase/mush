const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: touch [OPTION]... FILE...
Update the access and modification times of each FILE to the current time.

A FILE argument that does not exist is created empty, unless -c is supplied.

  -a                     change only the access time
  -c, --no-create        do not create any files
  -d, --date=STRING      parse STRING and use it instead of current time
  -m                     change only the modification time
  -r, --reference=FILE   use this file's times instead of current time
      --help             display this help and exit
      --version          output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TouchConfig {
    pub access_only: bool,
    pub modify_only: bool,
    pub no_create: bool,
    pub reference: Option<String>,
    pub date: Option<String>,
    pub files: Vec<String>,
}

impl TouchConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = TouchConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.files.push(arg.clone());
                i += 1;
                continue;
            }

            if arg == "--" {
                parsing_flags = false;
                i += 1;
                continue;
            }

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("touch {VERSION}");
                return None;
            }
            if arg == "--no-create" {
                config.no_create = true;
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--reference=") {
                config.reference = Some(val.to_string());
                i += 1;
                continue;
            }
            if arg == "--reference" {
                i += 1;
                if i < args.len() {
                    config.reference = Some(args[i].clone());
                } else {
                    eprintln!("touch: option '--reference' requires an argument");
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--date=") {
                config.date = Some(val.to_string());
                i += 1;
                continue;
            }
            if arg == "--date" {
                i += 1;
                if i < args.len() {
                    config.date = Some(args[i].clone());
                } else {
                    eprintln!("touch: option '--date' requires an argument");
                }
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'a' => config.access_only = true,
                    'm' => config.modify_only = true,
                    'c' => config.no_create = true,
                    'r' => {
                        // -r FILE or -rFILE
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.reference = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.reference = Some(args[i].clone());
                            } else {
                                eprintln!("touch: option '-r' requires an argument");
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    'd' => {
                        // -d STRING or -dSTRING
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.date = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.date = Some(args[i].clone());
                            } else {
                                eprintln!("touch: option '-d' requires an argument");
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("touch: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        if config.files.is_empty() {
            eprintln!("touch: missing file operand");
            eprintln!("Try 'touch --help' for more information.");
            return Some(config);
        }

        Some(config)
    }
}
