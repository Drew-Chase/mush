use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "nproc",
    about = "Print the number of processing units available to the current process.",
    version,
    disable_help_flag = true
)]
pub struct NprocConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(long = "all", help = "Print the number of installed processors")]
    pub all: bool,

    #[arg(long = "ignore", default_value_t = 0, help = "If possible, exclude N processing units")]
    pub ignore: usize,
}
