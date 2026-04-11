const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: awk [OPTION]... 'program' [file]...
   or: awk [OPTION]... -f progfile [file]...

Pattern scanning and processing language.

  -F fs, --field-separator=fs
                         use fs for the input field separator
  -v var=val, --assign var=val
                         assign the variable var the value val
  -f progfile, --file=progfile
                         read the AWK program source from progfile
      --help             display this help and exit
      --version          output version information and exit

If no file is given, or file is -, read standard input.";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AwkConfig {
    pub program: String,
    pub field_separator: String,
    pub variables: Vec<(String, String)>,
    pub program_file: Option<String>,
    pub files: Vec<String>,
}

impl AwkConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = AwkConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;
        let mut has_program = false;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                if !has_program && config.program_file.is_none() {
                    config.program = arg.clone();
                    has_program = true;
                } else {
                    config.files.push(arg.clone());
                }
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
                println!("awk {VERSION}");
                return None;
            }

            // --field-separator=fs
            if let Some(fs) = arg.strip_prefix("--field-separator=") {
                config.field_separator = fs.to_string();
                i += 1;
                continue;
            }
            if arg == "--field-separator" {
                i += 1;
                if i < args.len() {
                    config.field_separator = args[i].clone();
                } else {
                    eprintln!("awk: option '--field-separator' requires an argument");
                }
                i += 1;
                continue;
            }

            // --assign var=val
            if let Some(assign) = arg.strip_prefix("--assign=") {
                if let Some((var, val)) = assign.split_once('=') {
                    config.variables.push((var.to_string(), val.to_string()));
                } else {
                    eprintln!("awk: invalid assignment: {assign}");
                }
                i += 1;
                continue;
            }
            if arg == "--assign" {
                i += 1;
                if i < args.len() {
                    if let Some((var, val)) = args[i].split_once('=') {
                        config.variables.push((var.to_string(), val.to_string()));
                    } else {
                        eprintln!("awk: invalid assignment: {}", args[i]);
                    }
                } else {
                    eprintln!("awk: option '--assign' requires an argument");
                }
                i += 1;
                continue;
            }

            // --file=progfile
            if let Some(file) = arg.strip_prefix("--file=") {
                config.program_file = Some(file.to_string());
                has_program = true;
                i += 1;
                continue;
            }
            if arg == "--file" {
                i += 1;
                if i < args.len() {
                    config.program_file = Some(args[i].clone());
                    has_program = true;
                } else {
                    eprintln!("awk: option '--file' requires an argument");
                }
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'F' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.field_separator = rest;
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.field_separator = args[i].clone();
                            } else {
                                eprintln!("awk: option requires an argument -- 'F'");
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    'v' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        let assignment = if !rest.is_empty() {
                            rest
                        } else {
                            i += 1;
                            if i < args.len() {
                                args[i].clone()
                            } else {
                                eprintln!("awk: option requires an argument -- 'v'");
                                j = chars.len();
                                continue;
                            }
                        };
                        if let Some((var, val)) = assignment.split_once('=') {
                            config.variables.push((var.to_string(), val.to_string()));
                        } else {
                            eprintln!("awk: invalid assignment: {assignment}");
                        }
                        j = chars.len();
                        continue;
                    }
                    'f' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.program_file = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.program_file = Some(args[i].clone());
                            } else {
                                eprintln!("awk: option requires an argument -- 'f'");
                                j = chars.len();
                                continue;
                            }
                        }
                        has_program = true;
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("awk: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        Some(config)
    }
}
