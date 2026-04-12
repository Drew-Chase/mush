use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "comm",
    about = "Compare two sorted files line by line.",
    version,
    disable_help_flag = true
)]
pub struct CommConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Suppress column 1 (lines unique to FILE1)
    #[arg(short = '1')]
    pub suppress1: bool,

    /// Suppress column 2 (lines unique to FILE2)
    #[arg(short = '2')]
    pub suppress2: bool,

    /// Suppress column 3 (lines that appear in both)
    #[arg(short = '3')]
    pub suppress3: bool,

    /// Ignore differences in case when comparing
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Separate columns with STR
    #[arg(long = "output-delimiter")]
    pub output_delimiter: Option<String>,

    /// First file
    pub file1: String,

    /// Second file
    pub file2: String,
}
