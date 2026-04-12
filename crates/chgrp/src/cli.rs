use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "chgrp",
    about = "Change the group of each FILE to GROUP.",
    version,
    disable_help_flag = true
)]
pub struct ChgrpConfig {
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

    /// Use RFILE's group
    #[arg(long = "reference")]
    pub reference: Option<String>,

    /// Group name and files (first arg is group unless --reference is used)
    pub files: Vec<String>,

    /// Group name (populated from first positional when --reference is absent)
    #[arg(skip)]
    pub group: String,
}

impl ChgrpConfig {
    /// Post-process: split first positional as group when --reference is absent
    pub fn resolve(mut self) -> Result<Self, String> {
        if self.reference.is_none() {
            if self.files.is_empty() {
                return Err("chgrp: missing operand".to_string());
            }
            self.group = self.files.remove(0);
        }
        if self.files.is_empty() {
            return Err("chgrp: missing operand".to_string());
        }
        Ok(self)
    }
}
