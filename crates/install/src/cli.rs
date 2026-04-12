use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "install",
    about = "Copy files and set attributes.",
    version,
    disable_help_flag = true
)]
pub struct InstallConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Create all leading directories, treat DEST as directory
    #[arg(short = 'd')]
    pub directory_mode: bool,

    /// Set permission mode (as in chmod)
    #[arg(short = 'm', long = "mode")]
    pub mode: Option<String>,

    /// Print the name of each file as it is installed
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Compare each pair of files; skip copy if identical
    #[arg(short = 'C', long = "compare")]
    pub compare: bool,

    /// Create all leading components of DEST
    #[arg(short = 'D')]
    pub create_leading: bool,

    /// Copy all SOURCE args into DIR
    #[arg(short = 't', long = "target-directory")]
    pub target_dir: Option<String>,

    /// Files and directories
    pub files: Vec<String>,
}
