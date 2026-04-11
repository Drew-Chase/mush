use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process::ExitCode;

use uniq::cli::UniqConfig;
use uniq::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = UniqConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdin = io::stdin();
    let mut input: Box<dyn io::Read> = match &config.input {
        Some(f) if f != "-" => match File::open(f) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                eprintln!("uniq: {f}: {e}");
                return ExitCode::FAILURE;
            }
        },
        _ => Box::new(stdin.lock()),
    };

    let stdout = io::stdout();
    let mut output: Box<dyn io::Write> = match &config.output {
        Some(f) => match File::create(f) {
            Ok(file) => Box::new(BufWriter::new(file)),
            Err(e) => {
                eprintln!("uniq: {f}: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => Box::new(stdout.lock()),
    };

    if let Err(e) = ops::uniq(&mut input, &mut output, &config) {
        eprintln!("uniq: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
