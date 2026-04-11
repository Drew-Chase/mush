use std::fs::File;
use std::io::{self, Read, Write};
use std::process::ExitCode;

use base64::cli::Base64Config;
use base64::ops::{decode, encode};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = Base64Config::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let mut input = Vec::new();

    let file = config.file.as_deref().unwrap_or("-");
    if file == "-" {
        if let Err(e) = io::stdin().lock().read_to_end(&mut input) {
            eprintln!("base64: -: {e}");
            return ExitCode::FAILURE;
        }
    } else {
        match File::open(file) {
            Ok(mut f) => {
                if let Err(e) = f.read_to_end(&mut input) {
                    eprintln!("base64: {file}: {e}");
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!("base64: {file}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    if config.decode {
        let input_str = String::from_utf8_lossy(&input);
        match decode(&input_str, config.ignore_garbage) {
            Ok(data) => {
                if let Err(e) = io::stdout().write_all(&data) {
                    eprintln!("base64: write error: {e}");
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!("base64: invalid input: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let encoded = encode(&input, config.wrap);
        println!("{encoded}");
    }

    ExitCode::SUCCESS
}
