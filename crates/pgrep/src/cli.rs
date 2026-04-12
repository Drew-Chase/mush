use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "pgrep",
    about = "Look up processes by name pattern.",
    version,
    disable_help_flag = true
)]
pub struct PgrepConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// List PID and process name
    #[arg(short = 'l', long = "list-name")]
    pub list_name: bool,

    /// List PID and full command line
    #[arg(short = 'a', long = "list-full")]
    pub list_full: bool,

    /// Display count of matching processes
    #[arg(short = 'c', long = "count")]
    pub count: bool,

    /// Set output delimiter (default: newline)
    #[arg(short = 'd', long = "delimiter", default_value = "\n")]
    pub delimiter: String,

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

impl Default for PgrepConfig {
    fn default() -> Self {
        Self {
            help: None,
            list_name: false,
            list_full: false,
            count: false,
            delimiter: "\n".to_string(),
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
