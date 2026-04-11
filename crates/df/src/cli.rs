const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: df [OPTIONS] [FILE]...
Report filesystem disk space usage.

  -h, --human-readable  print sizes in human-readable format (powers of 1024)
  -H, --si              print sizes in human-readable format (powers of 1000)
  -T, --print-type      print file system type
  -t, --type TYPE       limit listing to file systems of TYPE
  -a, --all             include pseudo, duplicate, inaccessible file systems
      --total            produce a grand total
      --help             display this help and exit
      --version          output version information and exit";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DfConfig {
    pub human_readable: bool,
    pub si: bool,
    pub print_type: bool,
    pub type_filter: Option<String>,
    pub all: bool,
    pub total: bool,
    pub files: Vec<String>,
}

impl DfConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = DfConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("df {VERSION}");
                    return None;
                }
                "-h" | "--human-readable" => config.human_readable = true,
                "-H" | "--si" => config.si = true,
                "-T" | "--print-type" => config.print_type = true,
                "-a" | "--all" => config.all = true,
                "--total" => config.total = true,
                "-t" | "--type" => {
                    i += 1;
                    if i < args.len() {
                        config.type_filter = Some(args[i].clone());
                    } else {
                        eprintln!("df: option '{arg}' requires an argument");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("df: unknown option '{arg}'");
                    } else {
                        config.files.push(arg.clone());
                    }
                }
            }
            i += 1;
        }

        Some(config)
    }
}
