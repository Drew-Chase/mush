const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: nproc [OPTION]...
Print the number of processing units available to the current process,
which may be less than the number of online processors.

      --all        print the number of installed processors
      --ignore=N   if possible, exclude N processing units
      --help       display this help and exit
      --version    output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NprocConfig {
    pub all: bool,
    pub ignore: usize,
}

impl NprocConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = NprocConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("nproc {VERSION}");
                return None;
            }
            if arg == "--all" {
                config.all = true;
                i += 1;
                continue;
            }
            if let Some(val) = arg.strip_prefix("--ignore=") {
                if let Ok(n) = val.parse::<usize>() {
                    config.ignore = n;
                } else {
                    eprintln!("nproc: invalid number: '{val}'");
                }
                i += 1;
                continue;
            }
            if arg == "--ignore" {
                i += 1;
                if i < args.len() {
                    if let Ok(n) = args[i].parse::<usize>() {
                        config.ignore = n;
                    } else {
                        eprintln!("nproc: invalid number: '{}'", args[i]);
                    }
                }
                i += 1;
                continue;
            }

            eprintln!("nproc: unrecognized option '{arg}'");
            i += 1;
        }

        Some(config)
    }
}
