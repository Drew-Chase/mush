use clap::Parser;

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

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(name = "tput", about = "Query terminal capabilities", version, disable_help_flag = true)]
pub struct TputConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(required = true, help = "Capability name and optional parameters")]
    pub args: Vec<String>,

    #[clap(skip)]
    pub capability: Option<TputCapability>,
}

impl TputConfig {
    pub fn resolve(&mut self) -> Result<(), String> {
        if self.args.is_empty() {
            return Err("missing capability name".to_string());
        }

        let capability = match self.args[0].as_str() {
            "cols" => TputCapability::Cols,
            "lines" => TputCapability::Lines,
            "colors" => TputCapability::Colors,
            "bold" => TputCapability::Bold,
            "sgr0" => TputCapability::Sgr0,
            "setaf" => {
                let color: u8 = self.args.get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                TputCapability::Setaf(color)
            }
            "clear" => TputCapability::Clear,
            "cup" => {
                let row: u16 = self.args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                let col: u16 = self.args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                TputCapability::Cup(row, col)
            }
            other => {
                return Err(format!("unknown capability '{other}'"));
            }
        };

        self.capability = Some(capability);
        Ok(())
    }

    pub fn get_capability(&self) -> &TputCapability {
        self.capability.as_ref().expect("resolve() must be called first")
    }
}
