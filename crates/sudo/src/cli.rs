const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: sudo [OPTIONS] COMMAND [ARGS]...
Execute a command as another user.

  -u, --user USER      run command as USER (default: root)
  -i, --login          run login shell as the target user
  -s, --shell          run shell as the target user
  -E, --preserve-env   preserve user environment when running command
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SudoConfig {
    pub user: Option<String>,
    pub login: bool,
    pub shell: bool,
    pub preserve_env: bool,
    pub command: Vec<String>,
}

impl SudoConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = SudoConfig::default();
        let mut i = 0;
        let mut found_command = false;

        while i < args.len() {
            if found_command {
                config.command.push(args[i].clone());
                i += 1;
                continue;
            }

            let arg = &args[i];

            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("sudo {VERSION}");
                    return None;
                }
                "-u" | "--user" => {
                    i += 1;
                    if i < args.len() {
                        config.user = Some(args[i].clone());
                    } else {
                        eprintln!("sudo: option '{arg}' requires an argument");
                    }
                }
                "-i" | "--login" => config.login = true,
                "-s" | "--shell" => config.shell = true,
                "-E" | "--preserve-env" => config.preserve_env = true,
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("sudo: unknown option '{arg}'");
                    } else {
                        config.command.push(arg.clone());
                        found_command = true;
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
