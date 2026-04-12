use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use clap::Parser;

use rev::cli::RevConfig;
use rev::ops::rev_stream;

fn main() -> ExitCode {
    let config = RevConfig::parse();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut exit_code = ExitCode::SUCCESS;

    for filename in &files {
        if filename == "-" {
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            if let Err(e) = rev_stream(&mut reader, &mut out) {
                eprintln!("rev: {e}");
                exit_code = ExitCode::FAILURE;
            }
        } else {
            match File::open(filename) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    if let Err(e) = rev_stream(&mut reader, &mut out) {
                        eprintln!("rev: {filename}: {e}");
                        exit_code = ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("rev: {filename}: {e}");
                    exit_code = ExitCode::FAILURE;
                }
            }
        }
    }

    exit_code
}
