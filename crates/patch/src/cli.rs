use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "patch",
    about = "Apply a unified diff to files.",
    version,
    disable_help_flag = true
)]
pub struct PatchConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Strip NUM leading path components
    #[arg(short = 'p', default_value_t = 0)]
    pub strip: usize,

    /// Reverse the patch
    #[arg(short = 'R')]
    pub reverse: bool,

    /// Do not actually modify any files
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Create backup files (.orig)
    #[arg(short = 'b')]
    pub backup: bool,

    /// Read patch from FILE
    #[arg(short = 'i')]
    pub patch_file: Option<String>,

    /// Original file (positional)
    #[arg()]
    pub original_file: Option<String>,
}
