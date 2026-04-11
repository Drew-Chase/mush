use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::cli::TreeConfig;

pub struct TreeStats {
    pub dirs: usize,
    pub files: usize,
}

pub fn print_tree(
    path: &Path,
    config: &TreeConfig,
    writer: &mut dyn Write,
) -> io::Result<TreeStats> {
    if config.json {
        return print_tree_json(path, config, writer);
    }

    let mut stats = TreeStats { dirs: 0, files: 0 };

    let root_name = if config.full_path {
        path.to_string_lossy().to_string()
    } else {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string())
    };

    if config.color {
        writeln!(writer, "\x1b[1;34m{root_name}\x1b[0m")?;
    } else {
        writeln!(writer, "{root_name}")?;
    }

    print_tree_recursive(path, "", config, 1, &mut stats, writer)?;

    if !config.no_report {
        writeln!(writer, "\n{} directories, {} files", stats.dirs, stats.files)?;
    }

    Ok(stats)
}

fn print_tree_recursive(
    path: &Path,
    prefix: &str,
    config: &TreeConfig,
    depth: usize,
    stats: &mut TreeStats,
    writer: &mut dyn Write,
) -> io::Result<()> {
    if let Some(max_level) = config.level
        && depth > max_level
    {
        return Ok(());
    }

    let mut entries: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    // Sort entries alphabetically
    entries.sort_by(|a, b| {
        a.file_name()
            .to_string_lossy()
            .to_lowercase()
            .cmp(&b.file_name().to_string_lossy().to_lowercase())
    });

    // Sort dirs first if configured
    if config.dirs_first {
        entries.sort_by(|a, b| {
            let a_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            b_dir.cmp(&a_dir)
        });
    }

    // Filter entries
    let entries: Vec<fs::DirEntry> = entries
        .into_iter()
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();

            // Hidden files
            if !config.all && name.starts_with('.') {
                return false;
            }

            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            // Dirs only
            if config.dirs_only && !is_dir {
                return false;
            }

            // Exclude pattern (simple substring match)
            if let Some(ref exclude) = config.exclude
                && simple_match(&name, exclude)
            {
                return false;
            }

            // Include pattern (simple substring match) - only applies to files
            if let Some(ref pattern) = config.pattern
                && !is_dir && !simple_match(&name, pattern)
            {
                return false;
            }

            true
        })
        .collect();

    let count = entries.len();

    for (idx, entry) in entries.iter().enumerate() {
        let is_last = idx == count - 1;
        let connector = if is_last { "\u{2514}\u{2500}\u{2500} " } else { "\u{251c}\u{2500}\u{2500} " };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}\u{2502}   ")
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let entry_path = entry.path();

        let display_name = if config.full_path {
            entry_path.to_string_lossy().to_string()
        } else {
            name.clone()
        };

        // Build extra info
        let mut extras = String::new();

        if config.show_size
            && let Ok(meta) = fs::metadata(&entry_path)
        {
            let size = meta.len();
            let size_str = if config.human_readable {
                human_readable(size)
            } else {
                size.to_string()
            };
            extras.push_str(&format!("[{size_str}]  "));
        }

        if config.show_date
            && let Ok(meta) = fs::metadata(&entry_path)
            && let Ok(modified) = meta.modified()
        {
            let formatted = format_system_time(modified);
            extras.push_str(&format!("[{formatted}]  "));
        }

        if is_dir {
            stats.dirs += 1;
            if config.color {
                writeln!(writer, "{prefix}{connector}{extras}\x1b[1;34m{display_name}\x1b[0m")?;
            } else {
                writeln!(writer, "{prefix}{connector}{extras}{display_name}")?;
            }
            print_tree_recursive(&entry_path, &child_prefix, config, depth + 1, stats, writer)?;
        } else {
            stats.files += 1;
            writeln!(writer, "{prefix}{connector}{extras}{display_name}")?;
        }
    }

    Ok(())
}

fn simple_match(name: &str, pattern: &str) -> bool {
    // Support simple glob: * matches anything
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let (prefix, suffix) = (parts[0], parts[1]);
            return name.starts_with(prefix) && name.ends_with(suffix);
        }
        // Fallback: substring
        let stripped = pattern.replace('*', "");
        return name.contains(&stripped);
    }
    name.contains(pattern)
}

fn human_readable(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes}");
    }
    let units = ['K', 'M', 'G', 'T', 'P'];
    let mut size = bytes as f64;
    for unit in &units {
        size /= 1024.0;
        if size < 1024.0 {
            if size < 10.0 {
                return format!("{:.1}{unit}", size);
            }
            return format!("{:.0}{unit}", size);
        }
    }
    let unit = units[units.len() - 1];
    format!("{:.0}{unit}", size)
}

fn format_system_time(time: std::time::SystemTime) -> String {
    match time.duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            // Simple date formatting: YYYY-MM-DD HH:MM
            let days = secs / 86400;
            let time_of_day = secs % 86400;
            let hours = time_of_day / 3600;
            let minutes = (time_of_day % 3600) / 60;

            // Calculate year/month/day from days since epoch
            let (year, month, day) = days_to_date(days);
            format!("{year:04}-{month:02}-{day:02} {hours:02}:{minutes:02}")
        }
        Err(_) => "unknown".to_string(),
    }
}

fn days_to_date(days: u64) -> (u64, u64, u64) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

// JSON output
fn print_tree_json(
    path: &Path,
    config: &TreeConfig,
    writer: &mut dyn Write,
) -> io::Result<TreeStats> {
    let mut stats = TreeStats { dirs: 0, files: 0 };
    let json = build_json_node(path, config, 1, &mut stats)?;
    writeln!(writer, "{json}")?;
    Ok(stats)
}

fn build_json_node(
    path: &Path,
    config: &TreeConfig,
    depth: usize,
    stats: &mut TreeStats,
) -> io::Result<String> {
    let meta = fs::metadata(path)?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let escaped_name = json_escape(&name);

    if meta.is_file() {
        stats.files += 1;
        let mut obj = format!("{{\"type\":\"file\",\"name\":\"{escaped_name}\"");
        if config.show_size {
            obj.push_str(&format!(",\"size\":{}", meta.len()));
        }
        obj.push('}');
        return Ok(obj);
    }

    if !meta.is_dir() {
        return Ok(format!("{{\"type\":\"file\",\"name\":\"{escaped_name}\"}}"));
    }

    let at_max_level = config.level.is_some_and(|max| depth > max);

    let mut children = Vec::new();

    if !at_max_level {
        let mut entries: Vec<fs::DirEntry> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .collect();

        entries.sort_by(|a, b| {
            a.file_name()
                .to_string_lossy()
                .to_lowercase()
                .cmp(&b.file_name().to_string_lossy().to_lowercase())
        });

        if config.dirs_first {
            entries.sort_by(|a, b| {
                let a_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let b_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
                b_dir.cmp(&a_dir)
            });
        }

        for entry in entries {
            let entry_name = entry.file_name().to_string_lossy().to_string();

            if !config.all && entry_name.starts_with('.') {
                continue;
            }

            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            if config.dirs_only && !is_dir {
                continue;
            }

            if let Some(ref exclude) = config.exclude
                && simple_match(&entry_name, exclude)
            {
                continue;
            }

            if let Some(ref pattern) = config.pattern
                && !is_dir && !simple_match(&entry_name, pattern)
            {
                continue;
            }

            if is_dir {
                stats.dirs += 1;
            }

            let child_json = build_json_node(&entry.path(), config, depth + 1, stats)?;
            children.push(child_json);
        }
    }

    let contents = children.join(",");
    let obj = format!("{{\"type\":\"directory\",\"name\":\"{escaped_name}\",\"contents\":[{contents}]}}");
    Ok(obj)
}

fn json_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}
