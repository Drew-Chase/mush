use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "fold", about = "Wrap input lines in each FILE, writing to standard output", version, disable_help_flag = true)]
pub struct FoldConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'w', long = "width", default_value_t = 80, help = "Use WIDTH columns instead of 80")]
    pub width: usize,

    #[arg(short = 'b', long = "bytes", help = "Count bytes rather than columns")]
    pub bytes: bool,

    #[arg(short = 's', long = "spaces", help = "Break at spaces")]
    pub spaces: bool,

    pub files: Vec<String>,
}
