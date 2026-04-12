use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use basename::cli::BasenameConfig;
use basename::ops::basename;

fn main() -> ExitCode {
    let mut config = BasenameConfig::parse();
    config.fixup();

    if config.names.is_empty() {
        eprintln!("basename: missing operand");
        return ExitCode::FAILURE;
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let separator = if config.zero { '\0' } else { '\n' };

    for name in &config.names {
        let result = basename(name, config.suffix.as_deref());
        let _ = write!(out, "{result}{separator}");
    }

    let _ = out.flush();
    ExitCode::SUCCESS
}
