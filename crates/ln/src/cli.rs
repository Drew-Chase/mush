use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "ln",
    about = "Create a link to TARGET with the name LINK_NAME.",
    version,
    disable_help_flag = true
)]
pub struct LnConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Make symbolic links instead of hard links
    #[arg(short = 's', long = "symbolic")]
    pub symbolic: bool,

    /// Remove existing destination files
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Prompt whether to remove destinations
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Print name of each linked file
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Treat LINK_NAME as a normal file if it is a symbolic link to a directory
    #[arg(short = 'n', long = "no-dereference")]
    pub no_deref: bool,

    /// Target and link name paths
    pub targets: Vec<String>,
}
