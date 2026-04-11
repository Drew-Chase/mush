const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: uniq [OPTION]... [INPUT [OUTPUT]]

Filter adjacent matching lines from INPUT (or standard input),
writing to OUTPUT (or standard output).

With no options, matching lines are merged to the first occurrence.

  -c, --count           prefix lines by the number of occurrences
  -d, --repeated        only print duplicate lines, one for each group
  -D                    print all duplicate lines
  -u, --unique          only print unique lines
  -i, --ignore-case     ignore differences in case when comparing
  -f, --skip-fields N   avoid comparing the first N fields
  -s, --skip-chars N    avoid comparing the first N characters
  -w, --check-chars N   compare no more than N characters in lines
      --help     display this help and exit
      --version  output version information and exit";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UniqConfig {
    pub count: bool,
    pub repeated: bool,
    pub all_repeated: bool,
    pub unique: bool,
    pub ignore_case: bool,
    pub skip_fields: usize,
    pub skip_chars: usize,
    pub check_chars: Option<usize>,
    pub input: Option<String>,
    pub output: Option<String>,
}

impl UniqConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = UniqConfig::default();
        let mut positionals: Vec<String> = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("uniq {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            match arg.as_str() {
                "--count" => config.count = true,
                "--repeated" => config.repeated = true,
                "--unique" => config.unique = true,
                "--ignore-case" => config.ignore_case = true,
                "--skip-fields" => {
                    i += 1;
                    config.skip_fields = args.get(i)?.parse().ok()?;
                }
                "--skip-chars" => {
                    i += 1;
                    config.skip_chars = args.get(i)?.parse().ok()?;
                }
                "--check-chars" => {
                    i += 1;
                    config.check_chars = Some(args.get(i)?.parse().ok()?);
                }
                _ if arg.starts_with("--skip-fields=") => {
                    config.skip_fields = arg.strip_prefix("--skip-fields=")?.parse().ok()?;
                }
                _ if arg.starts_with("--skip-chars=") => {
                    config.skip_chars = arg.strip_prefix("--skip-chars=")?.parse().ok()?;
                }
                _ if arg.starts_with("--check-chars=") => {
                    config.check_chars = Some(arg.strip_prefix("--check-chars=")?.parse().ok()?);
                }
                _ if arg.starts_with('-') && arg.len() > 1 && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'c' => config.count = true,
                            'd' => config.repeated = true,
                            'D' => config.all_repeated = true,
                            'u' => config.unique = true,
                            'i' => config.ignore_case = true,
                            'f' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.skip_fields = rest.parse().ok()?;
                                } else {
                                    i += 1;
                                    config.skip_fields = args.get(i)?.parse().ok()?;
                                }
                                j = chars.len();
                                continue;
                            }
                            's' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.skip_chars = rest.parse().ok()?;
                                } else {
                                    i += 1;
                                    config.skip_chars = args.get(i)?.parse().ok()?;
                                }
                                j = chars.len();
                                continue;
                            }
                            'w' => {
                                let rest: String = chars[j + 1..].iter().collect();
                                if !rest.is_empty() {
                                    config.check_chars = Some(rest.parse().ok()?);
                                } else {
                                    i += 1;
                                    config.check_chars = Some(args.get(i)?.parse().ok()?);
                                }
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("uniq: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ => {
                    positionals.push(arg.clone());
                }
            }

            i += 1;
        }

        // Remaining args after -- are positionals
        positionals.extend(args[i..].iter().cloned());

        if let Some(inp) = positionals.first() {
            config.input = Some(inp.clone());
        }
        if let Some(outp) = positionals.get(1) {
            config.output = Some(outp.clone());
        }

        Some(config)
    }
}
