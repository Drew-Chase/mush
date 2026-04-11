use std::io;
use std::process::ExitCode;

use xargs::cli::XargsConfig;
use xargs::ops::{build_commands, execute_commands, read_items};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = XargsConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let items = read_items(&mut input, &config);

    if items.is_empty() && config.no_run_if_empty {
        return ExitCode::SUCCESS;
    }

    let commands = build_commands(&items, &config);

    if commands.is_empty() && config.no_run_if_empty {
        return ExitCode::SUCCESS;
    }

    let exit_code = execute_commands(&commands, &config);

    ExitCode::from(exit_code as u8)
}
