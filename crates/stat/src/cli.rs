use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "stat",
    about = "Display file or file system status.",
    version,
    disable_help_flag = true
)]
pub struct StatConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Follow links
    #[arg(short = 'L', long = "dereference")]
    pub dereference: bool,

    /// Use the specified FORMAT instead of the default
    #[arg(short = 'c', long = "format")]
    pub format: Option<String>,

    /// Print the information in terse form
    #[arg(short = 't', long = "terse")]
    pub terse: bool,

    /// Files to stat
    #[arg(required = true)]
    pub files: Vec<String>,
}
