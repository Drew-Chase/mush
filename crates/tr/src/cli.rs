use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "tr", about = "Translate, squeeze, and/or delete characters", version, disable_help_flag = true)]
pub struct TrConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'c', short_alias = 'C', long, help = "Use the complement of SET1")]
    pub complement: bool,

    #[arg(short = 'd', long, help = "Delete characters in SET1, do not translate")]
    pub delete: bool,

    #[arg(short = 's', long = "squeeze-repeats", help = "Replace each sequence of a repeated character with a single occurrence")]
    pub squeeze: bool,

    #[arg(short = 't', long = "truncate-set1", help = "First truncate SET1 to length of SET2")]
    pub truncate: bool,

    #[arg(required = true)]
    pub set1: String,

    pub set2: Option<String>,
}
