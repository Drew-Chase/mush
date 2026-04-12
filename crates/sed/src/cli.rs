use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "sed",
    about = "Stream editor for filtering and transforming text.",
    version,
    disable_help_flag = true
)]
pub struct SedConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Suppress automatic printing of pattern space
    #[arg(short = 'n', long = "quiet", aliases = ["silent"])]
    pub quiet: bool,

    /// Add the script to the commands to be executed (can be repeated)
    #[arg(short = 'e', long = "expression")]
    pub scripts: Vec<String>,

    /// Add the contents of script-file to the commands
    #[arg(short = 'f', long = "file")]
    pub script_files: Vec<String>,

    /// Edit files in place (makes backup if SUFFIX supplied)
    #[arg(short = 'i', long = "in-place")]
    pub in_place: Option<Option<String>>,

    /// Use extended regular expressions in the script
    #[arg(short = 'r', visible_short_alias = 'E', long = "regexp-extended")]
    pub extended_regexp: bool,

    /// Consider files as separate rather than as a single continuous long stream
    #[arg(short = 's', long = "separate")]
    pub separate: bool,

    /// Script (first positional, used if no -e is given) and input files
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub positionals: Vec<String>,
}

impl SedConfig {
    /// Get the effective files list, accounting for the first positional being
    /// the script when no -e/-f is given.
    pub fn effective_files(&self) -> &[String] {
        if self.scripts.is_empty() && self.script_files.is_empty() {
            // First positional is the script, rest are files
            if self.positionals.len() > 1 {
                &self.positionals[1..]
            } else {
                &[]
            }
        } else {
            &self.positionals
        }
    }

    /// Get all scripts, including the implicit first positional if no -e/-f was given.
    pub fn effective_scripts(&self) -> Vec<String> {
        if self.scripts.is_empty() && self.script_files.is_empty() {
            if let Some(first) = self.positionals.first() {
                vec![first.clone()]
            } else {
                vec![]
            }
        } else {
            self.scripts.clone()
        }
    }
}
