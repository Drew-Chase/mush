use std::process::ExitCode;
use date::cli::DateConfig;
use date::ops::{format_time, get_time};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = DateConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

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
