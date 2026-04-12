use std::process::ExitCode;

use clap::Parser;

use seq::cli::SeqConfig;
use seq::ops::run_seq;

fn main() -> ExitCode {
    let mut config = SeqConfig::parse();

    if let Err(e) = config.resolve() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    match run_seq(&config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("seq: {e}");
            ExitCode::FAILURE
        }
    }
}
