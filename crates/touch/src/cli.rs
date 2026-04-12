use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "touch", about = "Update the access and modification times of each FILE to the current time", version, disable_help_flag = true)]
pub struct TouchConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', help = "Change only the access time")]
    pub access_only: bool,

    #[arg(short = 'm', help = "Change only the modification time")]
    pub modify_only: bool,

    #[arg(short = 'c', long = "no-create", help = "Do not create any files")]
    pub no_create: bool,

    #[arg(short = 'r', long, help = "Use this file's times instead of current time")]
    pub reference: Option<String>,

    #[arg(short = 'd', long, help = "Parse STRING and use it instead of current time")]
    pub date: Option<String>,

    pub files: Vec<String>,
}
