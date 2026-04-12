use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "uniq",
    about = "Filter adjacent matching lines from INPUT (or standard input),\nwriting to OUTPUT (or standard output).\n\nWith no options, matching lines are merged to the first occurrence.",
    version,
    disable_help_flag = true
)]
pub struct UniqConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Prefix lines by the number of occurrences
    #[arg(short = 'c', long = "count")]
    pub count: bool,

    /// Only print duplicate lines, one for each group
    #[arg(short = 'd', long = "repeated")]
    pub repeated: bool,

    /// Print all duplicate lines
    #[arg(short = 'D')]
    pub all_repeated: bool,

    /// Only print unique lines
    #[arg(short = 'u', long = "unique")]
    pub unique: bool,

    /// Ignore differences in case when comparing
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Avoid comparing the first N fields
    #[arg(short = 'f', long = "skip-fields", default_value_t = 0)]
    pub skip_fields: usize,

    /// Avoid comparing the first N characters
    #[arg(short = 's', long = "skip-chars", default_value_t = 0)]
    pub skip_chars: usize,

    /// Compare no more than N characters in lines
    #[arg(short = 'w', long = "check-chars")]
    pub check_chars: Option<usize>,

    /// Input file
    pub input: Option<String>,

    /// Output file
    pub output: Option<String>,
}
