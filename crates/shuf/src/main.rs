use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use clap::Parser;

use shuf::cli::ShufConfig;
use shuf::ops::{XorShift64, range_to_lines, read_lines, shuf_lines};

fn main() -> ExitCode {
    let mut config = ShufConfig::parse();

    if let Err(e) = config.resolve() {
        eprintln!("shuf: {e}");
        return ExitCode::FAILURE;
    }

    let mut rng = XorShift64::from_time();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let lines = if let Some((lo, hi)) = config.range {
        range_to_lines(lo, hi)
    } else if config.echo_mode {
        config.echo_args.clone()
    } else if let Some(ref path) = config.file {
        if path == "-" {
            let stdin = io::stdin();
            match read_lines(&mut stdin.lock()) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("shuf: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            match File::open(path) {
                Ok(f) => match read_lines(&mut BufReader::new(f)) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("shuf: {path}: {e}");
                        return ExitCode::FAILURE;
                    }
                },
                Err(e) => {
                    eprintln!("shuf: {path}: {e}");
                    return ExitCode::FAILURE;
                }
            }
        }
    } else {
        let stdin = io::stdin();
        match read_lines(&mut stdin.lock()) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("shuf: {e}");
                return ExitCode::FAILURE;
            }
        }
    };

    if let Err(e) = shuf_lines(&lines, config.head_count, config.repeat, &mut out, &mut rng) {
        eprintln!("shuf: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
