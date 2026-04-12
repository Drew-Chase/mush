use std::process::ExitCode;

use clap::Parser;

use ps::cli::PsConfig;
use ps::ops::{format_processes, list_processes};

fn main() -> ExitCode {
    let config = PsConfig::parse();

    let procs = list_processes(&config);
    let lines = format_processes(&procs, &config);

    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
