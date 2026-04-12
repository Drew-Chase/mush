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
    name = "mv",
    about = "Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY.",
    version,
    disable_help_flag = true
)]
pub struct MvConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Do not prompt before overwriting
    #[arg(short = 'f', long = "force", overrides_with_all = ["interactive", "no_clobber"])]
    pub force: bool,

    /// Prompt before overwrite
    #[arg(short = 'i', long = "interactive", overrides_with_all = ["force", "no_clobber"])]
    pub interactive: bool,

    /// Do not overwrite an existing file
    #[arg(short = 'n', long = "no-clobber", overrides_with_all = ["force", "interactive"])]
    pub no_clobber: bool,

    /// Move only when the SOURCE file is newer
    #[arg(short = 'u', long = "update")]
    pub update: bool,

    /// Explain what is being done
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Remove any trailing slashes from each SOURCE
    #[arg(long = "strip-trailing-slashes")]
    pub strip_trailing_slashes: bool,

    /// Move all SOURCE arguments into DIRECTORY
    #[arg(short = 't', long = "target-directory")]
    pub target_directory: Option<String>,

    /// Treat DEST as a normal file
    #[arg(short = 'T', long = "no-target-directory")]
    pub no_target_directory: bool,

    /// Source and destination paths
    #[arg()]
    pub paths: Vec<String>,

    #[arg(skip)]
    pub overwrite: OverwriteMode,
}

impl MvConfig {
    /// Resolve the overwrite mode from mutually-exclusive flags.
    /// Must be called after parsing.
    pub fn resolve(mut self) -> Self {
        // clap's overrides_with means only the last-specified flag survives.
        // We check which flag is set to determine the mode.
        if self.no_clobber {
            self.overwrite = OverwriteMode::NoClobber;
        } else if self.interactive {
            self.overwrite = OverwriteMode::Interactive;
        } else {
            self.overwrite = OverwriteMode::Force;
        }
        self
    }
}
