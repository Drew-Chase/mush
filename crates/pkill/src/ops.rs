use std::io;

use regex::Regex;
use sysinfo::{ProcessesToUpdate, System};

use crate::cli::PkillConfig;

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

struct MatchedProcess {
    pid: u32,
    run_time: u64,
}

fn parse_signal(sig: &str) -> Option<i32> {
    // Try numeric
    if let Ok(n) = sig.parse::<i32>() {
        return Some(n);
    }
    // Try name (with or without SIG prefix)
    let name = sig.strip_prefix("SIG").unwrap_or(sig);
    SIGNALS
        .iter()
        .find(|&&(_, n)| n.eq_ignore_ascii_case(name))
        .map(|&(num, _)| num)
}

fn kill_process(pid: u32, signal: i32) -> io::Result<()> {
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
        let mut cmd = std::process::Command::new("taskkill");
        cmd.arg("/PID").arg(pid.to_string());
        if signal == 9 {
            cmd.arg("/F");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(io::Error::other(format!(
                "taskkill failed: {}",
                stderr.trim()
            )));
        }
        Ok(())
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (pid, signal);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "pkill is not supported on this platform",
        ))
    }
}

pub fn pkill(config: &PkillConfig) -> i32 {
    let signal_num = match parse_signal(&config.signal) {
        Some(n) => n,
        None => {
            eprintln!("pkill: unknown signal '{}'", config.signal);
            return 2;
        }
    };

    let pattern_str = if config.exact {
        format!("^{}$", regex::escape(&config.pattern))
    } else {
        config.pattern.clone()
    };

    let re = if config.ignore_case {
        Regex::new(&format!("(?i){pattern_str}"))
    } else {
        Regex::new(&pattern_str)
    };

    let re = match re {
        Ok(r) => r,
        Err(e) => {
            eprintln!("pkill: invalid pattern: {e}");
            return 2;
        }
    };

    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let my_pid = std::process::id();

    let mut matches: Vec<MatchedProcess> = sys
        .processes()
        .iter()
        .filter_map(|(pid, proc_)| {
            let pid_u32 = pid.as_u32();
            if pid_u32 == my_pid {
                return None;
            }

            let name = proc_.name().to_string_lossy().to_string();
            let cmd: String = proc_
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" ");

            let match_target = if config.full { &cmd } else { &name };

            if !re.is_match(match_target) {
                return None;
            }

            if let Some(ref user) = config.user_filter {
                let proc_user = proc_
                    .user_id()
                    .map(|u| u.to_string())
                    .unwrap_or_default();
                if !proc_user.contains(user.as_str()) {
                    return None;
                }
            }

            Some(MatchedProcess {
                pid: pid_u32,
                run_time: proc_.run_time(),
            })
        })
        .collect();

    matches.sort_by_key(|m| m.pid);

    if config.newest {
        matches.sort_by_key(|m| m.run_time);
        matches.truncate(1);
    } else if config.oldest {
        matches.sort_by_key(|m| std::cmp::Reverse(m.run_time));
        matches.truncate(1);
    }

    if matches.is_empty() {
        return 1;
    }

    let mut had_error = false;
    for m in &matches {
        if let Err(e) = kill_process(m.pid, signal_num) {
            eprintln!("pkill: killing pid {}: {e}", m.pid);
            had_error = true;
        }
    }

    if had_error { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signal_numeric() {
        assert_eq!(parse_signal("9"), Some(9));
        assert_eq!(parse_signal("15"), Some(15));
    }

    #[test]
    fn test_parse_signal_name() {
        assert_eq!(parse_signal("TERM"), Some(15));
        assert_eq!(parse_signal("KILL"), Some(9));
        assert_eq!(parse_signal("HUP"), Some(1));
    }

    #[test]
    fn test_parse_signal_with_sig_prefix() {
        assert_eq!(parse_signal("SIGTERM"), Some(15));
        assert_eq!(parse_signal("SIGKILL"), Some(9));
    }

    #[test]
    fn test_parse_signal_case_insensitive() {
        assert_eq!(parse_signal("term"), Some(15));
        assert_eq!(parse_signal("kill"), Some(9));
    }

    #[test]
    fn test_parse_signal_unknown() {
        assert_eq!(parse_signal("BOGUS"), None);
    }
}
