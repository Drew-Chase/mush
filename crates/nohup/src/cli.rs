use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(
    name = "nohup",
    about = "Run COMMAND, ignoring hangup signals.",
    version,
    disable_help_flag = true
)]
pub struct NohupConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Command and arguments to run
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,
}
