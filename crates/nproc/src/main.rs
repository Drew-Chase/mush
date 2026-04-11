use std::process::ExitCode;

use nproc::cli::NprocConfig;
use nproc::ops::nproc;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = NprocConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    println!("{}", nproc(&config));
    ExitCode::SUCCESS
}
