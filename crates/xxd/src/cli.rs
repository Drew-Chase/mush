use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "xxd",
    about = "Make a hex dump of a file or stdin.",
    version,
    disable_help_flag = true
)]
pub struct XxdConfig {
    #[arg(long = "help", short = 'h', action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Number of octets per line
    #[arg(short = 'c', default_value_t = 16)]
    pub cols: usize,

    /// Group size in bytes
    #[arg(short = 'g', default_value_t = 2)]
    pub group_size: usize,

    /// Stop after LEN octets
    #[arg(short = 'l')]
    pub length: Option<usize>,

    /// Start at SEEK bytes offset
    #[arg(short = 's', default_value_t = 0)]
    pub seek: usize,

    /// Use upper case hex letters
    #[arg(short = 'u')]
    pub upper: bool,

    /// Output in plain hex dump style
    #[arg(short = 'p')]
    pub plain: bool,

    /// Reverse: convert hex dump to binary
    #[arg(short = 'r')]
    pub reverse: bool,

    /// Output in C include file style
    #[arg(short = 'i')]
    pub include: bool,

    /// Binary digit dump
    #[arg(short = 'b')]
    pub bits: bool,

    /// File to read
    pub file: Option<String>,
}

impl Default for XxdConfig {
    fn default() -> Self {
        Self {
            help: None,
            cols: 16,
            group_size: 2,
            length: None,
            seek: 0,
            upper: false,
            plain: false,
            reverse: false,
            include: false,
            bits: false,
            file: None,
        }
    }
}
