use std::process::ExitCode;

use clap::Parser;

use free::cli::FreeConfig;
use free::ops::format_output;

fn main() -> ExitCode {
    let config = FreeConfig::parse();

    let lines = format_output(&config);
    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
