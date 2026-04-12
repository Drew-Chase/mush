use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "realpath",
    about = "Print the resolved absolute file name.",
    version,
    disable_help_flag = true
)]
pub struct RealpathConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'e', long = "canonicalize-existing", help = "All components of the path must exist")]
    pub canonicalize_existing: bool,

    #[arg(short = 'm', long = "canonicalize-missing", help = "No path components need exist or be a directory")]
    pub canonicalize_missing: bool,

    #[arg(short = 's', long = "strip", alias = "no-symlinks", help = "Don't expand symlinks")]
    pub no_symlinks: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress most error messages")]
    pub quiet: bool,

    #[arg(short = 'z', long = "zero", help = "End each output line with NUL, not newline")]
    pub zero: bool,

    pub files: Vec<String>,
}
