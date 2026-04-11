use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::cli::TimeoutConfig;

/// Run a command with a time limit.
///
/// Returns the exit code:
/// - The child's exit code if it finishes in time
/// - 124 if the command timed out (GNU convention)
/// - 137 if killed after kill_after period
pub fn run_with_timeout(config: &TimeoutConfig) -> io::Result<i32> {
    let mut child = Command::new(&config.command[0])
        .args(&config.command[1..])
        .spawn()?;

    let deadline = Instant::now() + Duration::from_secs_f64(config.duration_secs);
    let poll_interval = Duration::from_millis(10);

    // Poll until child exits or deadline passes
    loop {
        match child.try_wait()? {
            Some(status) => {
                return Ok(status.code().unwrap_or(1));
            }
            None => {
                if Instant::now() >= deadline {
                    break;
                }
                std::thread::sleep(poll_interval);
            }
        }
    }

    // Timed out: send the initial signal
    if config.verbose {
        eprintln!("timeout: sending signal {} to command", config.signal);
    }

    terminate_child(&mut child, &config.signal)?;

    // If kill_after is set, wait that duration and then force-kill
    if let Some(kill_after) = config.kill_after {
        let kill_deadline = Instant::now() + Duration::from_secs_f64(kill_after);
        loop {
            match child.try_wait()? {
                Some(status) => {
                    if config.preserve_status {
                        return Ok(status.code().unwrap_or(1));
                    }
                    return Ok(124);
                }
                None => {
                    if Instant::now() >= kill_deadline {
                        break;
                    }
                    std::thread::sleep(poll_interval);
                }
            }
        }

        if config.verbose {
            eprintln!("timeout: sending signal KILL to command");
        }
        let _ = child.kill();
        let _ = child.wait();
        return Ok(if config.preserve_status { 137 } else { 124 });
    }

    // Wait for the child to actually exit after signal
    let status = child.wait()?;

    if config.preserve_status {
        Ok(status.code().unwrap_or(1))
    } else {
        Ok(124)
    }
}

/// Send the configured signal to the child process.
fn terminate_child(child: &mut std::process::Child, signal: &str) -> io::Result<()> {
    // For KILL signal, use the built-in kill method
    if signal.eq_ignore_ascii_case("KILL") || signal == "9" {
        return child.kill();
    }

    // For other signals, on Windows we use taskkill, on Unix we'd use kill
    #[cfg(windows)]
    {
        // On Windows, taskkill is the main mechanism.
        // Without /F it attempts a graceful termination.
        let pid = child.id();
        let force = signal.eq_ignore_ascii_case("KILL") || signal == "9";
        let mut cmd = Command::new("taskkill");
        cmd.arg("/PID").arg(pid.to_string());
        if force {
            cmd.arg("/F");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            // Fall back to child.kill() if taskkill fails
            return child.kill();
        }
        Ok(())
    }

    #[cfg(unix)]
    {
        use std::ffi::c_int;

        let sig_num: c_int = match signal.to_uppercase().as_str() {
            "TERM" | "SIGTERM" => 15,
            "HUP" | "SIGHUP" => 1,
            "INT" | "SIGINT" => 2,
            "QUIT" | "SIGQUIT" => 3,
            "USR1" | "SIGUSR1" => 10,
            "USR2" | "SIGUSR2" => 12,
            _ => signal.parse().unwrap_or(15),
        };

        let pid = child.id() as c_int;
        let ret = unsafe { libc::kill(pid, sig_num) };
        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = signal;
        child.kill()
    }
}
