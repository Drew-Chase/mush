const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: su [OPTIONS] [USER]

Switch to another user (default: root).

  -c, --command CMD  pass CMD to the invoked shell
  -l, --login        make the shell a login shell
  -s, --shell SHELL  run SHELL instead of the default
      --help         display this help and exit
      --version      output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct SuConfig {
    pub command: Option<String>,
    pub login: bool,
    pub shell: Option<String>,
    pub user: String,
}

impl Default for SuConfig {
    fn default() -> Self {
        Self {
            command: None,
            login: false,
            shell: None,
            user: "root".to_string(),
        }
    }
}

impl SuConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = SuConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("su {VERSION}");
                    return None;
                }
                "-c" | "--command" => {
                    i += 1;
                    if i < args.len() {
                        config.command = Some(args[i].clone());
                    } else {
                        eprintln!("su: option '{arg}' requires an argument");
                    }
                }
                "-l" | "--login" | "-" => config.login = true,
                "-s" | "--shell" => {
                    i += 1;
                    if i < args.len() {
                        config.shell = Some(args[i].clone());
                    } else {
                        eprintln!("su: option '{arg}' requires an argument");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("su: unknown option '{arg}'");
                    } else {
                        config.user = arg.clone();
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
