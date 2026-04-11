use crate::cli::UnameConfig;

pub fn get_system_info(config: &UnameConfig) -> String {
    let mut parts = Vec::new();

    if config.kernel_name {
        parts.push(kernel_name());
    }
    if config.nodename {
        parts.push(nodename());
    }
    if config.kernel_release {
        parts.push(kernel_release());
    }
    if config.kernel_version {
        parts.push(kernel_version());
    }
    if config.machine {
        parts.push(machine());
    }
    if config.processor {
        parts.push(processor());
    }
    if config.operating_system {
        parts.push(operating_system());
    }

    parts.join(" ")
}

fn kernel_name() -> String {
    match std::env::consts::OS {
        "linux" => "Linux",
        "macos" => "Darwin",
        "windows" => "Windows_NT",
        "freebsd" => "FreeBSD",
        "openbsd" => "OpenBSD",
        "netbsd" => "NetBSD",
        other => other,
    }
    .to_string()
}

fn nodename() -> String {
    #[cfg(windows)]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
    }
}

fn kernel_release() -> String {
    "unknown".to_string()
}

fn kernel_version() -> String {
    "unknown".to_string()
}

fn machine() -> String {
    std::env::consts::ARCH.to_string()
}

fn processor() -> String {
    std::env::consts::ARCH.to_string()
}

fn operating_system() -> String {
    std::env::consts::OS.to_string()
}
