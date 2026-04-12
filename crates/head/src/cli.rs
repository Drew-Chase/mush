use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "head",
    about = "Print the first 10 lines of each FILE to standard output.",
    version,
    disable_help_flag = true
)]
pub struct HeadConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Print the first NUM lines instead of first 10
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    pub lines: usize,

    /// Print the first NUM bytes of each file
    #[arg(short = 'c', long = "bytes")]
    pub bytes: Option<usize>,

    /// Never print headers giving file names
    #[arg(short = 'q', long = "quiet", aliases = ["silent"])]
    pub quiet: bool,

    /// Always print headers giving file names
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Files to read
    pub files: Vec<String>,
}
