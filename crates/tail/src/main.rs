use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use tail::cli::TailConfig;
use tail::ops::{follow_file, tail_bytes, tail_lines};

fn main() -> ExitCode {
    let config = TailConfig::parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let multiple = files.len() > 1;
    let mut exit_code = 0u8;

    for (idx, file) in files.iter().enumerate() {
        let show_header = (multiple || config.verbose) && !config.quiet;

        if show_header {
            if idx > 0 {
                let _ = writeln!(out);
            }
            if file == "-" {
                let _ = writeln!(out, "==> standard input <==");
            } else {
                let _ = writeln!(out, "==> {file} <==");
            }
        }

        if file == "-" {
            let mut stdin = io::stdin().lock();
            let result = if let Some(n) = config.bytes {
                tail_bytes(&mut stdin, n, &mut out)
            } else {
                tail_lines(&mut stdin, config.lines, &mut out)
            };
            if let Err(e) = result {
                eprintln!("tail: error reading standard input: {e}");
                exit_code = 1;
            }
        } else {
            let path = Path::new(file);
            match File::open(path) {
                Ok(mut f) => {
                    let result = if let Some(n) = config.bytes {
                        tail_bytes(&mut f, n, &mut out)
                    } else {
                        tail_lines(&mut f, config.lines, &mut out)
                    };
                    if let Err(e) = result {
                        eprintln!("tail: error reading '{file}': {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("tail: cannot open '{file}' for reading: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    if config.follow {
        // Follow mode only works with actual files, not stdin
        let follow_files: Vec<&String> = files.iter().filter(|f| *f != "-").collect();
        if let Some(last) = follow_files.last() {
            let path = Path::new(last.as_str());
            if let Err(e) = follow_file(path, &mut out) {
                eprintln!("tail: error following '{last}': {e}");
                exit_code = 1;
            }
        }
    }

    ExitCode::from(exit_code)
}
