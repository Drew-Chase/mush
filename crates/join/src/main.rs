use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use join::cli::JoinConfig;
use join::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = JoinConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdin = io::stdin();
    let mut input1: Box<dyn io::Read> = if config.file1 == "-" {
        Box::new(stdin.lock())
    } else {
        match File::open(&config.file1) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("join: {}: {e}", config.file1);
                return ExitCode::FAILURE;
            }
        }
    };

    let mut input2: Box<dyn io::Read> = if config.file2 == "-" {
        Box::new(stdin.lock())
    } else {
        match File::open(&config.file2) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("join: {}: {e}", config.file2);
                return ExitCode::FAILURE;
            }
        }
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = ops::join(&mut input1, &mut input2, &mut out, &config) {
        eprintln!("join: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
