use std::fs;
use std::io;
use std::path::Path;

use crate::cli::MkdirConfig;

pub fn create_directory(path: &Path, config: &MkdirConfig) -> io::Result<()> {
    if config.parents {
        fs::create_dir_all(path)?;
    } else {
        fs::create_dir(path)?;
    }

    #[cfg(unix)]
    if let Some(mode) = config.mode {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    }

    if config.verbose {
        println!("mkdir: created directory '{}'", path.display());
    }

    Ok(())
}
