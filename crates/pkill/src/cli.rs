use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "pkill",
    about = "Signal processes by name pattern.",
    version,
    disable_help_flag = true
)]
pub struct PkillConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Signal to send (default: TERM)
    #[arg(short = 's', long = "signal", default_value = "TERM")]
    pub signal: String,

    /// Match against full command line
    #[arg(short = 'f', long = "full")]
    pub full: bool,

    /// Case-insensitive matching
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Require exact match of process name
    #[arg(short = 'x', long = "exact")]
    pub exact: bool,

    /// Match only processes owned by USER
    #[arg(short = 'u', long = "euid")]
    pub user_filter: Option<String>,

    /// Select most recently started
    #[arg(short = 'n', long = "newest")]
    pub newest: bool,

    /// Select least recently started
    #[arg(short = 'o', long = "oldest")]
    pub oldest: bool,

    /// Pattern to match
    #[arg(default_value = "")]
    pub pattern: String,
}

impl Default for PkillConfig {
    fn default() -> Self {
        Self {
            help: None,
            signal: "TERM".to_string(),
            full: false,
            ignore_case: false,
            exact: false,
            user_filter: None,
            newest: false,
            oldest: false,
            pattern: String::new(),
        }
    }
}
