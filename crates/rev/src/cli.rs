use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rev",
    about = "Reverse lines characterwise.",
    version,
    disable_help_flag = true
)]
pub struct RevConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    pub files: Vec<String>,
}
