use std::process::ExitCode;

use clap::Parser;

use uptime::cli::UptimeConfig;
use uptime::ops::format_uptime;

fn main() -> ExitCode {
    let config = UptimeConfig::parse();

    match format_uptime(&config) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("uptime: {e}");
            ExitCode::FAILURE
        }
    }
}
