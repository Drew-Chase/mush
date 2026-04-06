use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::platform;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Pipe,
    Socket,
    BlockDevice,
    CharDevice,
    Unknown,
}

pub struct Permissions {
    #[allow(dead_code)]
    pub mode: Option<u32>,
    pub readonly: bool,
    pub executable: bool,
}

pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub permissions: Permissions,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub nlinks: u64,
    pub inode: Option<u64>,
    pub blocks: Option<u64>,
    pub symlink_target: Option<PathBuf>,
}

impl FileEntry {
    pub fn from_dir_entry(dir_entry: &DirEntry) -> io::Result<Self> {
        let path = dir_entry.path();
        let metadata = path.symlink_metadata()?;
        let name = dir_entry.file_name().to_string_lossy().into_owned();
        let mut entry = Self::build(name, path, &metadata);
        if entry.file_type == FileType::Symlink {
            entry.symlink_target = fs::read_link(&entry.path).ok();
        }
        Ok(entry)
    }

    pub fn from_path(path: PathBuf) -> io::Result<Self> {
        let metadata = path.symlink_metadata()?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        let mut entry = Self::build(name, path, &metadata);
        if entry.file_type == FileType::Symlink {
            entry.symlink_target = fs::read_link(&entry.path).ok();
        }
        Ok(entry)
    }

    fn build(name: String, path: PathBuf, metadata: &Metadata) -> Self {
        let ft = metadata.file_type();
        let file_type = if ft.is_dir() {
            FileType::Directory
        } else if ft.is_symlink() {
            FileType::Symlink
        } else if ft.is_file() {
            FileType::Regular
        } else {
            FileType::Unknown
        };

        let mut entry = Self {
            name,
            path,
            file_type,
            size: metadata.len(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
            created: metadata.created().ok(),
            permissions: Permissions {
                mode: None,
                readonly: metadata.permissions().readonly(),
                executable: false,
            },
            owner: None,
            group: None,
            nlinks: 1,
            inode: None,
            blocks: None,
            symlink_target: None,
        };

        platform::populate_metadata(&mut entry, metadata);
        entry
    }
}
