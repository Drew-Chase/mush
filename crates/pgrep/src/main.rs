use std::process::ExitCode;

use clap::Parser;

use pgrep::cli::PgrepConfig;
use pgrep::ops::{find_processes, format_output};

fn main() -> ExitCode {
    let config = PgrepConfig::parse();

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
