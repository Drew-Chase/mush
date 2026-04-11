const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: sha256sum [OPTION]... [FILE]...

Print or check SHA-256 (256-bit) checksums.
With no FILE, or when FILE is -, read standard input.

  -a, --algorithm ALG  hash algorithm (default: sha256)
  -b, --binary         read in binary mode
  -c, --check          read checksums from the FILEs and check them
      --tag            create a BSD-style checksum
  -q, --quiet          don't print OK for each successfully verified file
      --status         don't output anything, status code shows success
  -w, --warn           warn about improperly formatted checksum lines
      --help           display this help and exit
      --version        output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sha256sumConfig {
    pub algorithm: String,
    pub binary: bool,
    pub check: bool,
    pub tag: bool,
    pub quiet: bool,
    pub status: bool,
    pub warn: bool,
    pub files: Vec<String>,
}

impl Default for Sha256sumConfig {
    fn default() -> Self {
        Self {
            algorithm: "sha256".to_string(),
            binary: false,
            check: false,
            tag: false,
            quiet: false,
            status: false,
            warn: false,
            files: Vec::new(),
        }
    }
}

impl Sha256sumConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = Sha256sumConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("sha256sum {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--algorithm" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("sha256sum: option '--algorithm' requires an argument");
                        return None;
                    }
                    config.algorithm = args[i].clone();
                }
                "--binary" => config.binary = true,
                "--check" => config.check = true,
                "--tag" => config.tag = true,
                "--quiet" => config.quiet = true,
                "--status" => config.status = true,
                "--warn" => config.warn = true,
                _ if arg.starts_with('-') && arg != "-" => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'a' => {
                                // Next chars or next arg is the algorithm
                                if j + 1 < chars.len() {
                                    config.algorithm = chars[j + 1..].iter().collect();
                                    j = chars.len(); // consumed rest
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("sha256sum: option '-a' requires an argument");
                                        return None;
                                    }
                                    config.algorithm = args[i].clone();
                                }
                            }
                            'b' => config.binary = true,
                            'c' => config.check = true,
                            'q' => config.quiet = true,
                            'w' => config.warn = true,
                            _ => {
                                eprintln!("sha256sum: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ => break,
            }

            i += 1;
        }

        config.files = args[i..].to_vec();

        if config.algorithm != "sha256" {
            eprintln!(
                "sha256sum: algorithm '{}' is not supported (only sha256 is supported)",
                config.algorithm
            );
            return None;
        }

        Some(config)
    }
}
