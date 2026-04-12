use std::io;

use crate::cli::UptimeConfig;

pub fn get_uptime_seconds() -> u64 {
    sysinfo::System::uptime()
}

pub fn format_uptime(config: &UptimeConfig) -> io::Result<String> {
    let total_secs = get_uptime_seconds();

    if config.since {
        return Ok(format_since(total_secs));
    }

    if config.pretty {
        return Ok(format_pretty(total_secs));
    }

    Ok(format_default(total_secs))
}

fn format_default(total_secs: u64) -> String {
    let days = total_secs / 86400;
    let remaining = total_secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    let now = {
        let secs_today = total_secs % 86400;
        let h = secs_today / 3600;
        let m = (secs_today % 3600) / 60;
        let s = secs_today % 60;
        format!("{h:02}:{m:02}:{s:02}")
    };

    let up_part = if days > 0 {
        format!("up {days} day{}, {hours:2}:{minutes:02}", if days == 1 { "" } else { "s" })
    } else {
        format!("up {hours:2}:{minutes:02}")
    };

    format!(" {now} {up_part}")
}

fn format_pretty(total_secs: u64) -> String {
    let days = total_secs / 86400;
    let remaining = total_secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days} day{}", if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{hours} hour{}", if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 || parts.is_empty() {
        parts.push(format!("{minutes} minute{}", if minutes == 1 { "" } else { "s" }));
    }

    format!("up {}", parts.join(", "))
}

fn format_since(total_secs: u64) -> String {
    use std::time::{SystemTime, Duration, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO);
    let boot_epoch = now.as_secs().saturating_sub(total_secs);

    // Simple date formatting without external crates
    let secs = boot_epoch;
    // Days since epoch
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;

    // Convert days since epoch to y-m-d (simplified Gregorian)
    let (year, month, day) = days_to_ymd(days_since_epoch);

    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}")
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    days += 719468;
    let era = days / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_pretty_zero() {
        assert_eq!(format_pretty(0), "up 0 minutes");
    }

    #[test]
    fn test_format_pretty_minutes() {
        assert_eq!(format_pretty(300), "up 5 minutes");
    }

    #[test]
    fn test_format_pretty_hours_and_minutes() {
        assert_eq!(format_pretty(3660), "up 1 hour, 1 minute");
    }

    #[test]
    fn test_format_pretty_days() {
        assert_eq!(format_pretty(90061), "up 1 day, 1 hour, 1 minute");
    }

    #[test]
    fn test_format_pretty_plural() {
        assert_eq!(format_pretty(180000), "up 2 days, 2 hours");
    }

    #[test]
    fn test_format_default_no_days() {
        let output = format_default(3661);
        assert!(output.contains("up  1:01"));
    }

    #[test]
    fn test_format_default_with_days() {
        let output = format_default(90061);
        assert!(output.contains("up 1 day,"));
    }

    #[test]
    fn test_days_to_ymd_epoch() {
        let (y, m, d) = days_to_ymd(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_ymd_known_date() {
        // 2024-01-01 is day 19723 since epoch
        let (y, m, d) = days_to_ymd(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }
}
