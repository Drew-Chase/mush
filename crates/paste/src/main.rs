use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use paste_cmd::cli::PasteConfig;
use paste_cmd::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = PasteConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let stdin = io::stdin();
    let mut inputs: Vec<Box<dyn io::Read>> = Vec::new();

    for filename in &files {
        if filename == "-" {
            inputs.push(Box::new(stdin.lock()));
        } else {
            match File::open(filename) {
                Ok(f) => inputs.push(Box::new(BufReader::new(f))),
                Err(e) => {
                    eprintln!("paste: {filename}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = ops::paste(&mut inputs, &mut out, &config) {
        eprintln!("paste: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
