const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: basename NAME [SUFFIX]
  or:  basename OPTION... NAME...
Print NAME with any leading directory components removed.
If specified, also remove a trailing SUFFIX.

  -a, --multiple       support multiple arguments and treat each as a NAME
  -s, --suffix=SUFFIX  remove a trailing SUFFIX; implies -a
  -z, --zero           end each output line with NUL, not newline
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BasenameConfig {
    pub multiple: bool,
    pub suffix: Option<String>,
    pub zero: bool,
    pub names: Vec<String>,
}

impl BasenameConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = BasenameConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.names.push(arg.clone());
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
                println!("basename {VERSION}");
                return None;
            }
            if arg == "--multiple" {
                config.multiple = true;
                i += 1;
                continue;
            }
            if arg == "--zero" {
                config.zero = true;
                i += 1;
                continue;
            }
            if let Some(suffix_str) = arg.strip_prefix("--suffix=") {
                config.suffix = Some(suffix_str.to_string());
                config.multiple = true;
                i += 1;
                continue;
            }
            if arg == "--suffix" {
                i += 1;
                if i < args.len() {
                    config.suffix = Some(args[i].clone());
                    config.multiple = true;
                }
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'a' => config.multiple = true,
                    'z' => config.zero = true,
                    's' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.suffix = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.suffix = Some(args[i].clone());
                            }
                        }
                        config.multiple = true;
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("basename: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        // When not in multiple mode, second positional is treated as suffix
        if !config.multiple && config.names.len() == 2 {
            config.suffix = Some(config.names.remove(1));
        }

        Some(config)
    }
}
