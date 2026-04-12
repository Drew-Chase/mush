use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "paste",
    about = "Write lines consisting of the sequentially corresponding lines from each FILE, separated by TABs, to standard output.",
    version,
    disable_help_flag = true
)]
pub struct PasteConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Reuse characters from LIST instead of TABs
    #[arg(short = 'd', long = "delimiters", default_value = "\t")]
    pub delimiters: String,

    /// Paste one file at a time instead of in parallel
    #[arg(short = 's', long = "serial")]
    pub serial: bool,

    /// Input files
    #[arg()]
    pub files: Vec<String>,
}

impl Default for PasteConfig {
    fn default() -> Self {
        Self {
            help: None,
            delimiters: "\t".to_string(),
            serial: false,
            files: Vec::new(),
        }
    }
}
