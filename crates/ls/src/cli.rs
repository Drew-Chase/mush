use clap::{Parser, ValueEnum};
use std::io::IsTerminal;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ls", about = "List directory contents", disable_help_flag = true)]
pub struct Cli {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help information")]
    pub help: Option<bool>,

    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    #[arg(short = 'a', long = "all", help = "Do not ignore entries starting with .")]
    pub all: bool,

    #[arg(short = 'A', long = "almost-all", help = "Do not list implied . and ..")]
    pub almost_all: bool,

    #[arg(short = 'B', long = "ignore-backups", help = "Do not list entries ending with ~")]
    pub ignore_backups: bool,

    #[arg(short = 'd', long = "directory", help = "List directories themselves, not their contents")]
    pub directory: bool,

    #[arg(short = 'l', help = "Use long listing format")]
    pub long: bool,

    #[arg(short = 'C', help = "List entries by columns")]
    pub columns: bool,

    #[arg(short = '1', help = "List one file per line")]
    pub one_per_line: bool,

    #[arg(short = 'm', help = "Fill width with a comma separated list of entries")]
    pub comma: bool,

    #[arg(long = "format", value_enum, help = "Output format")]
    pub format: Option<FormatArg>,

    #[arg(short = 'g', help = "Like -l, but do not list owner")]
    pub no_owner: bool,

    #[arg(short = 'o', help = "Like -l, but do not list group")]
    pub no_group: bool,

    #[arg(short = 'h', long = "human-readable", help = "Print human readable sizes")]
    pub human_readable: bool,

    #[arg(short = 'i', long = "inode", help = "Print the index number of each file")]
    pub inode: bool,

    #[arg(short = 'n', long = "numeric-uid-gid", help = "List numeric user and group IDs")]
    pub numeric_uid_gid: bool,

    #[arg(short = 's', long = "size", help = "Print the allocated size of each file")]
    pub show_size: bool,

    #[arg(long = "block-size", value_name = "SIZE", help = "Scale sizes by SIZE")]
    pub block_size: Option<String>,

    #[arg(short = 'S', help = "Sort by file size, largest first")]
    pub sort_by_size: bool,

    #[arg(short = 't', help = "Sort by time, newest first")]
    pub sort_by_time: bool,

    #[arg(short = 'X', help = "Sort alphabetically by entry extension")]
    pub sort_by_extension: bool,

    #[arg(short = 'U', help = "Do not sort; list entries in directory order")]
    pub no_sort: bool,

    #[arg(short = 'r', long = "reverse", help = "Reverse order while sorting")]
    pub reverse: bool,

    #[arg(long = "sort", value_enum, value_name = "WORD", help = "Sort by WORD")]
    pub sort_key: Option<SortKeyArg>,

    #[arg(long = "group-directories-first", help = "Group directories before files")]
    pub group_directories_first: bool,

    #[arg(short = 'c', help = "Sort by, and show, ctime")]
    pub ctime: bool,

    #[arg(long = "time", value_enum, value_name = "WORD", help = "Select which timestamp to use")]
    pub time_field: Option<TimeFieldArg>,

    #[arg(long = "time-style", value_name = "STYLE", help = "Time display format")]
    pub time_style: Option<String>,

    #[arg(long = "full-time", help = "Like -l --time-style=full-iso")]
    pub full_time: bool,

    #[arg(long = "color", default_missing_value = "always", num_args = 0..=1, value_enum, help = "Colorize the output")]
    pub color: Option<ColorWhenArg>,

    #[arg(short = 'F', long = "classify", help = "Append indicator (*/=>@|) to entries")]
    pub classify: bool,

    #[arg(long = "file-type", help = "Like --classify, except do not append '*'")]
    pub file_type_indicator: bool,

    #[arg(short = 'p', help = "Append / indicator to directories")]
    pub slash_dirs: bool,

    #[arg(short = 'b', long = "escape", help = "Print C-style escapes for nongraphic characters")]
    pub escape: bool,

    #[arg(short = 'R', long = "recursive", help = "List subdirectories recursively")]
    pub recursive: bool,

    #[arg(short = 'w', long = "width", value_name = "COLS", help = "Set output width")]
    pub width: Option<u16>,

    #[arg(short = 'f', help = "Do not sort, enable -aU, disable -ls --color")]
    pub force_all_unsorted: bool,

    #[arg(short = 'D', long = "dired", help = "Generate output designed for Emacs' dired mode")]
    pub dired: bool,

    #[arg(long = "author", help = "With -l, print the author of each file")]
    pub author: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum FormatArg {
    Long,
    Verbose,
    Commas,
    Horizontal,
    Across,
    #[value(name = "single-column")]
    SingleColumn,
    Vertical,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SortKeyArg {
    None,
    Size,
    Time,
    Extension,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum TimeFieldArg {
    Atime,
    Access,
    Use,
    Ctime,
    Status,
    Mtime,
    Modification,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ColorWhenArg {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HiddenMode {
    None,
    AlmostAll,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatMode {
    Long,
    Grid,
    SingleColumn,
    Commas,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Size,
    Time,
    Extension,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeField {
    Modified,
    Accessed,
    Created,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeStyle {
    Default,
    FullIso,
    LongIso,
    Iso,
    Custom(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassifyMode {
    None,
    All,
    FileTypeOnly,
    SlashDirs,
}

pub struct ResolvedConfig {
    pub paths: Vec<PathBuf>,
    pub show_hidden: HiddenMode,
    pub ignore_backups: bool,
    pub format_mode: FormatMode,
    pub sort_key: SortKey,
    pub sort_reverse: bool,
    pub group_dirs_first: bool,
    pub time_field: TimeField,
    pub time_style: TimeStyle,
    pub color_mode: ColorMode,
    pub classify: ClassifyMode,
    pub human_readable: bool,
    pub block_size: Option<u64>,
    pub show_inode: bool,
    pub show_blocks: bool,
    pub show_owner: bool,
    pub show_group: bool,
    #[allow(dead_code)]
    pub numeric_ids: bool,
    #[allow(dead_code)]
    pub show_author: bool,
    pub escape_nongraphic: bool,
    pub recursive: bool,
    pub directory_mode: bool,
    pub terminal_width: u16,
}

impl Default for ResolvedConfig {
    fn default() -> Self {
        Self {
            paths: vec![PathBuf::from(".")],
            show_hidden: HiddenMode::None,
            ignore_backups: false,
            format_mode: FormatMode::SingleColumn,
            sort_key: SortKey::Name,
            sort_reverse: false,
            group_dirs_first: false,
            time_field: TimeField::Modified,
            time_style: TimeStyle::Default,
            color_mode: ColorMode::Never,
            classify: ClassifyMode::None,
            human_readable: false,
            block_size: None,
            show_inode: false,
            show_blocks: false,
            show_owner: true,
            show_group: true,
            numeric_ids: false,
            show_author: false,
            escape_nongraphic: false,
            recursive: false,
            directory_mode: false,
            terminal_width: 80,
        }
    }
}

impl ResolvedConfig {
    pub fn from_cli(cli: Cli) -> Self {
        let is_tty = std::io::stdout().is_terminal();

        let mut show_hidden = if cli.all {
            HiddenMode::All
        } else if cli.almost_all {
            HiddenMode::AlmostAll
        } else {
            HiddenMode::None
        };

        let mut format_mode = if cli.long || cli.no_owner || cli.no_group {
            FormatMode::Long
        } else if cli.one_per_line {
            FormatMode::SingleColumn
        } else if cli.comma {
            FormatMode::Commas
        } else if cli.columns || is_tty {
            FormatMode::Grid
        } else {
            FormatMode::SingleColumn
        };

        if let Some(fmt) = cli.format {
            format_mode = match fmt {
                FormatArg::Long | FormatArg::Verbose => FormatMode::Long,
                FormatArg::Commas => FormatMode::Commas,
                FormatArg::Horizontal | FormatArg::Across | FormatArg::Vertical => FormatMode::Grid,
                FormatArg::SingleColumn => FormatMode::SingleColumn,
            };
        }

        let mut sort_key = if cli.no_sort {
            SortKey::None
        } else if cli.sort_by_size {
            SortKey::Size
        } else if cli.sort_by_time {
            SortKey::Time
        } else if cli.sort_by_extension {
            SortKey::Extension
        } else {
            SortKey::Name
        };

        if let Some(sk) = cli.sort_key {
            sort_key = match sk {
                SortKeyArg::None => SortKey::None,
                SortKeyArg::Size => SortKey::Size,
                SortKeyArg::Time => SortKey::Time,
                SortKeyArg::Extension => SortKey::Extension,
            };
        }

        let mut time_field = if cli.ctime {
            TimeField::Created
        } else {
            TimeField::Modified
        };

        if let Some(tf) = cli.time_field {
            time_field = match tf {
                TimeFieldArg::Atime | TimeFieldArg::Access | TimeFieldArg::Use => TimeField::Accessed,
                TimeFieldArg::Ctime | TimeFieldArg::Status => TimeField::Created,
                TimeFieldArg::Mtime | TimeFieldArg::Modification => TimeField::Modified,
            };
        }

        let mut time_style = if let Some(ref style) = cli.time_style {
            match style.as_str() {
                "full-iso" => TimeStyle::FullIso,
                "long-iso" => TimeStyle::LongIso,
                "iso" => TimeStyle::Iso,
                other => TimeStyle::Custom(other.to_string()),
            }
        } else {
            TimeStyle::Default
        };

        let mut color_mode = match cli.color {
            Some(ColorWhenArg::Always) => ColorMode::Always,
            Some(ColorWhenArg::Auto) => ColorMode::Auto,
            Some(ColorWhenArg::Never) => ColorMode::Never,
            None => ColorMode::Auto,
        };

        let classify = if cli.classify {
            ClassifyMode::All
        } else if cli.file_type_indicator {
            ClassifyMode::FileTypeOnly
        } else if cli.slash_dirs {
            ClassifyMode::SlashDirs
        } else {
            ClassifyMode::None
        };

        let mut show_owner = true;
        let mut show_group = true;

        if cli.no_owner {
            format_mode = FormatMode::Long;
            show_owner = false;
        }
        if cli.no_group {
            format_mode = FormatMode::Long;
            show_group = false;
        }

        if cli.full_time {
            format_mode = FormatMode::Long;
            time_style = TimeStyle::FullIso;
        }

        if cli.force_all_unsorted {
            show_hidden = HiddenMode::All;
            sort_key = SortKey::None;
            color_mode = ColorMode::Never;
        }

        let block_size = cli.block_size.as_deref().and_then(parse_block_size);

        let terminal_width = cli.width.unwrap_or_else(|| {
            terminal_size::terminal_size()
                .map(|(w, _)| w.0)
                .unwrap_or(80)
        });

        ResolvedConfig {
            paths: cli.paths,
            show_hidden,
            ignore_backups: cli.ignore_backups,
            format_mode,
            sort_key,
            sort_reverse: cli.reverse,
            group_dirs_first: cli.group_directories_first,
            time_field,
            time_style,
            color_mode,
            classify,
            human_readable: cli.human_readable,
            block_size,
            show_inode: cli.inode,
            show_blocks: cli.show_size,
            show_owner,
            show_group,
            numeric_ids: cli.numeric_uid_gid,
            show_author: cli.author,
            escape_nongraphic: cli.escape,
            recursive: cli.recursive,
            directory_mode: cli.directory,
            terminal_width,
        }
    }
}

fn parse_block_size(s: &str) -> Option<u64> {
    let s = s.trim();
    let (num_part, suffix) = if s.chars().last().is_some_and(|c| c.is_alphabetic()) {
        let idx = s.len() - 1;
        (&s[..idx], &s[idx..])
    } else {
        (s, "")
    };

    let base: u64 = if num_part.is_empty() { 1 } else { num_part.parse().ok()? };

    let multiplier = match suffix.to_uppercase().as_str() {
        "" => 1,
        "K" => 1024,
        "M" => 1024 * 1024,
        "G" => 1024 * 1024 * 1024,
        "T" => 1024 * 1024 * 1024 * 1024,
        _ => return None,
    };

    Some(base * multiplier)
}
