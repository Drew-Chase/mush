use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "tee", about = "Copy standard input to each FILE, and also to standard output", version, disable_help_flag = true)]
pub struct TeeConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', long = "append", help = "Append to the given FILEs, do not overwrite")]
    pub append: bool,

    pub files: Vec<String>,
}
