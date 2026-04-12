use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "su",
    about = "Switch to another user (default: root).",
    version,
    disable_help_flag = true
)]
pub struct SuConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Pass CMD to the invoked shell
    #[arg(short = 'c', long = "command")]
    pub command: Option<String>,

    /// Make the shell a login shell
    #[arg(short = 'l', long = "login")]
    pub login: bool,

    /// Run SHELL instead of the default
    #[arg(short = 's', long = "shell")]
    pub shell: Option<String>,

    /// User to switch to
    #[arg(default_value = "root")]
    pub user: String,
}

impl Default for SuConfig {
    fn default() -> Self {
        Self {
            help: None,
            command: None,
            login: false,
            shell: None,
            user: "root".to_string(),
        }
    }
}
