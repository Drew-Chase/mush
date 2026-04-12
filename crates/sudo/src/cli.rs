use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(
    name = "sudo",
    about = "Execute a command as another user.",
    version,
    disable_help_flag = true
)]
pub struct SudoConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Run command as USER (default: root)
    #[arg(short = 'u', long = "user")]
    pub user: Option<String>,

    /// Run login shell as the target user
    #[arg(short = 'i', long = "login")]
    pub login: bool,

    /// Run shell as the target user
    #[arg(short = 's', long = "shell")]
    pub shell: bool,

    /// Preserve user environment when running command
    #[arg(short = 'E', long = "preserve-env")]
    pub preserve_env: bool,

    /// Command and arguments to execute
    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,
}
