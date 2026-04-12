use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "expand", about = "Convert tabs in each FILE to spaces, writing to standard output", version, disable_help_flag = true)]
pub struct ExpandConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 't', long = "tabs", default_value_t = 8, help = "Have tabs NUMBER characters apart, not 8")]
    pub tab_width: usize,

    #[arg(short = 'i', long = "initial", help = "Do not convert tabs after non blanks")]
    pub initial_only: bool,

    pub files: Vec<String>,
}
