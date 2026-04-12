use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(
    name = "id",
    about = "Print user and group information for the specified USER, or current user",
    version,
    disable_help_flag = true
)]
pub struct IdConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'u', long = "user")]
    pub user_only: bool,

    #[arg(short = 'g', long = "group")]
    pub group_only: bool,

    #[arg(short = 'G', long = "groups")]
    pub groups_only: bool,

    #[arg(short = 'n', long = "name")]
    pub name: bool,

    #[arg(short = 'r', long = "real")]
    pub real: bool,

    pub target_user: Option<String>,
}
