use clap::Parser;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverwriteMode {
    #[default]
    Force,
    Interactive,
    NoClobber,
}

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "cp",
    about = "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.",
    version,
    disable_help_flag = true
)]
pub struct CpConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Do not prompt before overwriting
    #[arg(short = 'f', long = "force", overrides_with_all = ["interactive_flag", "no_clobber_flag"])]
    pub force_flag: bool,

    /// Prompt before overwrite
    #[arg(short = 'i', long = "interactive", overrides_with_all = ["force_flag", "no_clobber_flag"])]
    pub interactive_flag: bool,

    /// Do not overwrite an existing file
    #[arg(short = 'n', long = "no-clobber", overrides_with_all = ["force_flag", "interactive_flag"])]
    pub no_clobber_flag: bool,

    /// Copy directories recursively
    #[arg(short = 'r', short_alias = 'R', long = "recursive")]
    pub recursive: bool,

    /// Copy all SOURCE arguments into DIRECTORY
    #[arg(short = 't', long = "target-directory")]
    pub target_directory: Option<String>,

    /// Treat DEST as a normal file
    #[arg(short = 'T', long = "no-target-directory")]
    pub no_target_directory: bool,

    /// Copy only when the SOURCE file is newer
    #[arg(short = 'u', long = "update")]
    pub update: bool,

    /// Explain what is being done
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Source and destination paths
    pub paths: Vec<String>,
}

impl CpConfig {
    pub fn overwrite(&self) -> OverwriteMode {
        if self.no_clobber_flag {
            OverwriteMode::NoClobber
        } else if self.interactive_flag {
            OverwriteMode::Interactive
        } else {
            OverwriteMode::Force
        }
    }
}
