use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "less",
    about = "A pager similar to more, but with the ability to scroll backwards.",
    version,
    disable_help_flag = true
)]
pub struct LessConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Show line numbers
    #[arg(short = 'N', long = "line-numbers")]
    pub line_numbers: bool,

    /// Chop (truncate) long lines instead of wrapping
    #[arg(short = 'S', long = "chop-long-lines")]
    pub chop_long_lines: bool,

    /// Ignore case in searches
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Quit if entire file fits on one screen
    #[arg(short = 'F', long = "quit-if-one-screen")]
    pub quit_if_one_screen: bool,

    /// Output raw control characters
    #[arg(short = 'R', long = "RAW-CONTROL-CHARS")]
    pub raw_control_chars: bool,

    /// Don't clear the screen on init/exit
    #[arg(short = 'X', long = "no-init")]
    pub no_init: bool,

    /// Start displaying at line number NUM
    #[arg(short = 'n')]
    pub start_line: Option<usize>,

    /// Start at first occurrence of PATTERN (use +/PATTERN syntax)
    #[arg(skip)]
    pub start_pattern: Option<String>,

    /// Files to read
    pub files: Vec<String>,
}
