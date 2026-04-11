const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: tr [OPTION]... SET1 [SET2]

Translate, squeeze, and/or delete characters from standard input,
writing to standard output.

  -c, -C, --complement   use the complement of SET1
  -d, --delete            delete characters in SET1, do not translate
  -s, --squeeze-repeats   replace each sequence of a repeated character
                          that is listed in the last specified SET,
                          with a single occurrence of that character
  -t, --truncate-set1     first truncate SET1 to length of SET2
      --help              display this help and exit
      --version           output version information and exit";

#[derive(Debug, Clone, PartialEq)]
pub struct TrConfig {
    pub complement: bool,
    pub delete: bool,
    pub squeeze: bool,
    pub truncate: bool,
    pub set1: String,
    pub set2: Option<String>,
}

impl TrConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut complement = false;
        let mut delete = false;
        let mut squeeze = false;
        let mut truncate = false;
        let mut positionals: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" => {
                    println!("{HELP_TEXT}");
                    return None;
                }
                "--version" => {
                    println!("tr {VERSION}");
                    return None;
                }
                "--complement" => complement = true,
                "--delete" => delete = true,
                "--squeeze-repeats" => squeeze = true,
                "--truncate-set1" => truncate = true,
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 1 => {
                    for ch in arg[1..].chars() {
                        match ch {
                            'c' | 'C' => complement = true,
                            'd' => delete = true,
                            's' => squeeze = true,
                            't' => truncate = true,
                            _ => {
                                eprintln!("tr: invalid option -- '{ch}'");
                                return None;
                            }
                        }
                    }
                }
                _ => positionals.push(arg.clone()),
            }
            i += 1;
        }

        if positionals.is_empty() {
            eprintln!("tr: missing operand");
            return None;
        }

        let set1 = positionals[0].clone();
        let set2 = if positionals.len() > 1 {
            Some(positionals[1].clone())
        } else {
            None
        };

        Some(TrConfig {
            complement,
            delete,
            squeeze,
            truncate,
            set1,
            set2,
        })
    }
}
