use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "more", about = "A filter for paging through text one screenful at a time", version, disable_help_flag = true)]
pub struct MoreConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 's', help = "Squeeze multiple adjacent blank lines into one")]
    pub squeeze: bool,

    #[arg(short = 'n', help = "Lines per screenful (default: terminal height - 1)")]
    pub lines_per_screen: Option<usize>,

    #[arg(long = "start-line", help = "Start displaying at line number NUM")]
    pub start_line: Option<usize>,

    pub files: Vec<String>,
}
