const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: rm [OPTION]... [FILE]...
Remove (unlink) the FILE(s).

  -f, --force           ignore nonexistent files and arguments, never prompt
  -i                    prompt before every removal
  -I                    prompt once before removing more than three files, or
                          when removing recursively
      --interactive[=WHEN]  prompt according to WHEN: never, once (-I), or
                          always (-i); without WHEN, assume always
  -r, -R, --recursive   remove directories and their contents recursively
  -d, --dir             remove empty directories
  -v, --verbose         explain what is being done
      --no-preserve-root  do not treat '/' specially
      --preserve-root[=all]  do not remove '/' (default);
                          with 'all', reject any command line argument
                          on a separate device from its parent
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InteractiveMode {
    #[default]
    Never,
    Once,
    Always,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RmConfig {
    pub force: bool,
    pub interactive: InteractiveMode,
    pub recursive: bool,
    pub dir: bool,
    pub verbose: bool,
    pub preserve_root: bool,
    pub preserve_root_all: bool,
    pub paths: Vec<String>,
}

impl Default for RmConfig {
    fn default() -> Self {
        Self {
            force: false,
            interactive: InteractiveMode::Never,
            recursive: false,
            dir: false,
            verbose: false,
            preserve_root: true,
            preserve_root_all: false,
            paths: Vec::new(),
        }
    }
}

impl RmConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = RmConfig::default();
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
                println!("rm {VERSION}");
                return None;
            }
            if arg == "--force" {
                config.force = true;
                config.interactive = InteractiveMode::Never;
                i += 1;
                continue;
            }
            if arg == "--recursive" {
                config.recursive = true;
                i += 1;
                continue;
            }
            if arg == "--dir" {
                config.dir = true;
                i += 1;
                continue;
            }
            if arg == "--verbose" {
                config.verbose = true;
                i += 1;
                continue;
            }
            if arg == "--no-preserve-root" {
                config.preserve_root = false;
                i += 1;
                continue;
            }
            if arg == "--preserve-root" {
                config.preserve_root = true;
                i += 1;
                continue;
            }
            if arg == "--preserve-root=all" {
                config.preserve_root = true;
                config.preserve_root_all = true;
                i += 1;
                continue;
            }
            if arg == "--interactive" {
                config.interactive = InteractiveMode::Always;
                config.force = false;
                i += 1;
                continue;
            }
            if let Some(when) = arg.strip_prefix("--interactive=") {
                match when {
                    "never" => {
                        config.interactive = InteractiveMode::Never;
                    }
                    "once" => {
                        config.interactive = InteractiveMode::Once;
                        config.force = false;
                    }
                    "always" => {
                        config.interactive = InteractiveMode::Always;
                        config.force = false;
                    }
                    _ => {
                        eprintln!("rm: invalid argument '{when}' for '--interactive'");
                        eprintln!("Valid arguments are: 'never', 'once', 'always'");
                        return None;
                    }
                }
                i += 1;
                continue;
            }

            let chars: Vec<char> = arg[1..].chars().collect();
            let mut valid = true;
            for &c in &chars {
                match c {
                    'f' | 'r' | 'R' | 'i' | 'I' | 'd' | 'v' => {}
                    _ => {
                        eprintln!("rm: invalid option -- '{c}'");
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

            for &c in &chars {
                match c {
                    'f' => {
                        config.force = true;
                        config.interactive = InteractiveMode::Never;
                    }
                    'r' | 'R' => config.recursive = true,
                    'i' => {
                        config.interactive = InteractiveMode::Always;
                        config.force = false;
                    }
                    'I' => {
                        config.interactive = InteractiveMode::Once;
                        config.force = false;
                    }
                    'd' => config.dir = true,
                    'v' => config.verbose = true,
                    _ => unreachable!(),
                }
            }
            i += 1;
        }

        Some(config)
    }
}
