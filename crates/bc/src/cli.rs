use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "bc",
    about = "An arbitrary precision calculator language.",
    version,
    disable_help_flag = true
)]
pub struct BcConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Define the standard math library
    #[arg(short = 'l')]
    pub math_lib: bool,

    /// Input files
    #[arg()]
    pub files: Vec<String>,
}
