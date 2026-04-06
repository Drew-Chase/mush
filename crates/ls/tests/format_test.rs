use std::path::PathBuf;

use ls::cli::{ClassifyMode, ColorMode, FormatMode, ResolvedConfig};
use ls::color::{visible_width, ColorScheme};
use ls::entry::{FileEntry, FileType, Permissions};
use ls::format;

fn make_entry(name: &str, size: u64, file_type: FileType) -> FileEntry {
    FileEntry {
        name: name.to_string(),
        path: PathBuf::from(name),
        file_type,
        size,
        modified: Some(std::time::SystemTime::UNIX_EPOCH),
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

fn output_string(entries: &[FileEntry], config: &ResolvedConfig) -> String {
    let colors = ColorScheme::new(config);
    let mut buf = Vec::new();
    format::write_output(entries, config, &colors, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

#[test]
fn format_size_raw_bytes() {
    let config = ResolvedConfig::default();
    assert_eq!(format::format_size(1024, &config), "1024");
    assert_eq!(format::format_size(0, &config), "0");
}

#[test]
fn format_size_human_readable() {
    let config = ResolvedConfig {
        human_readable: true,
        ..Default::default()
    };
    assert_eq!(format::format_size(0, &config), "0");
    assert_eq!(format::format_size(500, &config), "500");
    assert_eq!(format::format_size(1024, &config), "1.0K");
    assert_eq!(format::format_size(1536, &config), "1.5K");
    assert_eq!(format::format_size(10240, &config), "10K");
    assert_eq!(format::format_size(1048576, &config), "1.0M");
    assert_eq!(format::format_size(1073741824, &config), "1.0G");
}

#[test]
fn format_size_block_size() {
    let config = ResolvedConfig {
        block_size: Some(1024),
        ..Default::default()
    };
    assert_eq!(format::format_size(0, &config), "0");
    assert_eq!(format::format_size(1024, &config), "1");
    assert_eq!(format::format_size(1025, &config), "2"); // rounds up
    assert_eq!(format::format_size(4096, &config), "4");
}

#[test]
fn single_column_output() {
    let entries = vec![
        make_entry("alpha", 0, FileType::Regular),
        make_entry("bravo", 0, FileType::Regular),
        make_entry("charlie", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        format_mode: FormatMode::SingleColumn,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert_eq!(out, "alpha\nbravo\ncharlie\n");
}

#[test]
fn comma_output() {
    let entries = vec![
        make_entry("a", 0, FileType::Regular),
        make_entry("b", 0, FileType::Regular),
        make_entry("c", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        format_mode: FormatMode::Commas,
        color_mode: ColorMode::Never,
        terminal_width: 80,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert_eq!(out, "a, b, c\n");
}

#[test]
fn long_output_contains_permissions_and_size() {
    let entries = vec![make_entry("file.txt", 42, FileType::Regular)];
    let config = ResolvedConfig {
        format_mode: FormatMode::Long,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert!(out.contains("42"), "long output should contain file size");
    assert!(
        out.contains("file.txt"),
        "long output should contain file name"
    );
}

#[test]
fn classify_appends_slash_to_dirs() {
    let entries = vec![
        make_entry("mydir", 0, FileType::Directory),
        make_entry("myfile", 0, FileType::Regular),
    ];
    let config = ResolvedConfig {
        format_mode: FormatMode::SingleColumn,
        classify: ClassifyMode::All,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert!(out.contains("mydir/"), "directory should have / suffix");
    assert!(
        !out.contains("myfile/"),
        "regular file should not have / suffix"
    );
}

#[test]
fn classify_appends_star_to_executables() {
    let mut entry = make_entry("script", 0, FileType::Regular);
    entry.permissions.executable = true;
    let entries = vec![entry];
    let config = ResolvedConfig {
        format_mode: FormatMode::SingleColumn,
        classify: ClassifyMode::All,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert!(out.contains("script*"), "executable should have * suffix");
}

#[test]
fn file_type_mode_no_star_for_executables() {
    let mut entry = make_entry("script", 0, FileType::Regular);
    entry.permissions.executable = true;
    let entries = vec![entry];
    let config = ResolvedConfig {
        format_mode: FormatMode::SingleColumn,
        classify: ClassifyMode::FileTypeOnly,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert!(
        !out.contains("script*"),
        "file-type mode should not append * to executables"
    );
}

#[test]
fn classify_symlink_at_sign() {
    let mut entry = make_entry("link", 0, FileType::Symlink);
    entry.symlink_target = Some(PathBuf::from("target"));
    let entries = vec![entry];
    let config = ResolvedConfig {
        format_mode: FormatMode::SingleColumn,
        classify: ClassifyMode::All,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    assert!(
        out.contains("link@"),
        "symlink should have @ suffix in classify mode"
    );
}

#[test]
fn visible_width_plain_text() {
    assert_eq!(visible_width("hello"), 5);
    assert_eq!(visible_width(""), 0);
}

#[test]
fn visible_width_strips_ansi() {
    assert_eq!(visible_width("\x1b[1;34mhello\x1b[0m"), 5);
    assert_eq!(visible_width("\x1b[31mred\x1b[0m plain"), 9);
}

#[test]
fn format_name_with_symlink_arrow() {
    let colors = ColorScheme::new(&ResolvedConfig {
        color_mode: ColorMode::Never,
        ..Default::default()
    });
    let mut entry = make_entry("link", 0, FileType::Symlink);
    entry.symlink_target = Some(PathBuf::from("/usr/bin/target"));
    let config = ResolvedConfig {
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let name = format::format_name(&entry, &config, &colors);
    assert!(
        name.contains("->"),
        "symlink name should contain arrow to target"
    );
    assert!(name.contains("/usr/bin/target"));
}

#[test]
fn escape_nongraphic_chars() {
    let colors = ColorScheme::new(&ResolvedConfig {
        color_mode: ColorMode::Never,
        ..Default::default()
    });
    let entry = make_entry("file\x01name", 0, FileType::Regular);
    let config = ResolvedConfig {
        escape_nongraphic: true,
        color_mode: ColorMode::Never,
        ..Default::default()
    };
    let name = format::format_name(&entry, &config, &colors);
    assert!(
        name.contains("\\001"),
        "control character should be escaped as octal"
    );
}

#[test]
fn grid_output_multiple_columns() {
    let entries: Vec<FileEntry> = (0..10)
        .map(|i| make_entry(&format!("f{i}"), 0, FileType::Regular))
        .collect();
    let config = ResolvedConfig {
        format_mode: FormatMode::Grid,
        color_mode: ColorMode::Never,
        terminal_width: 80,
        ..Default::default()
    };
    let out = output_string(&entries, &config);
    let lines: Vec<&str> = out.lines().collect();
    assert!(
        lines.len() < 10,
        "grid format with 80 cols should use fewer than 10 lines for short names"
    );
}
