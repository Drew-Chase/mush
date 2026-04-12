use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use readlink::cli::ReadlinkConfig;
use readlink::ops::readlink;

fn main() -> ExitCode {
    let config = ReadlinkConfig::parse();

    if config.files.is_empty() {
        eprintln!("readlink: missing operand");
        return ExitCode::FAILURE;
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut exit_code = ExitCode::SUCCESS;

    for (idx, file) in config.files.iter().enumerate() {
        match readlink(file, &config) {
            Ok(target) => {
                let _ = write!(out, "{target}");
                if config.zero {
                    let _ = write!(out, "\0");
                } else if !config.no_newline || idx + 1 < config.files.len() {
                    let _ = writeln!(out);
                }
            }
            Err(e) => {
                eprintln!("readlink: {e}");
                exit_code = ExitCode::FAILURE;
            }
        }
    }

    let _ = out.flush();
    exit_code
}
