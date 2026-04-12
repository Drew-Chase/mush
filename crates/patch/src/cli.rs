const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: patch [OPTIONS] [ORIGINAL [PATCHFILE]]
Apply a unified diff to files.

  -p NUM         strip NUM leading path components
  -R             reverse the patch
  --dry-run      do not actually modify any files
  -b             create backup files (.orig)
  -i FILE        read patch from FILE
  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PatchConfig {
    pub strip: usize,
    pub reverse: bool,
    pub dry_run: bool,
    pub backup: bool,
    pub patch_file: Option<String>,
    pub original_file: Option<String>,
}

impl PatchConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = PatchConfig::default();
        let mut i = 0;
        let mut positional = Vec::new();

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" || arg == "-h" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" || arg == "-V" {
                println!("patch {VERSION}");
                return None;
            }

            if arg == "-p" {
                i += 1;
                config.strip = args.get(i)?.parse().ok()?;
            } else if let Some(rest) = arg.strip_prefix("-p") {
                config.strip = rest.parse().ok()?;
            } else if arg == "-R" {
                config.reverse = true;
            } else if arg == "--dry-run" {
                config.dry_run = true;
            } else if arg == "-b" {
                config.backup = true;
            } else if arg == "-i" {
                i += 1;
                config.patch_file = Some(args.get(i)?.clone());
            } else if arg.starts_with('-') && arg != "-" {
                eprintln!("patch: invalid option '{arg}'");
            } else {
                positional.push(arg.clone());
            }
            i += 1;
        }

        if let Some(orig) = positional.first() {
            config.original_file = Some(orig.clone());
        }
        if positional.len() > 1 {
            config.patch_file = Some(positional[1].clone());
        }

        Some(config)
    }
}
