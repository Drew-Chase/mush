use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(name = "uptime", about = "Show how long the system has been running", version, disable_help_flag = true)]
pub struct UptimeConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'p', long, help = "Show uptime in pretty format")]
    pub pretty: bool,

    #[arg(short = 's', long, help = "System up since, in yyyy-mm-dd HH:MM:SS format")]
    pub since: bool,
}
