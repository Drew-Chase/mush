use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "wc",
    about = "Print newline, word, and byte counts for each FILE, and a total line if\nmore than one FILE is specified. A word is a non-zero-length sequence of\nprintable characters delimited by white space.\n\nWith no FILE, or when FILE is -, read standard input.",
    version,
    disable_help_flag = true
)]
pub struct WcConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Print the newline counts
    #[arg(short = 'l', long = "lines")]
    pub lines: bool,

    /// Print the word counts
    #[arg(short = 'w', long = "words")]
    pub words: bool,

    /// Print the byte counts
    #[arg(short = 'c', long = "bytes")]
    pub bytes: bool,

    /// Print the character counts
    #[arg(short = 'm', long = "chars")]
    pub chars: bool,

    /// Print the maximum display width
    #[arg(short = 'L', long = "max-line-length")]
    pub max_line_length: bool,

    /// Files to read
    pub files: Vec<String>,
}

impl WcConfig {
    /// Apply default flag behavior: if no flags are set, enable lines+words+bytes
    pub fn apply_defaults(&mut self) {
        if !self.lines && !self.words && !self.bytes && !self.chars && !self.max_line_length {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
    }
}
