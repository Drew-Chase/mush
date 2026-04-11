use std::process::ExitCode;

use kill::cli::KillConfig;
use kill::ops::{kill_process, list_signals, signal_name};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = KillConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.table {
        for (num, name) in list_signals() {
            println!("{num:>2} {name}");
        }
        return ExitCode::SUCCESS;
    }

    if config.list {
        // If PIDs were provided, treat them as signal numbers to look up
        if !config.pids.is_empty() {
            for pid in &config.pids {
                if let Some(name) = signal_name(*pid as i32) {
                    println!("{name}");
                } else {
                    eprintln!("kill: unknown signal {pid}");
                }
            }
        } else {
            let sigs: Vec<String> = list_signals().iter().map(|(_, name)| name.to_string()).collect();
            println!("{}", sigs.join(" "));
        }
        return ExitCode::SUCCESS;
    }

    if config.pids.is_empty() {
        eprintln!("kill: missing PID operand");
        eprintln!("Try 'kill --help' for more information.");
        return ExitCode::from(1);
    }

    let mut exit_code = 0u8;

    for &pid in &config.pids {
        if let Err(e) = kill_process(pid, config.signal) {
            eprintln!("kill: ({pid}) - {e}");
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}
