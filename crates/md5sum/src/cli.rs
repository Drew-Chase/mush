const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: md5sum [OPTION]... [FILE]...

Print or check MD5 (128-bit) checksums.
With no FILE, or when FILE is -, read standard input.

  -b, --binary   read in binary mode
  -c, --check    read checksums from the FILEs and check them
      --tag      create a BSD-style checksum
  -q, --quiet    don't print OK for each successfully verified file
      --status   don't output anything, status code shows success
  -w, --warn     warn about improperly formatted checksum lines
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Md5sumConfig {
    pub binary: bool,
    pub check: bool,
    pub tag: bool,
    pub quiet: bool,
    pub status: bool,
    pub warn: bool,
    pub files: Vec<String>,
}

impl Md5sumConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = Md5sumConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("md5sum {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--binary" => config.binary = true,
                "--check" => config.check = true,
                "--tag" => config.tag = true,
                "--quiet" => config.quiet = true,
                "--status" => config.status = true,
                "--warn" => config.warn = true,
                _ if arg.starts_with('-') && arg != "-" => {
                    for c in arg[1..].chars() {
                        match c {
                            'b' => config.binary = true,
                            'c' => config.check = true,
                            'q' => config.quiet = true,
                            'w' => config.warn = true,
                            _ => {
                                eprintln!("md5sum: invalid option -- '{c}'");
                                return None;
                            }
                        }
                    }
                }
                _ => break,
            }

            i += 1;
        }

        config.files = args[i..].to_vec();

        Some(config)
    }
}
