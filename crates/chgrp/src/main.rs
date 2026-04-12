use std::process::ExitCode;

use clap::Parser;

use chgrp::cli::ChgrpConfig;
use chgrp::ops::chgrp;

fn main() -> ExitCode {
    let config = match ChgrpConfig::parse().resolve() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(_e) = chgrp(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
