use std::io;
use std::process::ExitCode;

use clap::Parser;

use tr::cli::TrConfig;
use tr::ops;

fn main() -> ExitCode {
    let config = TrConfig::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    if let Err(e) = ops::translate(&mut input, &mut output, &config) {
        eprintln!("tr: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
