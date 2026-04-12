use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "reset", about = "Reset the terminal to a sane state.", version, disable_help_flag = true)]
pub struct ResetConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,
}
