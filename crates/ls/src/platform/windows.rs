use std::fs::Metadata;
use std::os::windows::fs::MetadataExt;

use crate::entry::{FileEntry, FileType};

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
const FILE_ATTRIBUTE_READONLY: u32 = 0x1;

pub fn populate_metadata(entry: &mut FileEntry, metadata: &Metadata) {
    let attrs = metadata.file_attributes();
    entry.permissions.readonly = attrs & FILE_ATTRIBUTE_READONLY != 0;
    entry.permissions.executable = is_executable_extension(&entry.name);
    entry.nlinks = 1;
    entry.inode = None;
    entry.blocks = None;
    entry.owner = None;
    entry.group = None;
}

pub fn format_permissions(entry: &FileEntry) -> String {
    let type_char = match entry.file_type {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        _ => '-',
    };

    let r = 'r';
    let w = if entry.permissions.readonly { '-' } else { 'w' };
    let x = if entry.permissions.executable || entry.file_type == FileType::Directory {
        'x'
    } else {
        '-'
    };

    format!("{type_char}{r}{w}{x}{r}{w}{x}{r}{w}{x}")
}

pub fn is_hidden(name: &str, metadata: &Metadata) -> bool {
    if name.starts_with('.') {
        return true;
    }
    let attrs = metadata.file_attributes();
    attrs & FILE_ATTRIBUTE_HIDDEN != 0
}

fn is_executable_extension(name: &str) -> bool {
    let lower = name.to_lowercase();
    [".exe", ".cmd", ".bat", ".com", ".ps1"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}
