const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: file [OPTION]... FILE...
Determine type of FILEs.

  -b, --brief          do not prepend filenames to output lines
  -i, --mime           output MIME type strings
      --mime-type       output the MIME type only
  -L, --dereference    follow symlinks
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileConfig {
    pub brief: bool,
    pub mime: bool,
    pub mime_type: bool,
    pub dereference: bool,
    pub files: Vec<String>,
}

impl FileConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = FileConfig::default();
        let mut i = 0;
        let mut parsing_flags = true;

        while i < args.len() {
            let arg = &args[i];

            if !parsing_flags || !arg.starts_with('-') || arg == "-" {
                config.files.push(arg.clone());
                i += 1;
                continue;
            }

            if arg == "--" {
                parsing_flags = false;
                i += 1;
                continue;
            }

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("file {VERSION}");
                return None;
            }
            if arg == "--brief" {
                config.brief = true;
                i += 1;
                continue;
            }
            if arg == "--mime" {
                config.mime = true;
                i += 1;
                continue;
            }
            if arg == "--mime-type" {
                config.mime_type = true;
                i += 1;
                continue;
            }
            if arg == "--dereference" {
                config.dereference = true;
                i += 1;
                continue;
            }

            // Short flags
            let chars: Vec<char> = arg[1..].chars().collect();
            for &c in &chars {
                match c {
                    'b' => config.brief = true,
                    'i' => config.mime = true,
                    'L' => config.dereference = true,
                    _ => {
                        eprintln!("file: invalid option -- '{c}'");
                    }
                }
            }
            i += 1;
        }

        if config.files.is_empty() {
            eprintln!("file: missing operand");
            return None;
        }

        Some(config)
    }
}
