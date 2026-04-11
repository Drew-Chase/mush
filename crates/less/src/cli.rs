const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: less [OPTION]... [FILE]...

A pager similar to more, but with the ability to scroll backwards.

With no FILE, or when FILE is -, read standard input.

  -N, --line-numbers       show line numbers
  -S, --chop-long-lines    chop (truncate) long lines instead of wrapping
  -i, --ignore-case        ignore case in searches
  -F, --quit-if-one-screen quit if entire file fits on one screen
  -R, --RAW-CONTROL-CHARS  output raw control characters
  -X, --no-init            don't clear the screen on init/exit
  -n NUM                   start displaying at line number NUM
  +NUM                     start displaying at line number NUM
  +/PATTERN                start at first occurrence of PATTERN
      --help               display this help and exit
      --version            output version information and exit

Interactive commands:
  q                quit
  j / Down         scroll down one line
  k / Up           scroll up one line
  f / Space / PgDn next page
  b / PgUp         previous page
  g / Home         go to first line
  G / End          go to last line
  /PATTERN         search forward
  n                next search match
  N                previous search match";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LessConfig {
    pub line_numbers: bool,
    pub chop_long_lines: bool,
    pub ignore_case: bool,
    pub quit_if_one_screen: bool,
    pub raw_control_chars: bool,
    pub no_init: bool,
    pub start_line: Option<usize>,
    pub start_pattern: Option<String>,
    pub files: Vec<String>,
}

impl LessConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = LessConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("less {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "-N" | "--line-numbers" => config.line_numbers = true,
                "-S" | "--chop-long-lines" => config.chop_long_lines = true,
                "-i" | "--ignore-case" => config.ignore_case = true,
                "-F" | "--quit-if-one-screen" => config.quit_if_one_screen = true,
                "-R" | "--RAW-CONTROL-CHARS" => config.raw_control_chars = true,
                "-X" | "--no-init" => config.no_init = true,
                "-n" => {
                    i += 1;
                    if i < args.len() {
                        config.start_line = args[i].parse().ok();
                    }
                }
                _ if arg.starts_with("+/") => {
                    let pattern = arg[2..].to_string();
                    if !pattern.is_empty() {
                        config.start_pattern = Some(pattern);
                    }
                }
                _ if arg.starts_with('+') && arg.len() > 1 => {
                    if let Ok(n) = arg[1..].parse::<usize>() {
                        config.start_line = Some(n);
                    }
                }
                _ if arg.starts_with('-') && arg.len() > 1 && arg != "-" => {
                    // Handle combined short flags like -NS
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut valid = true;
                    for ch in &chars {
                        match ch {
                            'N' => config.line_numbers = true,
                            'S' => config.chop_long_lines = true,
                            'i' => config.ignore_case = true,
                            'F' => config.quit_if_one_screen = true,
                            'R' => config.raw_control_chars = true,
                            'X' => config.no_init = true,
                            _ => {
                                eprintln!("less: invalid option -- '{ch}'");
                                valid = false;
                                break;
                            }
                        }
                    }
                    if !valid {
                        return None;
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
