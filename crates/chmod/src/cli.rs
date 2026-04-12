use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "chmod",
    about = "Change the mode of each FILE to MODE.",
    version,
    disable_help_flag = true
)]
pub struct ChmodConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Change files and directories recursively
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

    /// The permission mode (octal or symbolic) followed by files
    #[arg(required = true)]
    pub files: Vec<String>,

    /// The permission mode (populated from first positional)
    #[arg(skip)]
    pub mode: String,
}

impl ChmodConfig {
    /// Post-process: split first positional as mode
    pub fn resolve(mut self) -> Result<Self, String> {
        if self.files.is_empty() {
            return Err("chmod: missing operand".to_string());
        }
        self.mode = self.files.remove(0);
        if self.files.is_empty() {
            return Err(format!("chmod: missing operand after '{}'", self.mode));
        }
        Ok(self)
    }
}
