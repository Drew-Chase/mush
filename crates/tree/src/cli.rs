use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "tree", about = "List contents of directories in a tree-like format", version, disable_help_flag = true)]
pub struct TreeConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', long, help = "Include hidden files")]
    pub all: bool,

    #[arg(short = 'd', long = "dirs-only", help = "List directories only")]
    pub dirs_only: bool,

    #[arg(short = 'f', long = "full-path", help = "Print the full path prefix for each file")]
    pub full_path: bool,

    #[arg(short = 'L', long, help = "Descend only DEPTH directories deep")]
    pub level: Option<usize>,

    #[arg(short = 'I', long, help = "Exclude files matching pattern")]
    pub exclude: Option<String>,

    #[arg(short = 'P', long, help = "List only files matching pattern")]
    pub pattern: Option<String>,

    #[arg(short = 's', long = "size", help = "Print the size of each file")]
    pub show_size: bool,

    #[arg(short = 'h', long = "human-readable", help = "Print size in human readable format")]
    pub human_readable: bool,

    #[arg(short = 'D', long = "date", help = "Print the date of last modification")]
    pub show_date: bool,

    #[arg(long = "dirsfirst", help = "List directories before files")]
    pub dirs_first: bool,

    #[arg(long = "noreport", help = "Omit the file and directory report at end")]
    pub no_report: bool,

    #[arg(short = 'C', long, help = "Turn colorization on always")]
    pub color: bool,

    #[arg(short = 'J', long, help = "Output as JSON")]
    pub json: bool,

    pub paths: Vec<String>,
}
