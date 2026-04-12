use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "arch", about = "Print machine architecture.", version, disable_help_flag = true)]
pub struct ArchConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,
}
