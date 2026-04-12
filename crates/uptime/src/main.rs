use std::process::ExitCode;

use uptime::cli::UptimeConfig;
use uptime::ops::format_uptime;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = UptimeConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

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
