use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "diff", about = "Compare files line by line", version, disable_help_flag = true)]
pub struct DiffConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'u', long, num_args = 0..=1, default_missing_value = "3", require_equals = true, help = "Output NUM (default 3) lines of unified context")]
    pub unified: Option<usize>,

    #[arg(short = 'c', long, num_args = 0..=1, default_missing_value = "3", require_equals = true, help = "Output NUM (default 3) lines of copied context")]
    pub context: Option<usize>,

    #[arg(short = 'y', long = "side-by-side", help = "Output in two columns")]
    pub side_by_side: bool,

    #[arg(short = 'W', long, default_value = "130", help = "Output at most NUM (default 130) print columns")]
    pub width: usize,

    #[arg(short = 'i', long = "ignore-case", help = "Ignore case differences in file contents")]
    pub ignore_case: bool,

    #[arg(short = 'b', long = "ignore-space-change", help = "Ignore changes in the amount of white space")]
    pub ignore_space_change: bool,

    #[arg(short = 'w', long = "ignore-all-space", help = "Ignore all white space")]
    pub ignore_all_space: bool,

    #[arg(short = 'B', long = "ignore-blank-lines", help = "Ignore changes where lines are all blank")]
    pub ignore_blank_lines: bool,

    #[arg(short = 'r', long, help = "Recursively compare any subdirectories found")]
    pub recursive: bool,

    #[arg(short = 'q', long, help = "Report only when files differ")]
    pub brief: bool,

    #[arg(short = 's', long = "report-identical-files", help = "Report when two files are identical")]
    pub report_identical: bool,

    #[arg(long, help = "Colorize the output")]
    pub color: bool,

    pub file1: String,

    pub file2: String,
}
