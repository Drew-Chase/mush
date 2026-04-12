use std::process::ExitCode;

use clap::Parser;

use nproc::cli::NprocConfig;
use nproc::ops::nproc;

fn main() -> ExitCode {
    let config = NprocConfig::parse();

    println!("{}", nproc(&config));
    ExitCode::SUCCESS
}
