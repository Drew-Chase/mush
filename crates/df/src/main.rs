use std::process::ExitCode;

use clap::Parser;

use df::cli::DfConfig;
use df::ops::{format_output, get_disks};

fn main() -> ExitCode {
    let config = DfConfig::parse();

    let disks = get_disks(&config);
    let lines = format_output(&disks, &config);

    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
