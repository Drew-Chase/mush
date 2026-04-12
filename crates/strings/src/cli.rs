use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "strings",
    about = "Print the printable character sequences that are at least N characters long.",
    version,
    disable_help_flag = true
)]
pub struct StringsConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Print sequences of at least N characters
    #[arg(short = 'n', long = "bytes", default_value_t = 4)]
    pub min_length: usize,

    /// Scan the whole file (default)
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Print the offset with radix CHAR (o, x, or d)
    #[arg(short = 't', long = "radix")]
    pub radix: Option<char>,

    /// Files to process
    pub files: Vec<String>,
}

impl Default for StringsConfig {
    fn default() -> Self {
        Self {
            help: None,
            min_length: 4,
            all: false,
            radix: None,
            files: Vec::new(),
        }
    }
}
