use std::io;

use crate::cli::IdConfig;

#[cfg(unix)]
fn get_uid() -> u32 {
    unsafe { libc::getuid() }
}

#[cfg(unix)]
fn get_gid() -> u32 {
    unsafe { libc::getgid() }
}

#[cfg(unix)]
fn get_euid() -> u32 {
    unsafe { libc::geteuid() }
}

#[cfg(unix)]
fn get_egid() -> u32 {
    unsafe { libc::getegid() }
}

#[cfg(unix)]
fn get_username() -> io::Result<String> {
    std::env::var("USER")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "USER not set"))
}

#[cfg(unix)]
fn get_groups() -> Vec<u32> {
    let mut groups = vec![0u32; 64];
    let n = unsafe { libc::getgroups(groups.len() as i32, groups.as_mut_ptr()) };
    if n >= 0 {
        groups.truncate(n as usize);
    } else {
        groups.clear();
        groups.push(get_gid());
    }
    groups
}

#[cfg(windows)]
fn get_username() -> io::Result<String> {
    std::env::var("USERNAME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "USERNAME not set"))
}

pub fn execute(config: &IdConfig) -> io::Result<String> {
    #[cfg(unix)]
    {
        execute_unix(config)
    }

    #[cfg(windows)]
    {
        execute_windows(config)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = config;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "id: not supported on this platform",
        ))
    }
}

#[cfg(unix)]
fn execute_unix(config: &IdConfig) -> io::Result<String> {
    let uid = if config.real { get_uid() } else { get_euid() };
    let gid = if config.real { get_gid() } else { get_egid() };
    let username = get_username().unwrap_or_default();

    if config.user_only {
        return Ok(if config.name {
            username
        } else {
            uid.to_string()
        });
    }

    if config.group_only {
        return Ok(if config.name {
            // On Unix we'd look up group name; simplified to gid
            gid.to_string()
        } else {
            gid.to_string()
        });
    }

    if config.groups_only {
        let groups = get_groups();
        return Ok(groups
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
            .join(" "));
    }

    Ok(format!("uid={uid}({username}) gid={gid}"))
}

#[cfg(windows)]
fn execute_windows(config: &IdConfig) -> io::Result<String> {
    let username = get_username()?;

    if config.user_only {
        return Ok(if config.name {
            username
        } else {
            "1000".to_string()
        });
    }

    if config.group_only {
        return Ok(if config.name {
            "users".to_string()
        } else {
            "1000".to_string()
        });
    }

    if config.groups_only {
        return Ok("1000".to_string());
    }

    Ok(format!("uid=1000({username}) gid=1000(users)"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_output() {
        let config = IdConfig::default();
        let result = execute(&config).unwrap();
        assert!(result.contains("uid="));
        assert!(result.contains("gid="));
    }

    #[test]
    fn test_user_only() {
        let config = IdConfig {
            user_only: true,
            ..Default::default()
        };
        let result = execute(&config).unwrap();
        // Should be just a number or name, no "uid="
        assert!(!result.contains("uid="));
    }

    #[test]
    fn test_user_only_name() {
        let config = IdConfig {
            user_only: true,
            name: true,
            ..Default::default()
        };
        let result = execute(&config).unwrap();
        assert!(!result.is_empty());
        // Should be a name, not contain "uid="
        assert!(!result.contains("uid="));
    }

    #[test]
    fn test_group_only() {
        let config = IdConfig {
            group_only: true,
            ..Default::default()
        };
        let result = execute(&config).unwrap();
        assert!(!result.contains("gid="));
    }
}
