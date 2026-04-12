use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq)]
#[command(name = "ps", about = "Report running processes", version, disable_help_flag = true)]
pub struct PsConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'e', short_alias = 'A', help = "Select all processes")]
    pub all: bool,

    #[arg(short = 'f', long = "full", help = "Full format listing")]
    pub full: bool,

    #[arg(short = 'l', long = "long", help = "Long format")]
    pub long_format: bool,

    #[arg(short = 'u', long = "user", help = "Show only processes for USER")]
    pub user_filter: Option<String>,

    #[arg(short = 'p', long = "pid", value_delimiter = ',', help = "Show only specified PIDs (comma-separated)")]
    pub pid_filter: Vec<u32>,

    #[arg(short = 'C', long = "command", help = "Show only processes matching NAME")]
    pub command_filter: Option<String>,

    #[arg(long = "sort", help = "Sort by key: pid, ppid, cpu, mem, time, name")]
    pub sort_key: Option<String>,

    #[arg(short = 'o', long = "format", help = "Custom output format (comma-separated: pid,ppid,name,cpu,mem,time,cmd,user)")]
    pub format_spec: Option<String>,

    #[arg(long, help = "Suppress column headers")]
    pub no_headers: bool,

    #[arg(short = 'a', short_alias = 'x', help = "Show processes for all terminals")]
    pub show_threads: bool,
}
