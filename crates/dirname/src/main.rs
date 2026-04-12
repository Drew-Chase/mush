use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use dirname::cli::DirnameConfig;
use dirname::ops::dirname;

fn main() -> ExitCode {
    let config = DirnameConfig::parse();

    if config.names.is_empty() {
        eprintln!("dirname: missing operand");
        return ExitCode::FAILURE;
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let separator = if config.zero { '\0' } else { '\n' };

    for name in &config.names {
        let result = dirname(name);
        let _ = write!(out, "{result}{separator}");
    }

    let _ = out.flush();
    ExitCode::SUCCESS
}
