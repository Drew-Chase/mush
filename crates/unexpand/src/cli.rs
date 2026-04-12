use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "unexpand", about = "Convert blanks in each FILE to tabs, writing to standard output", version, disable_help_flag = true)]
pub struct UnexpandConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', long = "all", help = "Convert all blanks, instead of just initial blanks")]
    pub all: bool,

    #[arg(short = 't', long = "tabs", default_value_t = 8, help = "Have tabs NUMBER characters apart, not 8")]
    pub tab_width: usize,

    #[arg(long = "first-only", help = "Convert only leading sequences of blanks")]
    pub first_only: bool,

    pub files: Vec<String>,
}
