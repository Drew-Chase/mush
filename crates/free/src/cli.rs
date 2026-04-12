use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(
    name = "free",
    about = "Display amount of free and used memory in the system",
    version,
    disable_help_flag = true
)]
pub struct FreeConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'b')]
    pub bytes: bool,

    #[arg(short = 'k')]
    pub kibi: bool,

    #[arg(short = 'm')]
    pub mebi: bool,

    #[arg(short = 'g')]
    pub gibi: bool,

    #[arg(short = 'h', long = "human")]
    pub human: bool,

    #[arg(long = "si")]
    pub si: bool,

    #[arg(short = 't', long = "total")]
    pub total: bool,

    #[arg(short = 'w', long = "wide")]
    pub wide: bool,
}
