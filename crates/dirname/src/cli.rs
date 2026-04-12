use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "dirname",
    about = "Output each NAME with its last non-slash component and trailing slashes removed.",
    version,
    disable_help_flag = true
)]
pub struct DirnameConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'z', long = "zero", help = "End each output line with NUL, not newline")]
    pub zero: bool,

    #[arg(trailing_var_arg = true)]
    pub names: Vec<String>,
}
