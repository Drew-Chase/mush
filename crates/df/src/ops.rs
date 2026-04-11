use sysinfo::Disks;

use crate::cli::DfConfig;

pub struct DiskInfo {
    pub filesystem: String,
    pub fs_type: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub mount_point: String,
}

pub fn get_disks(config: &DfConfig) -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    let mut result: Vec<DiskInfo> = disks
        .iter()
        .filter_map(|disk| {
            let fs_type = disk.file_system().to_string_lossy().to_string();

            if let Some(ref filter) = config.type_filter
                && !fs_type.eq_ignore_ascii_case(filter)
            {
                return None;
            }

            let total = disk.total_space();
            let available = disk.available_space();

            if !config.all && total == 0 {
                return None;
            }

            Some(DiskInfo {
                filesystem: disk.name().to_string_lossy().to_string(),
                fs_type,
                total,
                used: total.saturating_sub(available),
                available,
                mount_point: disk.mount_point().to_string_lossy().to_string(),
            })
        })
        .collect();

    if !config.files.is_empty() {
        result.retain(|d| {
            config
                .files
                .iter()
                .any(|f| f.starts_with(&d.mount_point) || d.mount_point.starts_with(f))
        });
    }

    result
}

fn format_size(bytes: u64, human: bool, si: bool) -> String {
    if !human && !si {
        // Default: 1K blocks
        return format!("{}", bytes / 1024);
    }

    let (base, suffixes): (f64, &[&str]) = if si {
        (1000.0, &["B", "kB", "MB", "GB", "TB", "PB"])
    } else {
        (1024.0, &["B", "K", "M", "G", "T", "P"])
    };

    let mut value = bytes as f64;
    for &suffix in suffixes {
        if value < base {
            return if suffix == "B" {
                format!("{value:.0}{suffix}")
            } else {
                format!("{value:.1}{suffix}")
            };
        }
        value /= base;
    }
    format!("{value:.1}P")
}

fn use_percent(used: u64, total: u64) -> String {
    if total == 0 {
        return "-".to_string();
    }
    let pct = (used as f64 / total as f64 * 100.0).round() as u64;
    format!("{pct}%")
}

pub fn format_output(disks: &[DiskInfo], config: &DfConfig) -> Vec<String> {
    let mut lines = Vec::new();

    // Header
    let mut header = String::from("Filesystem");
    if config.print_type {
        header.push_str(&format!("  {:>10}", "Type"));
    }
    let size_label = if config.human_readable || config.si {
        "Size"
    } else {
        "1K-blocks"
    };
    header.push_str(&format!(
        "  {:>10}  {:>10}  {:>10}  {:>5}  {}",
        size_label, "Used", "Avail", "Use%", "Mounted on"
    ));
    lines.push(header);

    let mut total_size: u64 = 0;
    let mut total_used: u64 = 0;
    let mut total_avail: u64 = 0;

    for d in disks {
        let mut line = format!("{:<20}", d.filesystem);
        if config.print_type {
            line.push_str(&format!("  {:>10}", d.fs_type));
        }
        line.push_str(&format!(
            "  {:>10}  {:>10}  {:>10}  {:>5}  {}",
            format_size(d.total, config.human_readable, config.si),
            format_size(d.used, config.human_readable, config.si),
            format_size(d.available, config.human_readable, config.si),
            use_percent(d.used, d.total),
            d.mount_point,
        ));
        lines.push(line);

        total_size += d.total;
        total_used += d.used;
        total_avail += d.available;
    }

    if config.total {
        let mut line = format!("{:<20}", "total");
        if config.print_type {
            line.push_str(&format!("  {:>10}", "-"));
        }
        line.push_str(&format!(
            "  {:>10}  {:>10}  {:>10}  {:>5}  {}",
            format_size(total_size, config.human_readable, config.si),
            format_size(total_used, config.human_readable, config.si),
            format_size(total_avail, config.human_readable, config.si),
            use_percent(total_used, total_size),
            "-",
        ));
        lines.push(line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_default() {
        assert_eq!(format_size(1024, false, false), "1");
        assert_eq!(format_size(1_048_576, false, false), "1024");
    }

    #[test]
    fn test_format_size_human() {
        assert_eq!(format_size(500, true, false), "500B");
        assert_eq!(format_size(1024, true, false), "1.0K");
        assert_eq!(format_size(1_048_576, true, false), "1.0M");
        assert_eq!(format_size(1_073_741_824, true, false), "1.0G");
    }

    #[test]
    fn test_format_size_si() {
        assert_eq!(format_size(500, false, true), "500B");
        assert_eq!(format_size(1000, false, true), "1.0kB");
        assert_eq!(format_size(1_000_000, false, true), "1.0MB");
        assert_eq!(format_size(1_000_000_000, false, true), "1.0GB");
    }

    #[test]
    fn test_use_percent() {
        assert_eq!(use_percent(50, 100), "50%");
        assert_eq!(use_percent(0, 100), "0%");
        assert_eq!(use_percent(100, 100), "100%");
        assert_eq!(use_percent(0, 0), "-");
    }

    #[test]
    fn test_format_output_header() {
        let disks = vec![];
        let config = DfConfig::default();
        let lines = format_output(&disks, &config);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("Filesystem"));
        assert!(lines[0].contains("1K-blocks"));
    }

    #[test]
    fn test_format_output_with_type() {
        let disks = vec![DiskInfo {
            filesystem: "sda1".to_string(),
            fs_type: "ext4".to_string(),
            total: 1_073_741_824,
            used: 536_870_912,
            available: 536_870_912,
            mount_point: "/".to_string(),
        }];
        let config = DfConfig {
            print_type: true,
            ..Default::default()
        };
        let lines = format_output(&disks, &config);
        assert!(lines[0].contains("Type"));
        assert!(lines[1].contains("ext4"));
    }

    #[test]
    fn test_format_output_total() {
        let disks = vec![DiskInfo {
            filesystem: "sda1".to_string(),
            fs_type: "ext4".to_string(),
            total: 1_000_000,
            used: 500_000,
            available: 500_000,
            mount_point: "/".to_string(),
        }];
        let config = DfConfig {
            total: true,
            ..Default::default()
        };
        let lines = format_output(&disks, &config);
        assert_eq!(lines.len(), 3); // header + 1 disk + total
        assert!(lines[2].starts_with("total"));
    }

    #[test]
    fn test_format_output_human_readable_header() {
        let disks = vec![];
        let config = DfConfig {
            human_readable: true,
            ..Default::default()
        };
        let lines = format_output(&disks, &config);
        assert!(lines[0].contains("Size"));
    }
}
