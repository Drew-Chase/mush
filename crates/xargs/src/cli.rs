use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "xargs",
    about = "Build and execute command lines from standard input.",
    version,
    disable_help_flag = true
)]
pub struct XargsConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Input items are terminated by NUL, not whitespace
    #[arg(short = '0', long = "null")]
    pub null: bool,

    /// Input items are terminated by CHAR
    #[arg(short = 'd', long = "delimiter")]
    pub delimiter: Option<char>,

    /// Use at most N arguments per command line
    #[arg(short = 'n', long = "max-args")]
    pub max_args: Option<usize>,

    /// Replace STR in COMMAND with input items
    #[arg(short = 'I', long = "replace")]
    pub replace: Option<String>,

    /// Use at most N input lines per command line
    #[arg(short = 'L', long = "max-lines")]
    pub max_lines: Option<usize>,

    /// Run at most N processes at a time
    #[arg(short = 'P', long = "max-procs", default_value_t = 1)]
    pub max_procs: usize,

    /// Print commands to stderr before executing
    #[arg(short = 't', long = "verbose")]
    pub verbose: bool,

    /// Prompt the user before executing each command
    #[arg(short = 'p', long = "interactive")]
    pub interactive: bool,

    /// If there are no input items, do not run the command
    #[arg(short = 'r', long = "no-run-if-empty")]
    pub no_run_if_empty: bool,

    /// Limit length of command line to N chars
    #[arg(short = 's', long = "max-chars")]
    pub max_chars: Option<usize>,

    /// Command and initial arguments
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,
}
