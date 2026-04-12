use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use clap::Parser;

use strings_cmd::cli::StringsConfig;
use strings_cmd::ops;

fn main() -> ExitCode {
    let config = StringsConfig::parse();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let stdin = io::stdin();

    for filename in &files {
        let mut input: Box<dyn io::Read> = if filename == "-" {
            Box::new(stdin.lock())
        } else {
            match File::open(filename) {
                Ok(f) => Box::new(BufReader::new(f)),
                Err(e) => {
                    eprintln!("strings: {filename}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        };

        let stdout = io::stdout();
        let mut out = stdout.lock();

        if let Err(e) = ops::strings(&mut input, &mut out, &config) {
            eprintln!("strings: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
