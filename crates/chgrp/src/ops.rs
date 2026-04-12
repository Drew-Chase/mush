use std::io;

use crate::cli::ChgrpConfig;

/// Change group ownership of files.
#[cfg(unix)]
pub fn chgrp(config: &ChgrpConfig) -> io::Result<()> {
    use std::fs;
    use std::path::Path;

    let gid = if let Some(ref reference) = config.reference {
        let meta = fs::metadata(Path::new(reference))?;
        get_gid(&meta)
    } else if let Ok(id) = config.group.parse::<u32>() {
        id
    } else {
        lookup_gid(&config.group)?
    };

    for file in &config.files {
        let path = Path::new(file);
        chgrp_single(path, gid, config)?;

        if config.recursive && path.is_dir() {
            chgrp_recursive(path, gid, config)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn chgrp_single(path: &std::path::Path, gid: u32, config: &ChgrpConfig) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // uid = u32::MAX (-1 as unsigned) means don't change owner
    let ret = if config.no_deref {
        unsafe { libc::lchown(c_path.as_ptr(), u32::MAX, gid) }
    } else {
        unsafe { libc::chown(c_path.as_ptr(), u32::MAX, gid) }
    };

    if ret != 0 {
        let err = io::Error::last_os_error();
        if !config.quiet {
            eprintln!("chgrp: changing group of '{}': {}", path.display(), err);
        }
        return Err(err);
    }

    if config.verbose {
        eprintln!("changed group of '{}'", path.display());
    }

    Ok(())
}

#[cfg(unix)]
fn chgrp_recursive(dir: &std::path::Path, gid: u32, config: &ChgrpConfig) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        chgrp_single(&path, gid, config)?;
        if path.is_dir() {
            chgrp_recursive(&path, gid, config)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn get_gid(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    meta.gid()
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

/// On Windows, chgrp is not supported.
#[cfg(not(unix))]
pub fn chgrp(_config: &ChgrpConfig) -> io::Result<()> {
    eprintln!("chgrp: changing group ownership is not supported on this platform");
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "chgrp is not supported on Windows",
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // chgrp functionality is platform-specific; parse tests are in cli_test
        assert!(true);
    }
}
