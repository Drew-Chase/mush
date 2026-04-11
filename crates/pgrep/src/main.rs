use std::process::ExitCode;

use pgrep::cli::PgrepConfig;
use pgrep::ops::{find_processes, format_output};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = PgrepConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.pattern.is_empty() {
        return ExitCode::from(2);
    }

    let matches = find_processes(&config);

    if matches.is_empty() {
        return ExitCode::from(1);
    }

    let output = format_output(&matches, &config);
    println!("{output}");

    ExitCode::SUCCESS
}
