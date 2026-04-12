use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "mktemp",
    about = "Create a temporary file or directory, safely, and print its name",
    version,
    disable_help_flag = true
)]
pub struct MktempConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'd', long = "directory")]
    pub directory: bool,

    #[arg(short = 'u', long = "dry-run")]
    pub dry_run: bool,

    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    #[arg(short = 'p', long = "tmpdir")]
    pub tmpdir: Option<String>,

    #[arg(long = "suffix")]
    pub suffix: Option<String>,

    pub template: Option<String>,
}
