use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "md5sum",
    about = "Print or check MD5 (128-bit) checksums",
    version,
    disable_help_flag = true
)]
pub struct Md5sumConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'b', long = "binary")]
    pub binary: bool,

    #[arg(short = 'c', long = "check")]
    pub check: bool,

    #[arg(long = "tag")]
    pub tag: bool,

    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    #[arg(long = "status")]
    pub status: bool,

    #[arg(short = 'w', long = "warn")]
    pub warn: bool,

    pub files: Vec<String>,
}
