use std::io;

use crate::cli::ChownConfig;

/// Parse an OWNER[:GROUP] string into (Option<owner>, Option<group>).
pub fn parse_owner_group(spec: &str) -> (Option<&str>, Option<&str>) {
    if spec.is_empty() {
        return (None, None);
    }
    if let Some(idx) = spec.find(':') {
        let owner = &spec[..idx];
        let group = &spec[idx + 1..];
        (
            if owner.is_empty() { None } else { Some(owner) },
            if group.is_empty() { None } else { Some(group) },
        )
    } else {
        (Some(spec), None)
    }
}

/// Change ownership of files.
#[cfg(unix)]
pub fn chown(config: &ChownConfig) -> io::Result<()> {
    use std::fs;
    use std::path::Path;

    let (owner_name, group_name) = if let Some(ref reference) = config.reference {
        // Get owner/group from reference file
        let ref_path = Path::new(reference);
        let meta = fs::metadata(ref_path)?;
        let uid = get_uid(&meta);
        let gid = get_gid(&meta);
        return chown_files_with_ids(config, uid, gid);
    } else {
        parse_owner_group(&config.owner_group)
    };

    let uid = if let Some(name) = owner_name {
        // Try numeric first
        if let Ok(id) = name.parse::<u32>() {
            id
        } else {
            lookup_uid(name)?
        }
    } else {
        u32::MAX // -1 means don't change
    };

    let gid = if let Some(name) = group_name {
        if let Ok(id) = name.parse::<u32>() {
            id
        } else {
            lookup_gid(name)?
        }
    } else {
        u32::MAX
    };

    chown_files_with_ids(config, uid, gid)
}

#[cfg(unix)]
fn chown_files_with_ids(config: &ChownConfig, uid: u32, gid: u32) -> io::Result<()> {
    use std::path::Path;
    for file in &config.files {
        let path = Path::new(file);
        chown_single(path, uid, gid, config)?;

        if config.recursive && path.is_dir() {
            chown_recursive(path, uid, gid, config)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn chown_single(path: &std::path::Path, uid: u32, gid: u32, config: &ChownConfig) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let ret = if config.no_deref {
        unsafe { libc::lchown(c_path.as_ptr(), uid, gid) }
    } else {
        unsafe { libc::chown(c_path.as_ptr(), uid, gid) }
    };

    if ret != 0 {
        let err = io::Error::last_os_error();
        if !config.quiet {
            eprintln!("chown: changing ownership of '{}': {}", path.display(), err);
        }
        return Err(err);
    }

    if config.verbose {
        eprintln!("changed ownership of '{}'", path.display());
    }

    Ok(())
}

#[cfg(unix)]
fn chown_recursive(dir: &std::path::Path, uid: u32, gid: u32, config: &ChownConfig) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        chown_single(&path, uid, gid, config)?;
        if path.is_dir() {
            chown_recursive(&path, uid, gid, config)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn get_uid(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    meta.uid()
}

#[cfg(unix)]
fn get_gid(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    meta.gid()
}

#[cfg(unix)]
fn lookup_uid(name: &str) -> io::Result<u32> {
    use std::ffi::CString;
    let c_name = CString::new(name)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let pwd = unsafe { libc::getpwnam(c_name.as_ptr()) };
    if pwd.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("invalid user: '{name}'"),
        ));
    }
    Ok(unsafe { (*pwd).pw_uid })
}

#[cfg(unix)]
fn lookup_gid(name: &str) -> io::Result<u32> {
    use std::ffi::CString;
    let c_name = CString::new(name)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let grp = unsafe { libc::getgrnam(c_name.as_ptr()) };
    if grp.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("invalid group: '{name}'"),
        ));
    }
    Ok(unsafe { (*grp).gr_gid })
}

/// On Windows, chown is not supported.
#[cfg(not(unix))]
pub fn chown(_config: &ChownConfig) -> io::Result<()> {
    eprintln!("chown: changing ownership is not supported on this platform");
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "chown is not supported on Windows",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owner_group_owner_only() {
        assert_eq!(parse_owner_group("root"), (Some("root"), None));
    }

    #[test]
    fn test_parse_owner_group_both() {
        assert_eq!(parse_owner_group("root:wheel"), (Some("root"), Some("wheel")));
    }

    #[test]
    fn test_parse_owner_group_group_only() {
        assert_eq!(parse_owner_group(":wheel"), (None, Some("wheel")));
    }

    #[test]
    fn test_parse_owner_group_colon_only() {
        assert_eq!(parse_owner_group(":"), (None, None));
    }

    #[test]
    fn test_parse_owner_group_empty() {
        assert_eq!(parse_owner_group(""), (None, None));
    }

    #[test]
    fn test_parse_owner_group_numeric() {
        assert_eq!(parse_owner_group("1000:1000"), (Some("1000"), Some("1000")));
    }
}
