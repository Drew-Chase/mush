use std::process::ExitCode;

use file::cli::FileConfig;
use file::ops::run;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = FileConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if let Err(_e) = run(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
