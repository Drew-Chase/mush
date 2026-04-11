const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: ps [OPTIONS]
Report running processes.

  -e, -A             select all processes
  -f, --full         full format listing
  -l, --long         long format
  -u, --user USER    show only processes for USER
  -p, --pid PID      show only specified PIDs (comma-separated)
  -C, --command NAME show only processes matching NAME
      --sort KEY     sort by key: pid, ppid, cpu, mem, time, name
  -o, --format FMT   custom output format (comma-separated: pid,ppid,name,cpu,mem,time,cmd,user)
      --no-headers   suppress column headers
  -a                 show processes for all terminals
  -x                 show processes without controlling terminals
      --help         display this help and exit
      --version      output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PsConfig {
    pub all: bool,
    pub full: bool,
    pub long_format: bool,
    pub user_filter: Option<String>,
    pub pid_filter: Vec<u32>,
    pub command_filter: Option<String>,
    pub sort_key: Option<String>,
    pub format_spec: Option<String>,
    pub no_headers: bool,
    pub show_threads: bool,
}

impl PsConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = PsConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("ps {VERSION}");
                    return None;
                }
                "-e" | "-A" => config.all = true,
                "-f" | "--full" => config.full = true,
                "-l" | "--long" => config.long_format = true,
                "--no-headers" => config.no_headers = true,
                "-a" => config.show_threads = true,
                "-x" => config.show_threads = true,
                "-u" | "--user" => {
                    i += 1;
                    if i < args.len() {
                        config.user_filter = Some(args[i].clone());
                    } else {
                        eprintln!("ps: option '{arg}' requires an argument");
                    }
                }
                "-p" | "--pid" => {
                    i += 1;
                    if i < args.len() {
                        for part in args[i].split(',') {
                            let trimmed = part.trim();
                            if let Ok(pid) = trimmed.parse::<u32>() {
                                config.pid_filter.push(pid);
                            } else {
                                eprintln!("ps: invalid PID '{trimmed}'");
                            }
                        }
                    } else {
                        eprintln!("ps: option '{arg}' requires an argument");
                    }
                }
                "-C" | "--command" => {
                    i += 1;
                    if i < args.len() {
                        config.command_filter = Some(args[i].clone());
                    } else {
                        eprintln!("ps: option '{arg}' requires an argument");
                    }
                }
                "--sort" => {
                    i += 1;
                    if i < args.len() {
                        config.sort_key = Some(args[i].clone());
                    } else {
                        eprintln!("ps: option '--sort' requires an argument");
                    }
                }
                "-o" | "--format" => {
                    i += 1;
                    if i < args.len() {
                        config.format_spec = Some(args[i].clone());
                    } else {
                        eprintln!("ps: option '{arg}' requires an argument");
                    }
                }
                _ => {
                    // Try combined short flags like -ef, -aux
                    if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") {
                        for ch in arg[1..].chars() {
                            match ch {
                                'e' | 'A' => config.all = true,
                                'f' => config.full = true,
                                'l' => config.long_format = true,
                                'a' => config.show_threads = true,
                                'x' => config.show_threads = true,
                                _ => {
                                    eprintln!("ps: unknown option '-{ch}'");
                                }
                            }
                        }
                    } else {
                        eprintln!("ps: unexpected argument '{arg}'");
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
