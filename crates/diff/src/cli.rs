const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: diff [OPTION]... FILE1 FILE2

Compare files line by line.

  -u, --unified[=NUM]       output NUM (default 3) lines of unified context
  -c, --context[=NUM]       output NUM (default 3) lines of copied context
  -y, --side-by-side        output in two columns
  -W, --width NUM           output at most NUM (default 130) print columns
  -i, --ignore-case         ignore case differences in file contents
  -b, --ignore-space-change ignore changes in the amount of white space
  -w, --ignore-all-space    ignore all white space
  -B, --ignore-blank-lines  ignore changes where lines are all blank
  -r, --recursive           recursively compare any subdirectories found
  -q, --brief               report only when files differ
  -s, --report-identical-files report when two files are identical
      --color               colorize the output
      --help                display this help and exit
      --version             output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiffConfig {
    pub unified: Option<usize>,
    pub context: Option<usize>,
    pub side_by_side: bool,
    pub width: usize,
    pub ignore_case: bool,
    pub ignore_space_change: bool,
    pub ignore_all_space: bool,
    pub ignore_blank_lines: bool,
    pub recursive: bool,
    pub brief: bool,
    pub report_identical: bool,
    pub color: bool,
    pub file1: String,
    pub file2: String,
}

impl DiffConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = DiffConfig {
            width: 130,
            ..Default::default()
        };
        let mut positional: Vec<String> = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("diff {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            // Long options with = form
            if let Some(rest) = arg.strip_prefix("--unified=") {
                config.unified = Some(parse_num_arg("--unified", rest)?);
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--context=") {
                config.context = Some(parse_num_arg("--context", rest)?);
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--width=") {
                config.width = parse_num_arg("--width", rest)?;
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--unified" => {
                    if i + 1 < args.len()
                        && let Ok(n) = args[i + 1].parse::<usize>()
                    {
                        config.unified = Some(n);
                        i += 2;
                        continue;
                    }
                    config.unified = Some(3);
                }
                "--context" => {
                    if i + 1 < args.len()
                        && let Ok(n) = args[i + 1].parse::<usize>()
                    {
                        config.context = Some(n);
                        i += 2;
                        continue;
                    }
                    config.context = Some(3);
                }
                "--side-by-side" => config.side_by_side = true,
                "--width" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("diff: option '--width' requires an argument");
                        return None;
                    }
                    config.width = parse_num_arg("--width", &args[i])?;
                }
                "--ignore-case" => config.ignore_case = true,
                "--ignore-space-change" => config.ignore_space_change = true,
                "--ignore-all-space" => config.ignore_all_space = true,
                "--ignore-blank-lines" => config.ignore_blank_lines = true,
                "--recursive" => config.recursive = true,
                "--brief" => config.brief = true,
                "--report-identical-files" => config.report_identical = true,
                "--color" => config.color = true,
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg != "-" => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'u' => {
                                // Check if remaining chars are digits
                                if j + 1 < chars.len() && chars[j + 1].is_ascii_digit() {
                                    let val: String = chars[j + 1..].iter().collect();
                                    config.unified = Some(parse_num_arg("-u", &val)?);
                                    j = chars.len();
                                    continue;
                                }
                                config.unified = Some(3);
                            }
                            'c' => {
                                if j + 1 < chars.len() && chars[j + 1].is_ascii_digit() {
                                    let val: String = chars[j + 1..].iter().collect();
                                    config.context = Some(parse_num_arg("-c", &val)?);
                                    j = chars.len();
                                    continue;
                                }
                                config.context = Some(3);
                            }
                            'y' => config.side_by_side = true,
                            'W' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("diff: option requires an argument -- 'W'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.width = parse_num_arg("-W", &val)?;
                                j = chars.len();
                                continue;
                            }
                            'i' => config.ignore_case = true,
                            'b' => config.ignore_space_change = true,
                            'w' => config.ignore_all_space = true,
                            'B' => config.ignore_blank_lines = true,
                            'r' => config.recursive = true,
                            'q' => config.brief = true,
                            's' => config.report_identical = true,
                            _ => {
                                eprintln!("diff: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("diff: unrecognized option '{arg}'");
                    return None;
                }
                _ => {
                    positional.push(arg.clone());
                }
            }

            i += 1;
        }

        // Collect remaining positional args
        while i < args.len() {
            positional.push(args[i].clone());
            i += 1;
        }

        if positional.len() < 2 {
            eprintln!("diff: missing operand");
            return None;
        }

        config.file1 = positional[0].clone();
        config.file2 = positional[1].clone();

        Some(config)
    }
}

fn parse_num_arg(flag: &str, val: &str) -> Option<usize> {
    match val.parse::<usize>() {
        Ok(n) => Some(n),
        Err(_) => {
            eprintln!("diff: invalid argument '{val}' for '{flag}'");
            None
        }
    }
}
