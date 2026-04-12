use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "whoami", about = "Print the current user name.", version, disable_help_flag = true)]
pub struct WhoamiConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,
}
