use std::path::PathBuf;

use ls::cli::{ColorMode, ResolvedConfig};
use ls::color::ColorScheme;
use ls::entry::{FileEntry, FileType, Permissions};

fn make_entry(name: &str, file_type: FileType) -> FileEntry {
    FileEntry {
        name: name.to_string(),
        path: PathBuf::from(name),
        file_type,
        size: 0,
        modified: None,
        accessed: None,
        created: None,
        permissions: Permissions {
            mode: None,
            readonly: false,
            executable: false,
        },
        owner: None,
        group: None,
        nlinks: 1,
        inode: None,
        blocks: None,
        symlink_target: None,
    }
}

fn enabled_colors() -> ColorScheme {
    ColorScheme::new(&ResolvedConfig {
        color_mode: ColorMode::Always,
        ..Default::default()
    })
}

fn disabled_colors() -> ColorScheme {
    ColorScheme::new(&ResolvedConfig {
        color_mode: ColorMode::Never,
        ..Default::default()
    })
}

#[test]
fn directory_gets_blue() {
    let colors = enabled_colors();
    let entry = make_entry("mydir", FileType::Directory);
    let result = colors.colorize("mydir", &entry);
    assert!(result.contains("\x1b[1;34m"), "directory should be bold blue");
    assert!(result.contains("\x1b[0m"), "should have reset code");
    assert!(result.contains("mydir"));
}

#[test]
fn executable_gets_green() {
    let colors = enabled_colors();
    let mut entry = make_entry("script", FileType::Regular);
    entry.permissions.executable = true;
    let result = colors.colorize("script", &entry);
    assert!(
        result.contains("\x1b[1;32m"),
        "executable should be bold green"
    );
}

#[test]
fn symlink_valid_gets_cyan() {
    let colors = enabled_colors();
    let mut entry = make_entry("link", FileType::Symlink);
    // target exists check depends on actual filesystem, so we test with None target
    // which means symlink_target.is_some_and(|t| t.exists()) is false → broken link
    entry.symlink_target = None;
    let result = colors.colorize("link", &entry);
    // No target → broken symlink → red
    assert!(result.contains("\x1b[1;31m"), "broken symlink should be red");
}

#[test]
fn symlink_broken_gets_red() {
    let colors = enabled_colors();
    let mut entry = make_entry("link", FileType::Symlink);
    entry.symlink_target = Some(PathBuf::from("/nonexistent/path/99999"));
    let result = colors.colorize("link", &entry);
    assert!(result.contains("\x1b[1;31m"), "broken symlink should be red");
}

#[test]
fn regular_file_no_color() {
    let colors = enabled_colors();
    let entry = make_entry("file.txt", FileType::Regular);
    let result = colors.colorize("file.txt", &entry);
    assert_eq!(result, "file.txt", "regular file should have no ANSI codes");
}

#[test]
fn disabled_colors_no_codes() {
    let colors = disabled_colors();

    let dir = make_entry("mydir", FileType::Directory);
    assert_eq!(
        colors.colorize("mydir", &dir),
        "mydir",
        "disabled colors should produce plain text"
    );

    let mut exec = make_entry("script", FileType::Regular);
    exec.permissions.executable = true;
    assert_eq!(colors.colorize("script", &exec), "script");
}
