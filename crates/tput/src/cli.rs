const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: tput CAPABILITY [PARAMS]
Query terminal capabilities.

Capabilities:
  cols           number of columns
  lines          number of lines
  colors         number of colors
  bold           turn on bold
  sgr0           turn off all attributes
  setaf COLOR    set foreground color
  clear          clear screen
  cup ROW COL    move cursor to position

Options:
  -V, --version  output version information and exit
  -h, --help     display this help and exit";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TputCapability {
    Cols,
    Lines,
    Colors,
    Bold,
    Sgr0,
    Setaf(u8),
    Clear,
    Cup(u16, u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TputConfig {
    pub capability: TputCapability,
}

impl TputConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        if args.is_empty() {
            eprintln!("tput: missing capability name");
            return None;
        }

        let first = &args[0];

        if first == "--help" || first == "-h" {
            println!("{HELP_TEXT}");
            return None;
        }
        if first == "--version" || first == "-V" {
            println!("tput {VERSION}");
            return None;
        }

        let capability = match first.as_str() {
            "cols" => TputCapability::Cols,
            "lines" => TputCapability::Lines,
            "colors" => TputCapability::Colors,
            "bold" => TputCapability::Bold,
            "sgr0" => TputCapability::Sgr0,
            "setaf" => {
                let color: u8 = args.get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                TputCapability::Setaf(color)
            }
            "clear" => TputCapability::Clear,
            "cup" => {
                let row: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                let col: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                TputCapability::Cup(row, col)
            }
            other => {
                eprintln!("tput: unknown capability '{other}'");
                return None;
            }
        };

        Some(TputConfig { capability })
    }
}
