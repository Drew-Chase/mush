use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "base64",
    about = "Base64 encode or decode FILE, or standard input, to standard output.",
    version,
    disable_help_flag = true
)]
pub struct Base64Config {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Decode data
    #[arg(short = 'd', long = "decode")]
    pub decode: bool,

    /// When decoding, ignore non-alphabet characters
    #[arg(short = 'i', long = "ignore-garbage")]
    pub ignore_garbage: bool,

    /// Wrap encoded lines after COLS character (default 76). Use 0 to disable line wrapping
    #[arg(short = 'w', long = "wrap", default_value_t = 76)]
    pub wrap: usize,

    /// File to read (use - or omit for stdin)
    pub file: Option<String>,
}
