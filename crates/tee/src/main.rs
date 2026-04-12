use std::io;
use std::process::ExitCode;

use clap::Parser;

use tee::cli::TeeConfig;
use tee::ops::tee;

fn main() -> ExitCode {
    let config = TeeConfig::parse();

    let mut stdin = io::stdin().lock();

    if let Err(e) = tee(&mut stdin, &config.files, config.append) {
        eprintln!("tee: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
