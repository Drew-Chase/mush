use std::process::ExitCode;

use seq::cli::SeqConfig;
use seq::ops::run_seq;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config = match SeqConfig::from_args(&args) {
        Ok(Some(config)) => config,
        Ok(None) => return ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    match run_seq(&config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("seq: {e}");
            ExitCode::FAILURE
        }
    }
}
