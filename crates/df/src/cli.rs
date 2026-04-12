use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(
    name = "df",
    about = "Report filesystem disk space usage",
    version,
    disable_help_flag = true
)]
pub struct DfConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'h', long = "human-readable")]
    pub human_readable: bool,

    #[arg(short = 'H', long = "si")]
    pub si: bool,

    #[arg(short = 'T', long = "print-type")]
    pub print_type: bool,

    #[arg(short = 't', long = "type")]
    pub type_filter: Option<String>,

    #[arg(short = 'a', long = "all")]
    pub all: bool,

    #[arg(long = "total")]
    pub total: bool,

    pub files: Vec<String>,
}
