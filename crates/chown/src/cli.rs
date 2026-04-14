use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "chown",
    about = "Change the owner and/or group of each FILE to OWNER and/or GROUP.",
    version,
    disable_help_flag = true
)]
pub struct ChownConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Operate on files and directories recursively
    #[arg(short = 'R', long = "recursive")]
    pub recursive: bool,

    /// Output a diagnostic for every file processed
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Like verbose but report only when a change is made
    #[arg(short = 'c', long = "changes")]
    pub changes: bool,

    /// Suppress most error messages
    #[arg(short = 'f', long = "quiet", aliases = ["silent"])]
    pub quiet: bool,

    /// Affect symbolic links instead of referenced file
    #[arg(short = 'h', long = "no-dereference")]
    pub no_deref: bool,

    /// Use RFILE's owner and group
    #[arg(long = "reference")]
    pub reference: Option<String>,

    /// `Owner[:group]` and files (first arg is owner_group unless --reference is used)
    pub files: Vec<String>,

    /// `Owner[:group]` specification (populated from first positional when --reference is absent)
    #[arg(skip)]
    pub owner_group: String,
}

impl ChownConfig {
    /// Post-process: split first positional as owner_group when --reference is absent
    pub fn resolve(mut self) -> Result<Self, String> {
        if self.reference.is_none() {
            if self.files.is_empty() {
                return Err("chown: missing operand".to_string());
            }
            self.owner_group = self.files.remove(0);
        }
        if self.files.is_empty() {
            return Err("chown: missing operand".to_string());
        }
        Ok(self)
    }
}
