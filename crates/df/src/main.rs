use std::process::ExitCode;

use df::cli::DfConfig;
use df::ops::{format_output, get_disks};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = DfConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let disks = get_disks(&config);
    let lines = format_output(&disks, &config);

    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
