use std::process::ExitCode;

use clap::Parser;

use id::cli::IdConfig;
use id::ops::execute;

fn main() -> ExitCode {
    let config = IdConfig::parse();

    match execute(&config) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("id: {e}");
            ExitCode::FAILURE
        }
    }
}
