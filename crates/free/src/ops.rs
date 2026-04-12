use sysinfo::System;

use crate::cli::FreeConfig;

pub struct MemInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: u64,
    pub buff_cache: u64,
    pub available: u64,
}

pub struct SwapInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

pub fn get_memory_info() -> (MemInfo, SwapInfo) {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();
    let free = total.saturating_sub(used);
    // sysinfo doesn't expose shared/buff_cache directly on all platforms
    let buff_cache = available.saturating_sub(free);
    let shared = 0;

    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();
    let swap_free = swap_total.saturating_sub(swap_used);

    (
        MemInfo {
            total,
            used,
            free,
            shared,
            buff_cache,
            available,
        },
        SwapInfo {
            total: swap_total,
            used: swap_used,
            free: swap_free,
        },
    )
}

fn convert_value(bytes: u64, config: &FreeConfig) -> u64 {
    if config.bytes {
        return bytes;
    }
    if config.mebi {
        return bytes / (1024 * 1024);
    }
    if config.gibi {
        return bytes / (1024 * 1024 * 1024);
    }
    // default: kibi
    bytes / 1024
}

fn format_human(bytes: u64, si: bool) -> String {
    let base: f64 = if si { 1000.0 } else { 1024.0 };
    let suffixes: &[&str] = if si {
        &["B", "kB", "MB", "GB", "TB", "PB"]
    } else {
        &["B", "Ki", "Mi", "Gi", "Ti", "Pi"]
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
    format!("{value:.1}Pi")
}

fn format_value(bytes: u64, config: &FreeConfig) -> String {
    if config.human {
        format_human(bytes, config.si)
    } else {
        format!("{}", convert_value(bytes, config))
    }
}

pub fn format_output(config: &FreeConfig) -> Vec<String> {
    let (mem, swap) = get_memory_info();
    let mut lines = Vec::new();

    if config.wide {
        lines.push(format!(
            "{:>15} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}",
            "", "total", "used", "free", "shared", "buffers", "available"
        ));
    } else {
        lines.push(format!(
            "{:>15} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}",
            "", "total", "used", "free", "shared", "buff/cache", "available"
        ));
    }

    lines.push(format!(
        "{:<15} {:>12} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "Mem:",
        format_value(mem.total, config),
        format_value(mem.used, config),
        format_value(mem.free, config),
        format_value(mem.shared, config),
        format_value(mem.buff_cache, config),
        format_value(mem.available, config),
    ));

    lines.push(format!(
        "{:<15} {:>12} {:>12} {:>12}",
        "Swap:",
        format_value(swap.total, config),
        format_value(swap.used, config),
        format_value(swap.free, config),
    ));

    if config.total {
        let total_total = mem.total + swap.total;
        let total_used = mem.used + swap.used;
        let total_free = mem.free + swap.free;

        lines.push(format!(
            "{:<15} {:>12} {:>12} {:>12}",
            "Total:",
            format_value(total_total, config),
            format_value(total_used, config),
            format_value(total_free, config),
        ));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_value_bytes() {
        let config = FreeConfig {
            bytes: true,
            kibi: false,
            ..Default::default()
        };
        assert_eq!(convert_value(1024, &config), 1024);
    }

    #[test]
    fn test_convert_value_kibi() {
        let config = FreeConfig::default();
        assert_eq!(convert_value(1024, &config), 1);
    }

    #[test]
    fn test_convert_value_mebi() {
        let config = FreeConfig {
            kibi: false,
            mebi: true,
            ..Default::default()
        };
        assert_eq!(convert_value(1_048_576, &config), 1);
    }

    #[test]
    fn test_convert_value_gibi() {
        let config = FreeConfig {
            kibi: false,
            gibi: true,
            ..Default::default()
        };
        assert_eq!(convert_value(1_073_741_824, &config), 1);
    }

    #[test]
    fn test_format_human_bytes() {
        assert_eq!(format_human(500, false), "500B");
    }

    #[test]
    fn test_format_human_kibi() {
        assert_eq!(format_human(1024, false), "1.0Ki");
    }

    #[test]
    fn test_format_human_mebi() {
        assert_eq!(format_human(1_048_576, false), "1.0Mi");
    }

    #[test]
    fn test_format_human_si() {
        assert_eq!(format_human(1000, true), "1.0kB");
        assert_eq!(format_human(1_000_000, true), "1.0MB");
    }

    #[test]
    fn test_format_output_has_header() {
        let config = FreeConfig::default();
        let lines = format_output(&config);
        assert!(lines[0].contains("total"));
        assert!(lines[0].contains("used"));
        assert!(lines[0].contains("free"));
    }

    #[test]
    fn test_format_output_has_mem_and_swap() {
        let config = FreeConfig::default();
        let lines = format_output(&config);
        assert!(lines.len() >= 3);
        assert!(lines[1].starts_with("Mem:"));
        assert!(lines[2].starts_with("Swap:"));
    }

    #[test]
    fn test_format_output_with_total() {
        let config = FreeConfig {
            total: true,
            ..Default::default()
        };
        let lines = format_output(&config);
        assert!(lines.len() >= 4);
        assert!(lines[3].starts_with("Total:"));
    }
}
