use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "sha256sum",
    about = "Print or check SHA-256 (256-bit) checksums.",
    version,
    disable_help_flag = true
)]
pub struct Sha256sumConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Hash algorithm (default: sha256)
    #[arg(short = 'a', long = "algorithm", default_value = "sha256")]
    pub algorithm: String,

    /// Read in binary mode
    #[arg(short = 'b', long = "binary")]
    pub binary: bool,

    /// Read checksums from the FILEs and check them
    #[arg(short = 'c', long = "check")]
    pub check: bool,

    /// Create a BSD-style checksum
    #[arg(long = "tag")]
    pub tag: bool,

    /// Don't print OK for each successfully verified file
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Don't output anything, status code shows success
    #[arg(long = "status")]
    pub status: bool,

    /// Warn about improperly formatted checksum lines
    #[arg(short = 'w', long = "warn")]
    pub warn: bool,

    /// Files to process
    pub files: Vec<String>,
}

impl Default for Sha256sumConfig {
    fn default() -> Self {
        Self {
            help: None,
            algorithm: "sha256".to_string(),
            binary: false,
            check: false,
            tag: false,
            quiet: false,
            status: false,
            warn: false,
            files: Vec::new(),
        }
    }
}
