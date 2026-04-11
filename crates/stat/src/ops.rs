use std::fs;
use std::io;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::cli::StatConfig;

pub fn stat_file(path: &Path, config: &StatConfig) -> io::Result<String> {
    let metadata = if config.dereference {
        fs::metadata(path)?
    } else {
        // Use symlink_metadata to not follow symlinks by default
        fs::symlink_metadata(path)?
    };

    let info = FileInfo::from_metadata(path, &metadata);

    if let Some(ref fmt) = config.format {
        return Ok(expand_format(fmt, &info));
    }

    if config.terse {
        return Ok(format_terse(&info));
    }

    Ok(format_default(&info))
}

struct FileInfo {
    name: String,
    size: u64,
    blocks: u64,
    raw_mode: u32,
    octal_mode: u32,
    uid: u32,
    gid: u32,
    file_type: String,
    atime: i64,
    mtime: i64,
    ctime: i64,
}

impl FileInfo {
    #[cfg(unix)]
    fn from_metadata(path: &Path, metadata: &fs::Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let file_type = file_type_string(metadata);
        FileInfo {
            name: path.to_string_lossy().to_string(),
            size: metadata.size(),
            blocks: metadata.blocks(),
            raw_mode: metadata.mode(),
            octal_mode: metadata.mode() & 0o7777,
            uid: metadata.uid(),
            gid: metadata.gid(),
            file_type,
            atime: metadata.atime(),
            mtime: metadata.mtime(),
            ctime: metadata.ctime(),
        }
    }

    #[cfg(not(unix))]
    fn from_metadata(path: &Path, metadata: &fs::Metadata) -> Self {
        let file_type = file_type_string(metadata);
        let size = metadata.len();
        // Approximate blocks as ceil(size / 512)
        let blocks = size.div_ceil(512);
        let mode: u32 = if metadata.permissions().readonly() {
            0o444
        } else {
            0o666
        };

        let mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let atime = metadata
            .accessed()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let ctime = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        FileInfo {
            name: path.to_string_lossy().to_string(),
            size,
            blocks,
            raw_mode: mode,
            octal_mode: mode,
            uid: 0,
            gid: 0,
            file_type,
            atime,
            mtime,
            ctime,
        }
    }
}

fn file_type_string(metadata: &fs::Metadata) -> String {
    let ft = metadata.file_type();
    if ft.is_file() {
        "regular file".to_string()
    } else if ft.is_dir() {
        "directory".to_string()
    } else if ft.is_symlink() {
        "symbolic link".to_string()
    } else {
        "other".to_string()
    }
}

fn expand_format(fmt: &str, info: &FileInfo) -> String {
    let mut result = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 1 < chars.len() {
            i += 1;
            match chars[i] {
                'n' => result.push_str(&info.name),
                's' => result.push_str(&info.size.to_string()),
                'b' => result.push_str(&info.blocks.to_string()),
                'f' => result.push_str(&format!("{:x}", info.raw_mode)),
                'a' => result.push_str(&format!("{:o}", info.octal_mode)),
                'U' => result.push_str(&info.uid.to_string()),
                'G' => result.push_str(&info.gid.to_string()),
                'Y' => result.push_str(&info.mtime.to_string()),
                'X' => result.push_str(&info.atime.to_string()),
                'Z' => result.push_str(&info.ctime.to_string()),
                'F' => result.push_str(&info.file_type),
                '%' => result.push('%'),
                other => {
                    result.push('%');
                    result.push(other);
                }
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }

    result
}

fn format_terse(info: &FileInfo) -> String {
    format!(
        "{} {} {} {:x} {:o} {} {} {} {} {}",
        info.name,
        info.size,
        info.blocks,
        info.raw_mode,
        info.octal_mode,
        info.uid,
        info.gid,
        info.atime,
        info.mtime,
        info.ctime,
    )
}

fn format_default(info: &FileInfo) -> String {
    format!(
        "  File: {}\n  Size: {:<15} Blocks: {:<10} {}\n  Mode: ({:04o}/{})  Uid: {}  Gid: {}\nAccess: {}\nModify: {}\nChange: {}",
        info.name,
        info.size,
        info.blocks,
        info.file_type,
        info.octal_mode,
        format_permissions(info.raw_mode),
        info.uid,
        info.gid,
        info.atime,
        info.mtime,
        info.ctime,
    )
}

fn format_permissions(mode: u32) -> String {
    let mut s = String::with_capacity(10);

    // File type character
    let ft = mode & 0o170000;
    s.push(match ft {
        0o140000 => 's', // socket
        0o120000 => 'l', // symlink
        0o100000 => '-', // regular
        0o060000 => 'b', // block
        0o040000 => 'd', // directory
        0o020000 => 'c', // char
        0o010000 => 'p', // fifo
        _ => '-',
    });

    // Owner
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    // Group
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    // Other
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_format_name_and_size() {
        let info = FileInfo {
            name: "test.txt".to_string(),
            size: 1234,
            blocks: 8,
            raw_mode: 0o100644,
            octal_mode: 0o644,
            uid: 1000,
            gid: 1000,
            file_type: "regular file".to_string(),
            atime: 100,
            mtime: 200,
            ctime: 300,
        };
        assert_eq!(expand_format("%n %s", &info), "test.txt 1234");
    }

    #[test]
    fn test_expand_format_mode() {
        let info = FileInfo {
            name: "f".to_string(),
            size: 0,
            blocks: 0,
            raw_mode: 0o100755,
            octal_mode: 0o755,
            uid: 0,
            gid: 0,
            file_type: "regular file".to_string(),
            atime: 0,
            mtime: 0,
            ctime: 0,
        };
        assert_eq!(expand_format("%a", &info), "755");
        assert_eq!(expand_format("%f", &info), "81ed");
    }

    #[test]
    fn test_format_permissions() {
        assert_eq!(format_permissions(0o100755), "-rwxr-xr-x");
        assert_eq!(format_permissions(0o040755), "drwxr-xr-x");
        assert_eq!(format_permissions(0o100644), "-rw-r--r--");
    }

    #[test]
    fn test_format_terse() {
        let info = FileInfo {
            name: "a.txt".to_string(),
            size: 10,
            blocks: 1,
            raw_mode: 0o100644,
            octal_mode: 0o644,
            uid: 1000,
            gid: 1000,
            file_type: "regular file".to_string(),
            atime: 100,
            mtime: 200,
            ctime: 300,
        };
        let terse = format_terse(&info);
        assert!(terse.starts_with("a.txt"));
        assert!(terse.contains("10"));
    }
}
