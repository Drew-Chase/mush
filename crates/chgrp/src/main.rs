use std::process::ExitCode;

use chgrp::cli::ChgrpConfig;
use chgrp::ops::chgrp;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = ChgrpConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if let Err(_e) = chgrp(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
