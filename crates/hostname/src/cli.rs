const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: hostname [OPTION]...
Print the system's hostname.

  -s, --short   print the short hostname (up to the first dot)
  -f, --fqdn, --long
                 print the fully qualified domain name
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostnameConfig {
    pub short: bool,
    pub fqdn: bool,
}

impl HostnameConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = HostnameConfig::default();

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("hostname {VERSION}");
                    return None;
                }
                "--short" => config.short = true,
                "--fqdn" | "--long" => config.fqdn = true,
                s if s.starts_with('-') && !s.starts_with("--") => {
                    for c in s[1..].chars() {
                        match c {
                            's' => config.short = true,
                            'f' => config.fqdn = true,
                            _ => {
                                eprintln!("hostname: invalid option -- '{c}'");
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("hostname: unexpected argument '{arg}'");
                    return None;
                }
            }
        }

        Some(config)
    }
}
