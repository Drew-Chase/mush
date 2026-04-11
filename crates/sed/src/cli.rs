const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: sed [OPTION]... {script-only-if-no-other-script} [input-file]...

  -n, --quiet, --silent    suppress automatic printing of pattern space
  -e script, --expression=script
                           add the script to the commands to be executed
  -f script-file, --file=script-file
                           add the contents of script-file to the commands
  -i[SUFFIX], --in-place[=SUFFIX]
                           edit files in place (makes backup if SUFFIX supplied)
  -r, -E, --regexp-extended
                           use extended regular expressions in the script
  -s, --separate           consider files as separate rather than as a single
                           continuous long stream
      --help               display this help and exit
      --version            output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SedConfig {
    pub scripts: Vec<String>,
    pub script_files: Vec<String>,
    pub in_place: Option<Option<String>>,
    pub quiet: bool,
    pub extended_regexp: bool,
    pub separate: bool,
    pub files: Vec<String>,
}

impl SedConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = SedConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;
        let mut has_explicit_script = false;

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
                println!("sed {VERSION}");
                return None;
            }
            if arg == "--quiet" || arg == "--silent" {
                config.quiet = true;
                i += 1;
                continue;
            }
            if arg == "--regexp-extended" {
                config.extended_regexp = true;
                i += 1;
                continue;
            }
            if arg == "--separate" {
                config.separate = true;
                i += 1;
                continue;
            }

            // --expression=script
            if let Some(script) = arg.strip_prefix("--expression=") {
                config.scripts.push(script.to_string());
                has_explicit_script = true;
                i += 1;
                continue;
            }
            if arg == "--expression" {
                i += 1;
                if i < args.len() {
                    config.scripts.push(args[i].clone());
                    has_explicit_script = true;
                } else {
                    eprintln!("sed: option '--expression' requires an argument");
                }
                i += 1;
                continue;
            }

            // --file=script-file
            if let Some(file) = arg.strip_prefix("--file=") {
                config.script_files.push(file.to_string());
                has_explicit_script = true;
                i += 1;
                continue;
            }
            if arg == "--file" {
                i += 1;
                if i < args.len() {
                    config.script_files.push(args[i].clone());
                    has_explicit_script = true;
                } else {
                    eprintln!("sed: option '--file' requires an argument");
                }
                i += 1;
                continue;
            }

            // --in-place[=SUFFIX]
            if arg == "--in-place" {
                config.in_place = Some(None);
                i += 1;
                continue;
            }
            if let Some(suffix) = arg.strip_prefix("--in-place=") {
                config.in_place = Some(Some(suffix.to_string()));
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'n' => config.quiet = true,
                    'r' | 'E' => config.extended_regexp = true,
                    's' => config.separate = true,
                    'e' => {
                        // Rest of this arg or next arg is the script
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.scripts.push(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.scripts.push(args[i].clone());
                            } else {
                                eprintln!("sed: option requires an argument -- 'e'");
                            }
                        }
                        has_explicit_script = true;
                        j = chars.len(); // consumed rest
                        continue;
                    }
                    'f' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.script_files.push(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.script_files.push(args[i].clone());
                            } else {
                                eprintln!("sed: option requires an argument -- 'f'");
                            }
                        }
                        has_explicit_script = true;
                        j = chars.len();
                        continue;
                    }
                    'i' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.in_place = Some(Some(rest));
                        } else {
                            config.in_place = Some(None);
                        }
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("sed: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        // If no -e/-f, first positional arg is the script
        if !has_explicit_script
            && config.script_files.is_empty()
            && let Some(script) = config.files.first().cloned()
        {
            config.scripts.push(script);
            config.files.remove(0);
        }

        Some(config)
    }
}
