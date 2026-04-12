use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "hostname",
    about = "Print the system's hostname.",
    version,
    disable_help_flag = true
)]
pub struct HostnameConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 's', long = "short", help = "Print the short hostname (up to the first dot)")]
    pub short: bool,

    #[arg(short = 'f', long = "fqdn", alias = "long", help = "Print the fully qualified domain name")]
    pub fqdn: bool,
}
