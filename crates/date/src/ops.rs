use crate::cli::DateConfig;
use std::fs;
use time::OffsetDateTime;

pub fn get_time(config: &DateConfig) -> Result<OffsetDateTime, String> {
    if let Some(ref path) = config.reference {
        let meta = fs::metadata(path)
            .map_err(|e| format!("date: cannot stat '{path}': {e}"))?;
        let mtime = meta
            .modified()
            .map_err(|e| format!("date: cannot get modification time of '{path}': {e}"))?;
        let dt = OffsetDateTime::from(mtime);
        if config.utc {
            Ok(dt.to_offset(time::UtcOffset::UTC))
        } else {
            Ok(dt)
        }
    } else if let Some(ref date_str) = config.date_string {
        parse_date_string(date_str, config.utc)
    } else if config.utc {
        Ok(OffsetDateTime::now_utc())
    } else {
        OffsetDateTime::now_local()
            .map_err(|e| format!("date: cannot determine local time: {e}"))
    }
}

fn parse_date_string(s: &str, utc: bool) -> Result<OffsetDateTime, String> {
    // @epoch_seconds
    if let Some(epoch_str) = s.strip_prefix('@') {
        let secs: i64 = epoch_str
            .trim()
            .parse()
            .map_err(|_| format!("date: invalid date '@{epoch_str}'"))?;
        let dt = OffsetDateTime::from_unix_timestamp(secs)
            .map_err(|e| format!("date: invalid date '@{epoch_str}': {e}"))?;
        return if utc {
            Ok(dt)
        } else {
            // Epoch timestamps are UTC; try to convert to local
            let local_offset = time::UtcOffset::current_local_offset()
                .unwrap_or(time::UtcOffset::UTC);
            Ok(dt.to_offset(local_offset))
        };
    }

    // "YYYY-MM-DD HH:MM:SS"
    let s = s.trim();
    if let Some((date_part, time_part)) = s.split_once(' ') {
        let date = parse_date_part(date_part)?;
        let time = parse_time_part(time_part)?;
        let offset = if utc {
            time::UtcOffset::UTC
        } else {
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC)
        };
        return Ok(OffsetDateTime::new_in_offset(date, time, offset));
    }

    // "YYYY-MM-DD" only
    if s.contains('-') && s.len() == 10 {
        let date = parse_date_part(s)?;
        let time = time::Time::MIDNIGHT;
        let offset = if utc {
            time::UtcOffset::UTC
        } else {
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC)
        };
        return Ok(OffsetDateTime::new_in_offset(date, time, offset));
    }

    Err(format!("date: invalid date '{s}'"))
}

fn parse_date_part(s: &str) -> Result<time::Date, String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("date: invalid date '{s}'"));
    }
    let year: i32 = parts[0].parse().map_err(|_| format!("date: invalid date '{s}'"))?;
    let month: u8 = parts[1].parse().map_err(|_| format!("date: invalid date '{s}'"))?;
    let day: u8 = parts[2].parse().map_err(|_| format!("date: invalid date '{s}'"))?;
    let month = time::Month::try_from(month)
        .map_err(|_| format!("date: invalid date '{s}'"))?;
    time::Date::from_calendar_date(year, month, day)
        .map_err(|_| format!("date: invalid date '{s}'"))
}

fn parse_time_part(s: &str) -> Result<time::Time, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return Err(format!("date: invalid time '{s}'"));
    }
    let hour: u8 = parts[0].parse().map_err(|_| format!("date: invalid time '{s}'"))?;
    let minute: u8 = parts[1].parse().map_err(|_| format!("date: invalid time '{s}'"))?;
    let second: u8 = if parts.len() == 3 {
        parts[2].parse().map_err(|_| format!("date: invalid time '{s}'"))?
    } else {
        0
    };
    time::Time::from_hms(hour, minute, second)
        .map_err(|_| format!("date: invalid time '{s}'"))
}

pub fn format_time(dt: &OffsetDateTime, config: &DateConfig) -> String {
    if let Some(ref fmt) = config.format {
        return format_custom(dt, fmt);
    }

    if let Some(ref spec) = config.iso_format {
        return format_iso8601(dt, spec);
    }

    if config.rfc_email {
        return format_rfc_email(dt);
    }

    if let Some(ref spec) = config.rfc_3339 {
        return format_rfc3339(dt, spec);
    }

    format_default(dt)
}

fn format_custom(dt: &OffsetDateTime, fmt: &str) -> String {
    let mut result = String::new();
    let mut chars = fmt.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('Y') => result.push_str(&format!("{:04}", dt.year())),
                Some('m') => result.push_str(&format!("{:02}", dt.month() as u8)),
                Some('d') => result.push_str(&format!("{:02}", dt.day())),
                Some('H') => result.push_str(&format!("{:02}", dt.hour())),
                Some('M') => result.push_str(&format!("{:02}", dt.minute())),
                Some('S') => result.push_str(&format!("{:02}", dt.second())),
                Some('A') => result.push_str(weekday_full(dt.weekday())),
                Some('a') => result.push_str(weekday_short(dt.weekday())),
                Some('B') => result.push_str(month_full(dt.month())),
                Some('b') => result.push_str(month_short(dt.month())),
                Some('Z') => result.push_str(&timezone_abbr(dt.offset())),
                Some('z') => result.push_str(&format_offset_hhmm(dt.offset())),
                Some('s') => result.push_str(&dt.unix_timestamp().to_string()),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('%') => result.push('%'),
                Some(other) => {
                    result.push('%');
                    result.push(other);
                }
                None => result.push('%'),
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn format_iso8601(dt: &OffsetDateTime, spec: &str) -> String {
    let offset = format_offset_colon(dt.offset());
    match spec {
        "date" => format!("{:04}-{:02}-{:02}", dt.year(), dt.month() as u8, dt.day()),
        "hours" => format!(
            "{:04}-{:02}-{:02}T{:02}{offset}",
            dt.year(), dt.month() as u8, dt.day(), dt.hour()
        ),
        "minutes" => format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}{offset}",
            dt.year(), dt.month() as u8, dt.day(), dt.hour(), dt.minute()
        ),
        "seconds" => format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{offset}",
            dt.year(), dt.month() as u8, dt.day(), dt.hour(), dt.minute(), dt.second()
        ),
        "ns" => format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02},{:09}{offset}",
            dt.year(), dt.month() as u8, dt.day(),
            dt.hour(), dt.minute(), dt.second(), dt.nanosecond()
        ),
        _ => format!("{:04}-{:02}-{:02}", dt.year(), dt.month() as u8, dt.day()),
    }
}

fn format_rfc_email(dt: &OffsetDateTime) -> String {
    format!(
        "{}, {:02} {} {:04} {:02}:{:02}:{:02} {}",
        weekday_short(dt.weekday()),
        dt.day(),
        month_short(dt.month()),
        dt.year(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        format_offset_hhmm(dt.offset())
    )
}

fn format_rfc3339(dt: &OffsetDateTime, spec: &str) -> String {
    let offset = format_offset_colon(dt.offset());
    match spec {
        "date" => format!("{:04}-{:02}-{:02}", dt.year(), dt.month() as u8, dt.day()),
        "seconds" => format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}{offset}",
            dt.year(), dt.month() as u8, dt.day(),
            dt.hour(), dt.minute(), dt.second()
        ),
        "ns" => format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}{offset}",
            dt.year(), dt.month() as u8, dt.day(),
            dt.hour(), dt.minute(), dt.second(), dt.nanosecond()
        ),
        _ => format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}{offset}",
            dt.year(), dt.month() as u8, dt.day(),
            dt.hour(), dt.minute(), dt.second()
        ),
    }
}

fn format_default(dt: &OffsetDateTime) -> String {
    format!(
        "{} {} {:2} {:02}:{:02}:{:02} {} {:04}",
        weekday_short(dt.weekday()),
        month_short(dt.month()),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        timezone_abbr(dt.offset()),
        dt.year()
    )
}

fn format_offset_hhmm(offset: time::UtcOffset) -> String {
    let (h, m, _) = offset.as_hms();
    let sign = if h < 0 || m < 0 { '-' } else { '+' };
    format!("{sign}{:02}{:02}", h.unsigned_abs(), m.unsigned_abs())
}

fn format_offset_colon(offset: time::UtcOffset) -> String {
    let (h, m, _) = offset.as_hms();
    if h == 0 && m == 0 {
        return "+00:00".to_string();
    }
    let sign = if h < 0 || m < 0 { '-' } else { '+' };
    format!("{sign}{:02}:{:02}", h.unsigned_abs(), m.unsigned_abs())
}

fn timezone_abbr(offset: time::UtcOffset) -> String {
    let (h, m, _) = offset.as_hms();
    if h == 0 && m == 0 {
        return "UTC".to_string();
    }
    // Without OS-level timezone info, fall back to numeric offset
    format_offset_hhmm(offset)
}

fn weekday_full(w: time::Weekday) -> &'static str {
    match w {
        time::Weekday::Monday => "Monday",
        time::Weekday::Tuesday => "Tuesday",
        time::Weekday::Wednesday => "Wednesday",
        time::Weekday::Thursday => "Thursday",
        time::Weekday::Friday => "Friday",
        time::Weekday::Saturday => "Saturday",
        time::Weekday::Sunday => "Sunday",
    }
}

fn weekday_short(w: time::Weekday) -> &'static str {
    match w {
        time::Weekday::Monday => "Mon",
        time::Weekday::Tuesday => "Tue",
        time::Weekday::Wednesday => "Wed",
        time::Weekday::Thursday => "Thu",
        time::Weekday::Friday => "Fri",
        time::Weekday::Saturday => "Sat",
        time::Weekday::Sunday => "Sun",
    }
}

fn month_full(m: time::Month) -> &'static str {
    match m {
        time::Month::January => "January",
        time::Month::February => "February",
        time::Month::March => "March",
        time::Month::April => "April",
        time::Month::May => "May",
        time::Month::June => "June",
        time::Month::July => "July",
        time::Month::August => "August",
        time::Month::September => "September",
        time::Month::October => "October",
        time::Month::November => "November",
        time::Month::December => "December",
    }
}

fn month_short(m: time::Month) -> &'static str {
    match m {
        time::Month::January => "Jan",
        time::Month::February => "Feb",
        time::Month::March => "Mar",
        time::Month::April => "Apr",
        time::Month::May => "May",
        time::Month::June => "Jun",
        time::Month::July => "Jul",
        time::Month::August => "Aug",
        time::Month::September => "Sep",
        time::Month::October => "Oct",
        time::Month::November => "Nov",
        time::Month::December => "Dec",
    }
}
