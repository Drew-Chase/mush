use std::process::ExitCode;

use clap::Parser;

use date::cli::DateConfig;
use date::ops::{format_time, get_time};

fn main() -> ExitCode {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let config = DateConfig::parse().resolve(&raw_args);

    match get_time(&config) {
        Ok(dt) => {
            let output = format_time(&dt, &config);
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
