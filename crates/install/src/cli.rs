const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: install [OPTION]... SOURCE... DEST
  or:  install [OPTION]... -t DIRECTORY SOURCE...
  or:  install -d [OPTION]... DIRECTORY...
Copy files and set attributes.

  -d                   create all leading directories, treat DEST as directory
  -m, --mode MODE      set permission mode (as in chmod)
  -v, --verbose        print the name of each file as it is installed
  -C, --compare        compare each pair of files; skip copy if identical
  -D                   create all leading components of DEST
  -t, --target-directory DIR  copy all SOURCE args into DIR
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallConfig {
    pub directory_mode: bool,
    pub mode: Option<String>,
    pub verbose: bool,
    pub compare: bool,
    pub create_leading: bool,
    pub target_dir: Option<String>,
    pub files: Vec<String>,
}

impl InstallConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = InstallConfig::default();
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
                println!("install {VERSION}");
                return None;
            }
            if arg == "--mode" {
                i += 1;
                if i < args.len() {
                    config.mode = Some(args[i].clone());
                } else {
                    eprintln!("install: option '--mode' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--mode=") {
                config.mode = Some(val.to_string());
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if arg == "--compare" {
                config.compare = true;
                i += 1;
                continue;
            }
            if arg == "--target-directory" || arg == "-t" {
                i += 1;
                if i < args.len() {
                    config.target_dir = Some(args[i].clone());
                } else {
                    eprintln!("install: option '{arg}' requires an argument");
                    return None;
                }
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--target-directory=") {
                config.target_dir = Some(val.to_string());
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'd' => config.directory_mode = true,
                    'v' => config.verbose = true,
                    'C' => config.compare = true,
                    'D' => config.create_leading = true,
                    'm' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.mode = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.mode = Some(args[i].clone());
                            } else {
                                eprintln!("install: option '-m' requires an argument");
                                return None;
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    't' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.target_dir = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.target_dir = Some(args[i].clone());
                            } else {
                                eprintln!("install: option '-t' requires an argument");
                                return None;
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    _ => {
                        eprintln!("install: invalid option -- '{}'", chars[j]);
                    }
                }
                j += 1;
            }
            i += 1;
        }

        config.files = positional;
        Some(config)
    }
}
