const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: uptime [OPTIONS]

Show how long the system has been running.

  -p, --pretty   show uptime in pretty format
  -s, --since    system up since, in yyyy-mm-dd HH:MM:SS format
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UptimeConfig {
    pub pretty: bool,
    pub since: bool,
}

impl UptimeConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = UptimeConfig::default();

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("uptime {VERSION}");
                    return None;
                }
                "-p" | "--pretty" => config.pretty = true,
                "-s" | "--since" => config.since = true,
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("uptime: unknown option '{arg}'");
                    }
                }
            }
        }

        Some(config)
    }
}
