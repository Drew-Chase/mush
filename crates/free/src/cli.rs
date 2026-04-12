const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: free [OPTIONS]

Display amount of free and used memory in the system.

  -b             show output in bytes
  -k             show output in kibibytes (default)
  -m             show output in mebibytes
  -g             show output in gibibytes
  -h, --human    show human-readable output
      --si       use powers of 1000, not 1024
  -t, --total    show total for RAM + swap
  -w, --wide     wide output (separate buffers and cache)
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct FreeConfig {
    pub bytes: bool,
    pub kibi: bool,
    pub mebi: bool,
    pub gibi: bool,
    pub human: bool,
    pub si: bool,
    pub total: bool,
    pub wide: bool,
}

impl Default for FreeConfig {
    fn default() -> Self {
        Self {
            bytes: false,
            kibi: true,
            mebi: false,
            gibi: false,
            human: false,
            si: false,
            total: false,
            wide: false,
        }
    }
}

impl FreeConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = FreeConfig::default();

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("free {VERSION}");
                    return None;
                }
                "-b" => {
                    config.bytes = true;
                    config.kibi = false;
                    config.mebi = false;
                    config.gibi = false;
                    config.human = false;
                }
                "-k" => {
                    config.bytes = false;
                    config.kibi = true;
                    config.mebi = false;
                    config.gibi = false;
                    config.human = false;
                }
                "-m" => {
                    config.bytes = false;
                    config.kibi = false;
                    config.mebi = true;
                    config.gibi = false;
                    config.human = false;
                }
                "-g" => {
                    config.bytes = false;
                    config.kibi = false;
                    config.mebi = false;
                    config.gibi = true;
                    config.human = false;
                }
                "-h" | "--human" => {
                    config.bytes = false;
                    config.kibi = false;
                    config.mebi = false;
                    config.gibi = false;
                    config.human = true;
                }
                "--si" => config.si = true,
                "-t" | "--total" => config.total = true,
                "-w" | "--wide" => config.wide = true,
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("free: unknown option '{arg}'");
                    }
                }
            }
        }

        Some(config)
    }
}
