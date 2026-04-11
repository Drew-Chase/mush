const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = "\
Usage: seq [OPTION]... LAST
  or:  seq [OPTION]... FIRST LAST
  or:  seq [OPTION]... FIRST INCREMENT LAST

Print numbers from FIRST to LAST, in steps of INCREMENT.

  -f, --format=FORMAT   use printf style floating-point FORMAT
  -s, --separator=STRING use STRING to separate numbers (default: \\n)
  -w, --equal-width     equalize width by padding with leading zeroes
      --help            display this help and exit
      --version         output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct SeqConfig {
    pub first: f64,
    pub increment: f64,
    pub last: f64,
    pub separator: String,
    pub format: Option<String>,
    pub equal_width: bool,
}

impl SeqConfig {
    pub fn from_args(args: &[String]) -> Result<Option<Self>, String> {
        let mut format: Option<String> = None;
        let mut separator = "\n".to_string();
        let mut equal_width = false;
        let mut positional: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return Ok(None);
                }
                "--version" => {
                    println!("seq {VERSION}");
                    return Ok(None);
                }
                "-w" | "--equal-width" => equal_width = true,
                "-f" | "--format" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("seq: option '-f' requires an argument".to_string());
                    }
                    format = Some(args[i].clone());
                }
                "-s" | "--separator" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("seq: option '-s' requires an argument".to_string());
                    }
                    separator = args[i].clone();
                }
                _ if arg.starts_with("--format=") => {
                    format = Some(arg.strip_prefix("--format=").unwrap().to_string());
                }
                _ if arg.starts_with("--separator=") => {
                    separator = arg.strip_prefix("--separator=").unwrap().to_string();
                }
                _ if arg.starts_with("-f") && arg.len() > 2 => {
                    format = Some(arg[2..].to_string());
                }
                _ if arg.starts_with("-s") && arg.len() > 2 => {
                    separator = arg[2..].to_string();
                }
                _ => positional.push(arg.clone()),
            }
            i += 1;
        }

        let (first, increment, last) = match positional.len() {
            1 => {
                let last = parse_number(&positional[0])?;
                (1.0, 1.0, last)
            }
            2 => {
                let first = parse_number(&positional[0])?;
                let last = parse_number(&positional[1])?;
                (first, 1.0, last)
            }
            3 => {
                let first = parse_number(&positional[0])?;
                let increment = parse_number(&positional[1])?;
                let last = parse_number(&positional[2])?;
                (first, increment, last)
            }
            0 => return Err("seq: missing operand".to_string()),
            _ => return Err("seq: extra operand".to_string()),
        };

        Ok(Some(SeqConfig {
            first,
            increment,
            last,
            separator,
            format,
            equal_width,
        }))
    }
}

fn parse_number(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| format!("seq: invalid floating point argument: '{s}'"))
}
