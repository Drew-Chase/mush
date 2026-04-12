use std::process::ExitCode;

use mktemp::cli::MktempConfig;
use mktemp::ops::run;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = MktempConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if let Err(e) = run(&config) {
        if !config.quiet {
            eprintln!("mktemp: {e}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
