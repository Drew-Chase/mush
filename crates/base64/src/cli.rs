const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: base64 [OPTION]... [FILE]

Base64 encode or decode FILE, or standard input, to standard output.
With no FILE, or when FILE is -, read standard input.

  -d, --decode          decode data
  -i, --ignore-garbage  when decoding, ignore non-alphabet characters
  -w, --wrap COLS       wrap encoded lines after COLS character (default 76).
                        Use 0 to disable line wrapping
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base64Config {
    pub decode: bool,
    pub ignore_garbage: bool,
    pub wrap: usize,
    pub file: Option<String>,
}

impl Default for Base64Config {
    fn default() -> Self {
        Self {
            decode: false,
            ignore_garbage: false,
            wrap: 76,
            file: None,
        }
    }
}

impl Base64Config {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = Base64Config::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("base64 {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--decode" => config.decode = true,
                "--ignore-garbage" => config.ignore_garbage = true,
                "--wrap" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("base64: option '--wrap' requires an argument");
                        return None;
                    }
                    match args[i].parse::<usize>() {
                        Ok(n) => config.wrap = n,
                        Err(_) => {
                            eprintln!("base64: invalid wrap size: '{}'", args[i]);
                            return None;
                        }
                    }
                }
                _ if arg.starts_with('-') && arg != "-" => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'd' => config.decode = true,
                            'i' => config.ignore_garbage = true,
                            'w' => {
                                // Consume rest of this arg or next arg as wrap value
                                if j + 1 < chars.len() {
                                    let val: String = chars[j + 1..].iter().collect();
                                    match val.parse::<usize>() {
                                        Ok(n) => config.wrap = n,
                                        Err(_) => {
                                            eprintln!("base64: invalid wrap size: '{val}'");
                                            return None;
                                        }
                                    }
                                    j = chars.len();
                                    continue;
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("base64: option '-w' requires an argument");
                                        return None;
                                    }
                                    match args[i].parse::<usize>() {
                                        Ok(n) => config.wrap = n,
                                        Err(_) => {
                                            eprintln!(
                                                "base64: invalid wrap size: '{}'",
                                                args[i]
                                            );
                                            return None;
                                        }
                                    }
                                }
                            }
                            _ => {
                                eprintln!("base64: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ => {
                    config.file = Some(arg.clone());
                    i += 1;
                    break;
                }
            }

            i += 1;
        }

        // If there's still a positional arg left and no file set
        if config.file.is_none() && i < args.len() {
            config.file = Some(args[i].clone());
        }

        Some(config)
    }
}
