use std::io;

const SIGNALS: &[(i32, &str)] = &[
    (1, "HUP"),
    (2, "INT"),
    (3, "QUIT"),
    (9, "KILL"),
    (10, "USR1"),
    (12, "USR2"),
    (15, "TERM"),
    (18, "CONT"),
    (19, "STOP"),
];

pub fn kill_process(pid: u32, signal: i32) -> io::Result<()> {
    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(pid as i32, signal) };
        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    #[cfg(windows)]
    {
        // On Windows, use taskkill: /F for forceful (SIGKILL), without /F for graceful
        let mut cmd = std::process::Command::new("taskkill");
        cmd.arg("/PID").arg(pid.to_string());
        if signal == 9 {
            cmd.arg("/F");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::other(
                format!("taskkill failed: {}", stderr.trim()),
            ));
        }
        Ok(())
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (pid, signal);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "kill is not supported on this platform",
        ))
    }
}

pub fn list_signals() -> Vec<(i32, &'static str)> {
    SIGNALS.to_vec()
}

pub fn signal_name(num: i32) -> Option<&'static str> {
    SIGNALS.iter().find(|&&(n, _)| n == num).map(|&(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_signals() {
        let sigs = list_signals();
        assert!(!sigs.is_empty());
        assert!(sigs.contains(&(15, "TERM")));
        assert!(sigs.contains(&(9, "KILL")));
    }

    #[test]
    fn test_signal_name_found() {
        assert_eq!(signal_name(15), Some("TERM"));
        assert_eq!(signal_name(9), Some("KILL"));
        assert_eq!(signal_name(2), Some("INT"));
        assert_eq!(signal_name(1), Some("HUP"));
    }

    #[test]
    fn test_signal_name_not_found() {
        assert_eq!(signal_name(99), None);
    }
}
