use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "date",
    about = "Display the current time in the given FORMAT, or set the system date.",
    version,
    disable_help_flag = true
)]
pub struct DateConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Display time described by STRING, not 'now'
    #[arg(short = 'd', long = "date")]
    pub date_string: Option<String>,

    /// Output date/time in ISO 8601 format
    #[arg(short = 'I', long = "iso-8601")]
    pub iso_format: Option<String>,

    /// Output date and time in RFC 5322 format
    #[arg(short = 'R', long = "rfc-email")]
    pub rfc_email: bool,

    /// Output date/time in RFC 3339 format
    #[arg(long = "rfc-3339")]
    pub rfc_3339: Option<String>,

    /// Display the last modification time of FILE
    #[arg(short = 'r', long = "reference")]
    pub reference: Option<String>,

    /// Print or set Coordinated Universal Time (UTC)
    #[arg(short = 'u', long = "utc", aliases = ["universal"])]
    pub utc: bool,

    /// Output format string (e.g. +%Y-%m-%d)
    #[arg(skip)]
    pub format: Option<String>,
}

impl DateConfig {
    /// Post-process: extract +FORMAT from trailing args since clap can't handle the +prefix syntax.
    /// Call this after parsing to handle the format argument.
    pub fn resolve(mut self, raw_args: &[String]) -> Self {
        for arg in raw_args {
            if let Some(fmt) = arg.strip_prefix('+') {
                self.format = Some(fmt.to_string());
            }
        }
        self
    }
}
