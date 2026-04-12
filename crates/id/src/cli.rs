const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: id [OPTIONS] [USER]

Print user and group information for the specified USER, or current user.

  -u, --user     print only the effective user ID
  -g, --group    print only the effective group ID
  -G, --groups   print all group IDs
  -n, --name     print a name instead of a number, for -ugG
  -r, --real     print the real ID instead of the effective ID, for -ugG
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IdConfig {
    pub user_only: bool,
    pub group_only: bool,
    pub groups_only: bool,
    pub name: bool,
    pub real: bool,
    pub target_user: Option<String>,
}

impl IdConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = IdConfig::default();

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("id {VERSION}");
                    return None;
                }
                "-u" | "--user" => config.user_only = true,
                "-g" | "--group" => config.group_only = true,
                "-G" | "--groups" => config.groups_only = true,
                "-n" | "--name" => config.name = true,
                "-r" | "--real" => config.real = true,
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("id: unknown option '{arg}'");
                    } else {
                        config.target_user = Some(arg.clone());
                    }
                }
            }
        }

        Some(config)
    }
}
