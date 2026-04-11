use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::time::SystemTime;

use filetime::FileTime;

use crate::cli::TouchConfig;

pub fn touch(path: &Path, config: &TouchConfig) -> io::Result<()> {
    let exists = path.exists();

    if !exists && config.no_create {
        return Ok(());
    }

    if !exists {
        File::create(path)?;
    }

    let (new_atime, new_mtime) = determine_times(path, config)?;

    let meta = fs::metadata(path)?;
    let current_atime = FileTime::from_last_access_time(&meta);
    let current_mtime = FileTime::from_last_modification_time(&meta);

    let atime = if config.modify_only && !config.access_only {
        current_atime
    } else {
        new_atime
    };

    let mtime = if config.access_only && !config.modify_only {
        current_mtime
    } else {
        new_mtime
    };

    filetime::set_file_times(path, atime, mtime)?;

    Ok(())
}

fn determine_times(path: &Path, config: &TouchConfig) -> io::Result<(FileTime, FileTime)> {
    if let Some(ref_path) = &config.reference {
        let ref_meta = fs::metadata(ref_path)?;
        let atime = FileTime::from_last_access_time(&ref_meta);
        let mtime = FileTime::from_last_modification_time(&ref_meta);
        return Ok((atime, mtime));
    }

    if let Some(date_str) = &config.date {
        let ft = parse_date(date_str).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("touch: invalid date format '{date_str}'"),
            )
        })?;
        return Ok((ft, ft));
    }

    // Default: use current time
    let _ = path;
    let now = FileTime::from_system_time(SystemTime::now());
    Ok((now, now))
}

/// Parse a date string in common formats.
/// Supports: "YYYY-MM-DD HH:MM:SS", "YYYY-MM-DDTHH:MM:SS", epoch seconds prefixed with @.
fn parse_date(s: &str) -> Option<FileTime> {
    // @epoch format
    if let Some(epoch_str) = s.strip_prefix('@') {
        let secs: i64 = epoch_str.parse().ok()?;
        return Some(FileTime::from_unix_time(secs, 0));
    }

    // Try "YYYY-MM-DD HH:MM:SS" or "YYYY-MM-DDTHH:MM:SS"
    let normalized = s.replace('T', " ");
    let parts: Vec<&str> = normalized.split(' ').collect();
    if parts.len() == 2 {
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if date_parts.len() == 3 && time_parts.len() == 3 {
            let year: i32 = date_parts[0].parse().ok()?;
            let month: u32 = date_parts[1].parse().ok()?;
            let day: u32 = date_parts[2].parse().ok()?;
            let hour: u32 = time_parts[0].parse().ok()?;
            let min: u32 = time_parts[1].parse().ok()?;
            let sec: u32 = time_parts[2].parse().ok()?;

            // Simple epoch calculation (not accounting for all edge cases)
            let epoch = datetime_to_epoch(year, month, day, hour, min, sec)?;
            return Some(FileTime::from_unix_time(epoch, 0));
        }
    }

    // Try "YYYY-MM-DD" only
    let date_parts: Vec<&str> = s.split('-').collect();
    if date_parts.len() == 3 {
        let year: i32 = date_parts[0].parse().ok()?;
        let month: u32 = date_parts[1].parse().ok()?;
        let day: u32 = date_parts[2].parse().ok()?;
        let epoch = datetime_to_epoch(year, month, day, 0, 0, 0)?;
        return Some(FileTime::from_unix_time(epoch, 0));
    }

    None
}

fn datetime_to_epoch(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || min > 59 || sec > 59 {
        return None;
    }

    // Days from year 1970 to the start of `year`
    let mut days: i64 = 0;
    if year >= 1970 {
        for y in 1970..year {
            days += if is_leap(y) { 366 } else { 365 };
        }
    } else {
        for y in year..1970 {
            days -= if is_leap(y) { 366 } else { 365 };
        }
    }

    let month_days = [31, 28 + if is_leap(year) { 1 } else { 0 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for &d in &month_days[..(month - 1) as usize] {
        days += d as i64;
    }
    days += (day - 1) as i64;

    Some(days * 86400 + hour as i64 * 3600 + min as i64 * 60 + sec as i64)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_epoch() {
        let ft = parse_date("@1000000").unwrap();
        assert_eq!(ft, FileTime::from_unix_time(1000000, 0));
    }

    #[test]
    fn test_parse_date_datetime() {
        let ft = parse_date("2000-01-01 00:00:00").unwrap();
        assert_eq!(ft, FileTime::from_unix_time(946684800, 0));
    }

    #[test]
    fn test_parse_date_iso() {
        let ft = parse_date("2000-01-01T00:00:00").unwrap();
        assert_eq!(ft, FileTime::from_unix_time(946684800, 0));
    }

    #[test]
    fn test_parse_date_date_only() {
        let ft = parse_date("2000-01-01").unwrap();
        assert_eq!(ft, FileTime::from_unix_time(946684800, 0));
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("not-a-date").is_none());
    }
}
