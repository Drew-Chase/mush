use sysinfo::{ProcessesToUpdate, System};

use crate::cli::PsConfig;

pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub user: String,
    pub cpu: f32,
    pub mem: u64,
    pub time: u64,
    pub name: String,
    pub cmd: String,
}

pub fn list_processes(config: &PsConfig) -> Vec<ProcessInfo> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut procs: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc_)| ProcessInfo {
            pid: pid.as_u32(),
            ppid: proc_.parent().map(|p| p.as_u32()).unwrap_or(0),
            user: proc_
                .user_id()
                .map(|u| u.to_string())
                .unwrap_or_default(),
            cpu: proc_.cpu_usage(),
            mem: proc_.memory(),
            time: proc_.run_time(),
            name: proc_.name().to_string_lossy().to_string(),
            cmd: proc_
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        })
        .collect();

    // Apply filters
    if let Some(ref user) = config.user_filter {
        procs.retain(|p| p.user.contains(user));
    }
    if !config.pid_filter.is_empty() {
        procs.retain(|p| config.pid_filter.contains(&p.pid));
    }
    if let Some(ref cmd) = config.command_filter {
        procs.retain(|p| p.name == *cmd);
    }

    // Sort
    if let Some(ref key) = config.sort_key {
        match key.as_str() {
            "pid" => procs.sort_by_key(|p| p.pid),
            "ppid" => procs.sort_by_key(|p| p.ppid),
            "cpu" => procs.sort_by(|a, b| {
                b.cpu
                    .partial_cmp(&a.cpu)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            "mem" => procs.sort_by_key(|p| std::cmp::Reverse(p.mem)),
            "time" => procs.sort_by_key(|p| std::cmp::Reverse(p.time)),
            "name" => procs.sort_by(|a, b| a.name.cmp(&b.name)),
            _ => procs.sort_by_key(|p| p.pid),
        }
    } else {
        procs.sort_by_key(|p| p.pid);
    }

    procs
}

pub fn format_processes(procs: &[ProcessInfo], config: &PsConfig) -> Vec<String> {
    let mut lines = Vec::new();

    if let Some(ref spec) = config.format_spec {
        let fields: Vec<&str> = spec.split(',').map(|s| s.trim()).collect();
        if !config.no_headers {
            let header: Vec<String> = fields
                .iter()
                .map(|f| format_header(f))
                .collect();
            lines.push(header.join("  "));
        }
        for p in procs {
            let cols: Vec<String> = fields
                .iter()
                .map(|f| format_field(f, p))
                .collect();
            lines.push(cols.join("  "));
        }
    } else if config.long_format {
        if !config.no_headers {
            lines.push(format!(
                "{:>8}  {:>8}  {:>12}  {:>5}  {:>10}  {:>8}  {:<16}  {}",
                "PID", "PPID", "USER", "%CPU", "MEM", "TIME", "NAME", "CMD"
            ));
        }
        for p in procs {
            lines.push(format!(
                "{:>8}  {:>8}  {:>12}  {:>5.1}  {:>10}  {:>8}  {:<16}  {}",
                p.pid,
                p.ppid,
                p.user,
                p.cpu,
                format_mem(p.mem),
                format_time(p.time),
                p.name,
                p.cmd
            ));
        }
    } else if config.full {
        if !config.no_headers {
            lines.push(format!(
                "{:>8}  {:>8}  {:>12}  {}",
                "PID", "PPID", "USER", "CMD"
            ));
        }
        for p in procs {
            let display_cmd = if p.cmd.is_empty() { &p.name } else { &p.cmd };
            lines.push(format!(
                "{:>8}  {:>8}  {:>12}  {}",
                p.pid, p.ppid, p.user, display_cmd
            ));
        }
    } else {
        // Default: PID NAME
        if !config.no_headers {
            lines.push(format!("{:>8}  {}", "PID", "NAME"));
        }
        for p in procs {
            lines.push(format!("{:>8}  {}", p.pid, p.name));
        }
    }

    lines
}

fn format_header(field: &str) -> String {
    match field {
        "pid" => format!("{:>8}", "PID"),
        "ppid" => format!("{:>8}", "PPID"),
        "user" => format!("{:>12}", "USER"),
        "cpu" => format!("{:>5}", "%CPU"),
        "mem" => format!("{:>10}", "MEM"),
        "time" => format!("{:>8}", "TIME"),
        "name" => "NAME".to_string(),
        "cmd" => "CMD".to_string(),
        other => other.to_uppercase(),
    }
}

fn format_field(field: &str, p: &ProcessInfo) -> String {
    match field {
        "pid" => format!("{:>8}", p.pid),
        "ppid" => format!("{:>8}", p.ppid),
        "user" => format!("{:>12}", p.user),
        "cpu" => format!("{:>5.1}", p.cpu),
        "mem" => format!("{:>10}", format_mem(p.mem)),
        "time" => format!("{:>8}", format_time(p.time)),
        "name" => p.name.clone(),
        "cmd" => p.cmd.clone(),
        _ => String::new(),
    }
}

fn format_mem(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1}G", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1}M", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.0}K", bytes as f64 / 1024.0)
    } else {
        format!("{bytes}B")
    }
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes}:{secs:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mem_bytes() {
        assert_eq!(format_mem(500), "500B");
    }

    #[test]
    fn test_format_mem_kilobytes() {
        assert_eq!(format_mem(2048), "2K");
    }

    #[test]
    fn test_format_mem_megabytes() {
        assert_eq!(format_mem(10_485_760), "10.0M");
    }

    #[test]
    fn test_format_mem_gigabytes() {
        assert_eq!(format_mem(2_147_483_648), "2.0G");
    }

    #[test]
    fn test_format_time_seconds() {
        assert_eq!(format_time(45), "0:45");
    }

    #[test]
    fn test_format_time_minutes() {
        assert_eq!(format_time(125), "2:05");
    }

    #[test]
    fn test_format_time_hours() {
        assert_eq!(format_time(3661), "1:01:01");
    }

    #[test]
    fn test_format_processes_default() {
        let procs = vec![ProcessInfo {
            pid: 1,
            ppid: 0,
            user: "root".to_string(),
            cpu: 0.0,
            mem: 1024,
            time: 100,
            name: "init".to_string(),
            cmd: "/sbin/init".to_string(),
        }];
        let config = PsConfig::default();
        let lines = format_processes(&procs, &config);
        assert_eq!(lines.len(), 2); // header + 1 process
        assert!(lines[0].contains("PID"));
        assert!(lines[0].contains("NAME"));
        assert!(lines[1].contains("init"));
    }

    #[test]
    fn test_format_processes_no_headers() {
        let procs = vec![ProcessInfo {
            pid: 1,
            ppid: 0,
            user: "root".to_string(),
            cpu: 0.0,
            mem: 1024,
            time: 100,
            name: "init".to_string(),
            cmd: "/sbin/init".to_string(),
        }];
        let config = PsConfig {
            no_headers: true,
            ..Default::default()
        };
        let lines = format_processes(&procs, &config);
        assert_eq!(lines.len(), 1); // no header
    }

    #[test]
    fn test_format_processes_full() {
        let procs = vec![ProcessInfo {
            pid: 42,
            ppid: 1,
            user: "alice".to_string(),
            cpu: 1.5,
            mem: 2048,
            time: 60,
            name: "bash".to_string(),
            cmd: "/bin/bash".to_string(),
        }];
        let config = PsConfig {
            full: true,
            ..Default::default()
        };
        let lines = format_processes(&procs, &config);
        assert!(lines[0].contains("PPID"));
        assert!(lines[0].contains("USER"));
        assert!(lines[1].contains("alice"));
        assert!(lines[1].contains("/bin/bash"));
    }

    #[test]
    fn test_format_processes_long() {
        let procs = vec![ProcessInfo {
            pid: 42,
            ppid: 1,
            user: "alice".to_string(),
            cpu: 1.5,
            mem: 10_485_760,
            time: 3661,
            name: "bash".to_string(),
            cmd: "/bin/bash".to_string(),
        }];
        let config = PsConfig {
            long_format: true,
            ..Default::default()
        };
        let lines = format_processes(&procs, &config);
        assert!(lines[0].contains("%CPU"));
        assert!(lines[0].contains("MEM"));
        assert!(lines[0].contains("TIME"));
    }

    #[test]
    fn test_format_processes_custom_format() {
        let procs = vec![ProcessInfo {
            pid: 42,
            ppid: 1,
            user: "alice".to_string(),
            cpu: 1.5,
            mem: 2048,
            time: 60,
            name: "bash".to_string(),
            cmd: "/bin/bash".to_string(),
        }];
        let config = PsConfig {
            format_spec: Some("pid,name".to_string()),
            ..Default::default()
        };
        let lines = format_processes(&procs, &config);
        assert!(lines[0].contains("PID"));
        assert!(lines[0].contains("NAME"));
        assert!(lines[1].contains("42"));
        assert!(lines[1].contains("bash"));
    }
}
