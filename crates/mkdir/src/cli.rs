use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "mkdir",
    about = "Create the DIRECTORY(ies), if they do not already exist",
    version,
    disable_help_flag = true
)]
pub struct MkdirConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'p', long = "parents")]
    pub parents: bool,

    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    #[arg(short = 'm', long = "mode", value_parser = parse_mode)]
    pub mode: Option<u32>,

    pub directories: Vec<String>,
}

fn parse_mode(s: &str) -> Result<u32, String> {
    u32::from_str_radix(s, 8).map_err(|e| format!("invalid mode: {e}"))
}
