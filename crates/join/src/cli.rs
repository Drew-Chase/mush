use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "join",
    about = "For each pair of input lines with identical join fields, write a line to standard output",
    version,
    disable_help_flag = true
)]
pub struct JoinConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = '1', default_value_t = 1)]
    pub field1: usize,

    #[arg(short = '2', default_value_t = 1)]
    pub field2: usize,

    #[arg(short = 't')]
    pub separator: Option<char>,

    #[arg(short = 'a')]
    pub unpaired: Vec<String>,

    #[arg(short = 'v')]
    pub only_unpaired: Vec<String>,

    #[arg(short = 'e')]
    pub empty: Option<String>,

    #[arg(short = 'o')]
    pub format: Option<String>,

    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    pub file1: String,

    pub file2: String,

    // Derived fields (not set by clap)
    #[arg(skip)]
    pub unpaired1: bool,

    #[arg(skip)]
    pub unpaired2: bool,

    #[arg(skip)]
    pub only_unpaired1: bool,

    #[arg(skip)]
    pub only_unpaired2: bool,
}

impl JoinConfig {
    /// Resolve -a and -v flags into the boolean fields.
    pub fn resolve(&mut self) {
        for val in &self.unpaired {
            match val.as_str() {
                "1" => self.unpaired1 = true,
                "2" => self.unpaired2 = true,
                _ => {}
            }
        }
        for val in &self.only_unpaired {
            match val.as_str() {
                "1" => self.only_unpaired1 = true,
                "2" => self.only_unpaired2 = true,
                _ => {}
            }
        }
    }
}
