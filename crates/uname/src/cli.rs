const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: uname [OPTION]...
Print certain system information. With no OPTION, same as -s.

  -a, --all                print all information
  -s, --kernel-name        print the kernel name
  -n, --nodename           print the network node hostname
  -r, --kernel-release     print the kernel release
  -v, --kernel-version     print the kernel version
  -m, --machine            print the machine hardware name
  -p, --processor          print the processor type
  -o, --operating-system   print the operating system
      --help               display this help and exit
      --version            output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnameConfig {
    pub all: bool,
    pub kernel_name: bool,
    pub nodename: bool,
    pub kernel_release: bool,
    pub kernel_version: bool,
    pub machine: bool,
    pub processor: bool,
    pub operating_system: bool,
}

impl UnameConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = UnameConfig::default();
        let mut any_flag = false;

        for arg in args {
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("uname {VERSION}");
                    return None;
                }
                "--all" => {
                    config.all = true;
                    any_flag = true;
                }
                "--kernel-name" => {
                    config.kernel_name = true;
                    any_flag = true;
                }
                "--nodename" => {
                    config.nodename = true;
                    any_flag = true;
                }
                "--kernel-release" => {
                    config.kernel_release = true;
                    any_flag = true;
                }
                "--kernel-version" => {
                    config.kernel_version = true;
                    any_flag = true;
                }
                "--machine" => {
                    config.machine = true;
                    any_flag = true;
                }
                "--processor" => {
                    config.processor = true;
                    any_flag = true;
                }
                "--operating-system" => {
                    config.operating_system = true;
                    any_flag = true;
                }
                s if s.starts_with('-') && !s.starts_with("--") => {
                    for c in s[1..].chars() {
                        match c {
                            'a' => config.all = true,
                            's' => config.kernel_name = true,
                            'n' => config.nodename = true,
                            'r' => config.kernel_release = true,
                            'v' => config.kernel_version = true,
                            'm' => config.machine = true,
                            'p' => config.processor = true,
                            'o' => config.operating_system = true,
                            _ => {
                                eprintln!("uname: invalid option -- '{c}'");
                                return None;
                            }
                        }
                    }
                    any_flag = true;
                }
                _ => {
                    eprintln!("uname: extra operand '{arg}'");
                    return None;
                }
            }
        }

        if !any_flag {
            config.kernel_name = true;
        }

        if config.all {
            config.kernel_name = true;
            config.nodename = true;
            config.kernel_release = true;
            config.kernel_version = true;
            config.machine = true;
            config.processor = true;
            config.operating_system = true;
        }

        Some(config)
    }
}
