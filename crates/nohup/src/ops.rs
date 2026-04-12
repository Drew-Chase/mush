use std::fs::OpenOptions;
use std::io;
use std::process::{Command, ExitCode, Stdio};

use crate::cli::NohupConfig;

#[cfg(unix)]
fn ignore_sighup() {
    unsafe {
        libc::signal(libc::SIGHUP, libc::SIG_IGN);
    }
}

fn is_terminal_stdout() -> bool {
    // Use std::io::IsTerminal (stabilized in Rust 1.70)
    use std::io::IsTerminal;
    io::stdout().is_terminal()
}

pub fn execute(config: &NohupConfig) -> ExitCode {
    if config.command.is_empty() {
        eprintln!("nohup: missing operand");
        return ExitCode::from(127);
    }

    #[cfg(unix)]
    ignore_sighup();

    let program = &config.command[0];
    let args = &config.command[1..];

    let stdout_cfg = if is_terminal_stdout() {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("nohup.out")
        {
            Ok(file) => {
                eprintln!("nohup: ignoring input and appending output to 'nohup.out'");
                Stdio::from(file)
            }
            Err(e) => {
                eprintln!("nohup: failed to open 'nohup.out': {e}");
                return ExitCode::from(127);
            }
        }
    } else {
        Stdio::inherit()
    };

    match Command::new(program)
        .args(args)
        .stdout(stdout_cfg)
        .stderr(Stdio::inherit())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(mut child) => match child.wait() {
            Ok(status) => ExitCode::from(status.code().unwrap_or(125) as u8),
            Err(e) => {
                eprintln!("nohup: error waiting for child: {e}");
                ExitCode::from(125)
            }
        },
        Err(e) => {
            eprintln!("nohup: failed to run '{program}': {e}");
            // 127 if not found, 126 if not executable
            if e.kind() == io::ErrorKind::NotFound {
                ExitCode::from(127)
            } else {
                ExitCode::from(126)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_command() {
        let config = NohupConfig { command: vec![] };
        let code = execute(&config);
        assert_eq!(code, ExitCode::from(127));
    }

    #[test]
    fn test_is_terminal_check() {
        // In test environment, stdout is typically not a terminal
        // Just verify the function doesn't panic
        let _ = is_terminal_stdout();
    }
}
