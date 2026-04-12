use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use clap::Parser;

use cut::cli::CutConfig;
use cut::ops;

fn main() -> ExitCode {
    let mut config = CutConfig::parse();

    if config.resolve().is_none() {
        return ExitCode::FAILURE;
    }

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut exit_code = ExitCode::SUCCESS;
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for filename in &files {
        if filename == "-" {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            if let Err(e) = ops::cut(&mut input, &mut out, &config) {
                eprintln!("cut: {e}");
                exit_code = ExitCode::FAILURE;
            }
        } else {
            match File::open(filename) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    if let Err(e) = ops::cut(&mut reader, &mut out, &config) {
                        eprintln!("cut: {filename}: {e}");
                        exit_code = ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("cut: {filename}: {e}");
                    exit_code = ExitCode::FAILURE;
                }
            }
        }
    }

    exit_code
}
