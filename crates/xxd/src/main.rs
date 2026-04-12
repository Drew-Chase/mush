use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use xxd::cli::XxdConfig;
use xxd::ops::{xxd_hex_dump, xxd_reverse};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = XxdConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if config.reverse {
        let result = if let Some(ref path) = config.file {
            match File::open(path) {
                Ok(f) => xxd_reverse(&mut BufReader::new(f), &mut out),
                Err(e) => {
                    eprintln!("xxd: {path}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            let stdin = io::stdin();
            xxd_reverse(&mut stdin.lock(), &mut out)
        };
        if let Err(e) = result {
            eprintln!("xxd: {e}");
            return ExitCode::FAILURE;
        }
    } else {
        let result = if let Some(ref path) = config.file {
            match File::open(path) {
                Ok(mut f) => xxd_hex_dump(&mut f, &mut out, &config),
                Err(e) => {
                    eprintln!("xxd: {path}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            xxd_hex_dump(&mut reader, &mut out, &config)
        };
        if let Err(e) = result {
            eprintln!("xxd: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
