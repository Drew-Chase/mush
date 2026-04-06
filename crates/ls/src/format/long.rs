use std::io::{self, Write};

use time::OffsetDateTime;

use crate::cli::{ResolvedConfig, TimeField, TimeStyle};
use crate::color::ColorScheme;
use crate::entry::FileEntry;
use crate::platform;

use super::{format_name, format_size};

struct ColumnWidths {
    inode: usize,
    blocks: usize,
    nlinks: usize,
    owner: usize,
    group: usize,
    size: usize,
}

pub fn write(
    entries: &[FileEntry],
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
) -> io::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let sizes: Vec<String> = entries.iter().map(|e| format_size(e.size, config)).collect();
    let widths = calculate_widths(entries, &sizes, config);

    let total_blocks: u64 = entries.iter().filter_map(|e| e.blocks).sum();
    writeln!(out, "total {total_blocks}")?;

    for (entry, size_str) in entries.iter().zip(&sizes) {
        if config.show_inode {
            write!(out, "{:>w$} ", entry.inode.unwrap_or(0), w = widths.inode)?;
        }
        if config.show_blocks {
            write!(out, "{:>w$} ", entry.blocks.unwrap_or(0), w = widths.blocks)?;
        }

        let perms = platform::format_permissions(entry);
        write!(out, "{perms} {:>w$}", entry.nlinks, w = widths.nlinks)?;

        if config.show_owner {
            let owner = entry.owner.as_deref().unwrap_or("-");
            write!(out, " {owner:<w$}", w = widths.owner)?;
        }
        if config.show_group {
            let group = entry.group.as_deref().unwrap_or("-");
            write!(out, " {group:<w$}", w = widths.group)?;
        }

        let time = format_time(entry, config);
        let name = format_name(entry, config, colors);

        writeln!(out, " {size_str:>w$} {time} {name}", w = widths.size)?;
    }

    Ok(())
}

fn calculate_widths(entries: &[FileEntry], sizes: &[String], config: &ResolvedConfig) -> ColumnWidths {
    let mut widths = ColumnWidths {
        inode: 0,
        blocks: 0,
        nlinks: 1,
        owner: 1,
        group: 1,
        size: 1,
    };

    for (entry, size_str) in entries.iter().zip(sizes) {
        if config.show_inode {
            let w = digit_count(entry.inode.unwrap_or(0));
            widths.inode = widths.inode.max(w);
        }
        if config.show_blocks {
            let w = digit_count(entry.blocks.unwrap_or(0));
            widths.blocks = widths.blocks.max(w);
        }
        widths.nlinks = widths.nlinks.max(digit_count(entry.nlinks));
        if config.show_owner {
            widths.owner = widths.owner.max(entry.owner.as_ref().map_or(1, |s| s.len()));
        }
        if config.show_group {
            widths.group = widths.group.max(entry.group.as_ref().map_or(1, |s| s.len()));
        }
        widths.size = widths.size.max(size_str.len());
    }

    widths
}

fn digit_count(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    ((n as f64).log10().floor() as usize) + 1
}

fn format_time(entry: &FileEntry, config: &ResolvedConfig) -> String {
    let system_time = match config.time_field {
        TimeField::Modified => entry.modified,
        TimeField::Accessed => entry.accessed,
        TimeField::Created => entry.created,
    };

    let Some(st) = system_time else {
        return "            ?".to_string();
    };

    let dt: OffsetDateTime = st.into();

    match &config.time_style {
        TimeStyle::FullIso => {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09} {:+03}{:02}",
                dt.year(),
                dt.month() as u8,
                dt.day(),
                dt.hour(),
                dt.minute(),
                dt.second(),
                dt.nanosecond(),
                dt.offset().whole_hours(),
                dt.offset().minutes_past_hour().unsigned_abs(),
            )
        }
        TimeStyle::LongIso => {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                dt.year(),
                dt.month() as u8,
                dt.day(),
                dt.hour(),
                dt.minute(),
            )
        }
        TimeStyle::Iso => {
            format!(
                "{:02}-{:02} {:02}:{:02}",
                dt.month() as u8,
                dt.day(),
                dt.hour(),
                dt.minute(),
            )
        }
        TimeStyle::Default => format_default_time(dt),
        TimeStyle::Custom(fmt) => {
            // Basic strftime-like formatting
            fmt.replace("%Y", &format!("{:04}", dt.year()))
                .replace("%m", &format!("{:02}", dt.month() as u8))
                .replace("%d", &format!("{:02}", dt.day()))
                .replace("%H", &format!("{:02}", dt.hour()))
                .replace("%M", &format!("{:02}", dt.minute()))
                .replace("%S", &format!("{:02}", dt.second()))
        }
    }
}

fn format_default_time(dt: OffsetDateTime) -> String {
    let now = OffsetDateTime::now_utc();
    let six_months = time::Duration::days(183);
    let month = abbreviated_month(dt.month());

    if (now - dt).abs() < six_months {
        format!("{month} {:>2} {:02}:{:02}", dt.day(), dt.hour(), dt.minute())
    } else {
        format!("{month} {:>2}  {:>4}", dt.day(), dt.year())
    }
}

fn abbreviated_month(month: time::Month) -> &'static str {
    match month {
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
