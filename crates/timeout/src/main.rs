use std::process::ExitCode;

use timeout::cli::TimeoutConfig;
use timeout::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = TimeoutConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    match ops::run_with_timeout(&config) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("timeout: {e}");
            ExitCode::from(125)
        }
    }
}
