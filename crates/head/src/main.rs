use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use clap::Parser;

use head::cli::HeadConfig;
use head::ops::head;

fn main() -> ExitCode {
    let config = HeadConfig::parse();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let multiple = files.len() > 1;
    let mut stdout = io::stdout().lock();
    let mut exit_code = ExitCode::SUCCESS;

    for (idx, filename) in files.iter().enumerate() {
        let print_header = (multiple && !config.quiet) || config.verbose;
        if print_header {
            if idx > 0 {
                let _ = writeln!(stdout);
            }
            let display = if filename == "-" { "standard input" } else { filename.as_str() };
            let _ = writeln!(stdout, "==> {display} <==");
        }

        if filename == "-" {
            let mut stdin = io::stdin().lock();
            if let Err(e) = head(&mut stdin, &config, &mut stdout) {
                eprintln!("head: standard input: {e}");
                exit_code = ExitCode::FAILURE;
            }
        } else {
            match File::open(filename) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    if let Err(e) = head(&mut reader, &config, &mut stdout) {
                        eprintln!("head: {filename}: {e}");
                        exit_code = ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("head: cannot open '{filename}' for reading: {e}");
                    exit_code = ExitCode::FAILURE;
                }
            }
        }
    }

    exit_code
}
