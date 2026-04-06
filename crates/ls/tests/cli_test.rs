use ls::cli::*;

fn parse(args: &[&str]) -> ResolvedConfig {
    use clap::Parser;
    let mut full_args = vec!["ls"];
    full_args.extend_from_slice(args);
    let cli = Cli::parse_from(full_args);
    ResolvedConfig::from_cli(cli)
}

#[test]
fn default_config() {
    let config = ResolvedConfig::default();
    assert_eq!(config.sort_key, SortKey::Name);
    assert_eq!(config.show_hidden, HiddenMode::None);
    assert_eq!(config.format_mode, FormatMode::SingleColumn);
    assert!(!config.recursive);
    assert!(!config.directory_mode);
}

#[test]
fn flag_f_implies_all_nosort_nocolor() {
    let config = parse(&["-f"]);
    assert_eq!(config.show_hidden, HiddenMode::All);
    assert_eq!(config.sort_key, SortKey::None);
    assert_eq!(config.color_mode, ColorMode::Never);
}

#[test]
fn flag_full_time_implies_long_and_full_iso() {
    let config = parse(&["--full-time"]);
    assert_eq!(config.format_mode, FormatMode::Long);
    assert_eq!(config.time_style, TimeStyle::FullIso);
}

#[test]
fn flag_g_implies_long_no_owner() {
    let config = parse(&["-g"]);
    assert_eq!(config.format_mode, FormatMode::Long);
    assert!(!config.show_owner);
    assert!(config.show_group);
}

#[test]
fn flag_o_implies_long_no_group() {
    let config = parse(&["-o"]);
    assert_eq!(config.format_mode, FormatMode::Long);
    assert!(config.show_owner);
    assert!(!config.show_group);
}

#[test]
fn explicit_sort_key_overrides_short_flags() {
    let config = parse(&["-S", "--sort=time"]);
    assert_eq!(config.sort_key, SortKey::Time);
}

#[test]
fn explicit_format_overrides_short_flags() {
    let config = parse(&["-l", "--format=commas"]);
    assert_eq!(config.format_mode, FormatMode::Commas);
}

#[test]
fn human_readable_flag() {
    let config = parse(&["-h"]);
    assert!(config.human_readable);
}

#[test]
fn recursive_flag() {
    let config = parse(&["-R"]);
    assert!(config.recursive);
}

#[test]
fn directory_flag() {
    let config = parse(&["-d"]);
    assert!(config.directory_mode);
}

#[test]
fn width_override() {
    let config = parse(&["-w", "120"]);
    assert_eq!(config.terminal_width, 120);
}

#[test]
fn color_always() {
    let config = parse(&["--color=always"]);
    assert_eq!(config.color_mode, ColorMode::Always);
}

#[test]
fn color_never() {
    let config = parse(&["--color=never"]);
    assert_eq!(config.color_mode, ColorMode::Never);
}

#[test]
fn classify_flag() {
    let config = parse(&["-F"]);
    assert_eq!(config.classify, ClassifyMode::All);
}

#[test]
fn multiple_paths() {
    let config = parse(&["/tmp", "/var"]);
    assert_eq!(config.paths.len(), 2);
}

#[test]
fn time_field_ctime() {
    let config = parse(&["-c"]);
    assert_eq!(config.time_field, TimeField::Created);
}

#[test]
fn time_field_explicit() {
    let config = parse(&["--time=atime"]);
    assert_eq!(config.time_field, TimeField::Accessed);
}

#[test]
fn block_size_parse() {
    let config = parse(&["--block-size=4K"]);
    assert_eq!(config.block_size, Some(4096));
}

#[test]
fn ignore_backups_flag() {
    let config = parse(&["-B"]);
    assert!(config.ignore_backups);
}

#[test]
fn group_directories_first_flag() {
    let config = parse(&["--group-directories-first"]);
    assert!(config.group_dirs_first);
}
