const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: cp [OPTION]... [-T] SOURCE DEST
  or:  cp [OPTION]... SOURCE... DIRECTORY
  or:  cp [OPTION]... -t DIRECTORY SOURCE...
Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.

  -f, --force                do not prompt before overwriting
  -i, --interactive          prompt before overwrite
  -n, --no-clobber           do not overwrite an existing file
  -r, -R, --recursive        copy directories recursively
  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY
  -T, --no-target-directory  treat DEST as a normal file
  -u, --update               copy only when the SOURCE file is newer
                               than the destination file or when the
                               destination file is missing
  -v, --verbose              explain what is being done
      --help                 display this help and exit
      --version              output version information and exit

If you specify more than one of -i, -f, -n, only the final one takes effect.";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverwriteMode {
    #[default]
    Force,
    Interactive,
    NoClobber,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CpConfig {
    pub overwrite: OverwriteMode,
    pub recursive: bool,
    pub update: bool,
    pub verbose: bool,
    pub target_directory: Option<String>,
    pub no_target_directory: bool,
    pub paths: Vec<String>,
}

impl CpConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = CpConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.paths.push(arg.clone());
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
                println!("cp {VERSION}");
                return None;
            }
            if arg == "--force" {
                config.overwrite = OverwriteMode::Force;
                i += 1;
                continue;
            }
            if arg == "--interactive" {
                config.overwrite = OverwriteMode::Interactive;
                i += 1;
                continue;
            }
            if arg == "--no-clobber" {
                config.overwrite = OverwriteMode::NoClobber;
                i += 1;
                continue;
            }
            if arg == "--recursive" {
                config.recursive = true;
                i += 1;
                continue;
            }
            if arg == "--update" {
                config.update = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if arg == "--no-target-directory" {
                config.no_target_directory = true;
                i += 1;
                continue;
            }
            if let Some(dir) = arg.strip_prefix("--target-directory=") {
                config.target_directory = Some(dir.to_string());
                i += 1;
                continue;
            }
            if arg == "--target-directory" {
                i += 1;
                if i < args.len() {
                    config.target_directory = Some(args[i].clone());
                }
                i += 1;
                continue;
            }

            let chars: Vec<char> = arg[1..].chars().collect();
            let mut valid = true;
            for &c in &chars {
                match c {
                    'f' | 'i' | 'n' | 'r' | 'R' | 'u' | 'v' | 'T' => {}
                    't' => break,
                    _ => {
                        eprintln!("cp: invalid option -- '{c}'");
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                config.paths.push(arg.clone());
                i += 1;
                continue;
            }

            let mut j = 0;
            while j < chars.len() {
                match chars[j] {
                    'f' => config.overwrite = OverwriteMode::Force,
                    'i' => config.overwrite = OverwriteMode::Interactive,
                    'n' => config.overwrite = OverwriteMode::NoClobber,
                    'r' | 'R' => config.recursive = true,
                    'u' => config.update = true,
                    'v' => config.verbose = true,
                    'T' => config.no_target_directory = true,
                    't' => {
                        let rest: String = chars[j + 1..].iter().collect();
                        if !rest.is_empty() {
                            config.target_directory = Some(rest);
                        } else {
                            i += 1;
                            if i < args.len() {
                                config.target_directory = Some(args[i].clone());
                            }
                        }
                        j = chars.len();
                        continue;
                    }
                    _ => unreachable!(),
                }
                j += 1;
            }
            i += 1;
        }

        Some(config)
    }
}
