use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::cli::FileConfig;

/// Detect the type of a file by reading its magic bytes and checking properties.
pub fn detect_file_type(path: &Path, config: &FileConfig) -> io::Result<String> {
    let metadata = if config.dereference {
        fs::metadata(path)?
    } else {
        fs::symlink_metadata(path)?
    };

    // Check symlink first (only when not dereferencing)
    if metadata.file_type().is_symlink() {
        if config.mime || config.mime_type {
            return Ok("inode/symlink".to_string());
        }
        let target = fs::read_link(path)?;
        return Ok(format!("symbolic link to {}", target.display()));
    }

    if metadata.is_dir() {
        if config.mime || config.mime_type {
            return Ok("inode/directory".to_string());
        }
        return Ok("directory".to_string());
    }

    if metadata.len() == 0 {
        if config.mime || config.mime_type {
            return Ok("inode/x-empty".to_string());
        }
        return Ok("empty".to_string());
    }

    // Read first 16 bytes for magic detection
    let mut buf = [0u8; 16];
    let bytes_read = {
        let mut f = fs::File::open(path)?;
        f.read(&mut buf)?
    };

    let magic = &buf[..bytes_read];

    // Check magic bytes
    if magic.len() >= 4 && magic[..4] == [0x7f, b'E', b'L', b'F'] {
        if config.mime || config.mime_type {
            return Ok("application/x-executable".to_string());
        }
        return Ok("ELF executable".to_string());
    }

    if magic.len() >= 2 && magic[..2] == [b'M', b'Z'] {
        if config.mime || config.mime_type {
            return Ok("application/x-dosexec".to_string());
        }
        return Ok("PE32 executable (Windows)".to_string());
    }

    if magic.len() >= 5 && magic[..5] == [b'%', b'P', b'D', b'F', b'-'] {
        if config.mime || config.mime_type {
            return Ok("application/pdf".to_string());
        }
        return Ok("PDF document".to_string());
    }

    if magic.len() >= 8 && magic[..8] == [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a] {
        if config.mime || config.mime_type {
            return Ok("image/png".to_string());
        }
        return Ok("PNG image data".to_string());
    }

    if magic.len() >= 3 && magic[..3] == [0xff, 0xd8, 0xff] {
        if config.mime || config.mime_type {
            return Ok("image/jpeg".to_string());
        }
        return Ok("JPEG image data".to_string());
    }

    if magic.len() >= 6 && (magic[..6] == [b'G', b'I', b'F', b'8', b'7', b'a']
        || magic[..6] == [b'G', b'I', b'F', b'8', b'9', b'a'])
    {
        if config.mime || config.mime_type {
            return Ok("image/gif".to_string());
        }
        return Ok("GIF image data".to_string());
    }

    if magic.len() >= 4 && magic[..4] == [b'P', b'K', 0x03, 0x04] {
        if config.mime || config.mime_type {
            return Ok("application/zip".to_string());
        }
        return Ok("Zip archive data".to_string());
    }

    if magic.len() >= 2 && magic[..2] == [0x1f, 0x8b] {
        if config.mime || config.mime_type {
            return Ok("application/gzip".to_string());
        }
        return Ok("gzip compressed data".to_string());
    }

    // Check if text (read more data for this)
    let content = fs::read(path)?;
    let check_len = content.len().min(8192);
    let sample = &content[..check_len];

    if is_utf8_text(sample) {
        if config.mime || config.mime_type {
            return Ok("text/plain".to_string());
        }
        return Ok("ASCII text".to_string());
    }

    if config.mime || config.mime_type {
        return Ok("application/octet-stream".to_string());
    }
    Ok("data".to_string())
}

/// Check if the given bytes look like valid UTF-8 text.
fn is_utf8_text(data: &[u8]) -> bool {
    if std::str::from_utf8(data).is_err() {
        return false;
    }
    // Reject if it contains too many control characters (except common whitespace)
    let control_count = data
        .iter()
        .filter(|&&b| b < 0x20 && b != b'\n' && b != b'\r' && b != b'\t' && b != 0x0c)
        .count();
    // Allow up to 1% control chars
    control_count * 100 <= data.len()
}

/// Run the file command on all files.
pub fn run(config: &FileConfig) -> io::Result<()> {
    let mut had_error = false;

    for file in &config.files {
        let path = Path::new(file);
        match detect_file_type(path, config) {
            Ok(file_type) => {
                if config.brief {
                    println!("{file_type}");
                } else {
                    println!("{file}: {file_type}");
                }
            }
            Err(e) => {
                eprintln!("file: {file}: {e}");
                had_error = true;
            }
        }
    }

    if had_error {
        Err(io::Error::other("some files could not be read"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_utf8_text_plain() {
        assert!(is_utf8_text(b"Hello, world!\n"));
    }

    #[test]
    fn test_is_utf8_text_binary() {
        assert!(!is_utf8_text(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]));
    }

    #[test]
    fn test_is_utf8_text_with_tabs() {
        assert!(is_utf8_text(b"line1\tvalue\nline2\tvalue\n"));
    }
}
