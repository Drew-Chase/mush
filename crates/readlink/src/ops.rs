use std::env;
use std::path::{Path, PathBuf};

use crate::cli::ReadlinkConfig;

pub fn readlink(path: &str, config: &ReadlinkConfig) -> Result<String, String> {
    if config.canonicalize || config.canonicalize_existing || config.canonicalize_missing {
        return canonicalize_path(path, config);
    }

    // Default: just read the symlink target
    std::fs::read_link(path)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| format!("{path}: {e}"))
}

fn canonicalize_path(path: &str, config: &ReadlinkConfig) -> Result<String, String> {
    let p = Path::new(path);

    if config.canonicalize_existing {
        // All components must exist
        std::fs::canonicalize(p)
            .map(|r| r.to_string_lossy().into_owned())
            .map_err(|e| format!("{path}: {e}"))
    } else if config.canonicalize_missing {
        // No components need exist
        let absolute = if p.is_absolute() {
            p.to_path_buf()
        } else {
            env::current_dir()
                .map_err(|e| format!("{e}"))?
                .join(p)
        };
        match std::fs::canonicalize(&absolute) {
            Ok(resolved) => Ok(resolved.to_string_lossy().into_owned()),
            Err(_) => Ok(normalize_path(&absolute).to_string_lossy().into_owned()),
        }
    } else {
        // -f: all but the last component must exist
        std::fs::canonicalize(p)
            .or_else(|_| {
                if let Some(parent) = p.parent() {
                    let resolved_parent = if parent.as_os_str().is_empty() {
                        env::current_dir().map_err(|e| format!("{e}"))?
                    } else {
                        std::fs::canonicalize(parent)
                            .map_err(|e| format!("{path}: {e}"))?
                    };
                    if let Some(file_name) = p.file_name() {
                        Ok(resolved_parent.join(file_name))
                    } else {
                        Ok(resolved_parent)
                    }
                } else {
                    Err(format!("{path}: No such file or directory"))
                }
            })
            .map(|r| r.to_string_lossy().into_owned())
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {}
            other => components.push(other),
        }
    }

    if components.is_empty() {
        PathBuf::from("/")
    } else {
        components.iter().collect()
    }
}
