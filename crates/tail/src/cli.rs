use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "tail",
    about = "Print the last 10 lines of each FILE to standard output.",
    version,
    disable_help_flag = true
)]
pub struct TailConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Output the last NUM lines
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    pub lines: usize,

    /// Output the last NUM bytes
    #[arg(short = 'c', long = "bytes")]
    pub bytes: Option<usize>,

    /// Output appended data as the file grows
    #[arg(short = 'f', long = "follow")]
    pub follow: bool,

    /// Never output headers giving file names
    #[arg(short = 'q', long = "quiet", aliases = ["silent"])]
    pub quiet: bool,

    /// Always output headers giving file names
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Files to read
    pub files: Vec<String>,
}
