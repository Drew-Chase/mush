use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "du",
    about = "Summarize device usage of the set of FILEs, recursively for directories",
    version,
    disable_help_flag = true
)]
pub struct DuConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'h', long = "human-readable")]
    pub human_readable: bool,

    #[arg(short = 's', long = "summarize")]
    pub summarize: bool,

    #[arg(short = 'a', long = "all")]
    pub all: bool,

    #[arg(short = 'c', long = "total")]
    pub total: bool,

    #[arg(short = 'd', long = "max-depth")]
    pub max_depth: Option<usize>,

    #[arg(long = "apparent-size")]
    pub apparent_size: bool,

    #[arg(short = 'b', long = "bytes")]
    pub bytes: bool,

    #[arg(short = 'k')]
    pub kilobytes: bool,

    #[arg(short = 'm')]
    pub megabytes: bool,

    pub files: Vec<String>,
}
