use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "fmt", about = "Reformat each paragraph in the FILE(s), writing to standard output", version, disable_help_flag = true)]
pub struct FmtConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'w', long = "width", default_value_t = 75, help = "Maximum line width (default 75)")]
    pub width: usize,

    #[arg(short = 's', long = "split-only", help = "Split long lines, but do not refill")]
    pub split_only: bool,

    #[arg(short = 'u', long = "uniform-spacing", help = "One space between words, two after sentences")]
    pub uniform: bool,

    #[arg(short = 'p', long = "prefix", help = "Reformat only lines beginning with STRING")]
    pub prefix: Option<String>,

    pub files: Vec<String>,
}
