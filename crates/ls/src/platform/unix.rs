use std::fs::Metadata;
use std::os::unix::fs::{FileTypeExt, MetadataExt};

use crate::entry::{FileEntry, FileType};

pub fn populate_metadata(entry: &mut FileEntry, metadata: &Metadata) {
    let mode = metadata.mode();
    entry.permissions.mode = Some(mode);
    entry.permissions.executable = mode & 0o111 != 0;
    entry.nlinks = metadata.nlink();
    entry.inode = Some(metadata.ino());
    entry.blocks = Some(metadata.blocks());
    entry.owner = Some(metadata.uid().to_string());
    entry.group = Some(metadata.gid().to_string());

    let ft = metadata.file_type();
    if ft.is_fifo() {
        entry.file_type = FileType::Pipe;
    } else if ft.is_socket() {
        entry.file_type = FileType::Socket;
    } else if ft.is_block_device() {
        entry.file_type = FileType::BlockDevice;
    } else if ft.is_char_device() {
        entry.file_type = FileType::CharDevice;
    }
}

pub fn format_permissions(entry: &FileEntry) -> String {
    let mode = entry.permissions.mode.unwrap_or(0);
    let type_char = match entry.file_type {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::Pipe => 'p',
        FileType::Socket => 's',
        FileType::BlockDevice => 'b',
        FileType::CharDevice => 'c',
        _ => '-',
    };

    let mut perms = String::with_capacity(10);
    perms.push(type_char);

    for shift in [6, 3, 0] {
        let bits = (mode >> shift) & 0o7;
        perms.push(if bits & 4 != 0 { 'r' } else { '-' });
        perms.push(if bits & 2 != 0 { 'w' } else { '-' });
        let exec = bits & 1 != 0;
        if shift == 6 && mode & 0o4000 != 0 {
            perms.push(if exec { 's' } else { 'S' });
        } else if shift == 3 && mode & 0o2000 != 0 {
            perms.push(if exec { 's' } else { 'S' });
        } else if shift == 0 && mode & 0o1000 != 0 {
            perms.push(if exec { 't' } else { 'T' });
        } else {
            perms.push(if exec { 'x' } else { '-' });
        }
    }

    perms
}

pub fn is_hidden(name: &str, _metadata: &Metadata) -> bool {
    name.starts_with('.')
}
