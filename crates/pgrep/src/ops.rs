use regex::Regex;
use sysinfo::{ProcessesToUpdate, System};

use crate::cli::PgrepConfig;

pub struct MatchedProcess {
    pub pid: u32,
    pub name: String,
    pub cmd: String,
    pub run_time: u64,
}

pub fn find_processes(config: &PgrepConfig) -> Vec<MatchedProcess> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

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
            eprintln!("pgrep: invalid pattern: {e}");
            return Vec::new();
        }
    };

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
                name,
                cmd,
                run_time: proc_.run_time(),
            })
        })
        .collect();

    matches.sort_by_key(|m| m.pid);

    if config.newest {
        if let Some(m) = matches.iter().max_by_key(|m| m.run_time) {
            // Newest = least run_time (started most recently)
            // Actually run_time is how long it's been running, so newest = smallest run_time
            let _ = m;
        }
        matches.sort_by_key(|m| m.run_time);
        matches.truncate(1);
    } else if config.oldest {
        matches.sort_by_key(|m| std::cmp::Reverse(m.run_time));
        matches.truncate(1);
    }

    matches
}

pub fn format_output(matches: &[MatchedProcess], config: &PgrepConfig) -> String {
    if config.count {
        return matches.len().to_string();
    }

    let parts: Vec<String> = matches
        .iter()
        .map(|m| {
            if config.list_full {
                format!("{} {}", m.pid, m.cmd)
            } else if config.list_name {
                format!("{} {}", m.pid, m.name)
            } else {
                m.pid.to_string()
            }
        })
        .collect();

    parts.join(&config.delimiter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_output_pid_only() {
        let matches = vec![
            MatchedProcess {
                pid: 100,
                name: "foo".to_string(),
                cmd: "/usr/bin/foo --bar".to_string(),
                run_time: 60,
            },
            MatchedProcess {
                pid: 200,
                name: "foo".to_string(),
                cmd: "/usr/bin/foo --baz".to_string(),
                run_time: 30,
            },
        ];
        let config = PgrepConfig {
            pattern: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(format_output(&matches, &config), "100\n200");
    }

    #[test]
    fn test_format_output_list_name() {
        let matches = vec![MatchedProcess {
            pid: 100,
            name: "foo".to_string(),
            cmd: "/usr/bin/foo".to_string(),
            run_time: 60,
        }];
        let config = PgrepConfig {
            list_name: true,
            pattern: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(format_output(&matches, &config), "100 foo");
    }

    #[test]
    fn test_format_output_list_full() {
        let matches = vec![MatchedProcess {
            pid: 100,
            name: "foo".to_string(),
            cmd: "/usr/bin/foo --bar".to_string(),
            run_time: 60,
        }];
        let config = PgrepConfig {
            list_full: true,
            pattern: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(format_output(&matches, &config), "100 /usr/bin/foo --bar");
    }

    #[test]
    fn test_format_output_count() {
        let matches = vec![
            MatchedProcess {
                pid: 100,
                name: "foo".to_string(),
                cmd: String::new(),
                run_time: 60,
            },
            MatchedProcess {
                pid: 200,
                name: "foo".to_string(),
                cmd: String::new(),
                run_time: 30,
            },
        ];
        let config = PgrepConfig {
            count: true,
            pattern: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(format_output(&matches, &config), "2");
    }

    #[test]
    fn test_format_output_custom_delimiter() {
        let matches = vec![
            MatchedProcess {
                pid: 100,
                name: "foo".to_string(),
                cmd: String::new(),
                run_time: 60,
            },
            MatchedProcess {
                pid: 200,
                name: "bar".to_string(),
                cmd: String::new(),
                run_time: 30,
            },
        ];
        let config = PgrepConfig {
            delimiter: ",".to_string(),
            pattern: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(format_output(&matches, &config), "100,200");
    }
}
