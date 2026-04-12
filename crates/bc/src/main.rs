use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use bc::cli::BcConfig;
use bc::ops::{BcState, bc_repl};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = BcConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let mut state = BcState::new(config.math_lib);

    // Process files first
    for path in &config.files {
        match File::open(path) {
            Ok(f) => {
                let stdout = io::stdout();
                let mut out = stdout.lock();
                if let Err(e) = bc_repl(&mut BufReader::new(f), &mut out, &mut state) {
                    eprintln!("bc: {path}: {e}");
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!("bc: {path}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    // If no files, read from stdin
    if config.files.is_empty() {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut out = stdout.lock();
        if let Err(e) = bc_repl(&mut stdin.lock(), &mut out, &mut state) {
            eprintln!("bc: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
