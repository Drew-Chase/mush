use std::process::ExitCode;

use id::cli::IdConfig;
use id::ops::execute;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = IdConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

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
