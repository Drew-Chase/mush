use std::io;
use std::path::Path;
use std::process::ExitCode;

use sha256sum::cli::Sha256sumConfig;
use sha256sum::ops::{check_file, format_hash, hash_file, hash_reader};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = Sha256sumConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut exit_code = ExitCode::SUCCESS;

    if config.check {
        for filename in &files {
            let path = Path::new(filename);
            match check_file(path, &config) {
                Ok((_ok, fail)) => {
                    if fail > 0 {
                        if !config.status {
                            eprintln!(
                                "sha256sum: WARNING: {fail} computed checksum did NOT match"
                            );
                        }
                        exit_code = ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("sha256sum: {filename}: {e}");
                    exit_code = ExitCode::FAILURE;
                }
            }
        }
    } else {
        for filename in &files {
            if filename == "-" {
                let mut stdin = io::stdin().lock();
                match hash_reader(&mut stdin) {
                    Ok(hash) => println!("{}", format_hash(&hash, "-", &config)),
                    Err(e) => {
                        eprintln!("sha256sum: -: {e}");
                        exit_code = ExitCode::FAILURE;
                    }
                }
            } else {
                match hash_file(Path::new(filename)) {
                    Ok(hash) => println!("{}", format_hash(&hash, filename, &config)),
                    Err(e) => {
                        eprintln!("sha256sum: {filename}: {e}");
                        exit_code = ExitCode::FAILURE;
                    }
                }
            }
        }
    }

    exit_code
}
