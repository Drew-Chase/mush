use std::process::ExitCode;

use ps::cli::PsConfig;
use ps::ops::{format_processes, list_processes};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = PsConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let procs = list_processes(&config);
    let lines = format_processes(&procs, &config);

    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
