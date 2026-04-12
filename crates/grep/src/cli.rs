use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "grep", about = "Search for PATTERN in each FILE or standard input", version, disable_help_flag = true)]
pub struct GrepConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    pub pattern: String,

    pub files: Vec<String>,

    #[arg(short = 'i', long = "ignore-case", help = "Ignore case distinctions in patterns and data")]
    pub ignore_case: bool,

    #[arg(short = 'v', long = "invert-match", help = "Select non-matching lines")]
    pub invert: bool,

    #[arg(short = 'w', long = "word-regexp", help = "Match only whole words")]
    pub word_regexp: bool,

    #[arg(short = 'x', long = "line-regexp", help = "Match only whole lines")]
    pub line_regexp: bool,

    #[arg(short = 'c', long, help = "Print only a count of selected lines per FILE")]
    pub count: bool,

    #[arg(short = 'l', long = "files-with-matches", help = "Print only names of FILEs with selected lines")]
    pub files_with_matches: bool,

    #[arg(short = 'L', long = "files-without-match", help = "Print only names of FILEs with no selected lines")]
    pub files_without_match: bool,

    #[arg(short = 'n', long = "line-number", help = "Prefix each output line with line number")]
    pub line_number: bool,

    #[arg(short = 'H', long = "with-filename", help = "Print file name with output lines")]
    pub with_filename: bool,

    #[arg(short = 'h', long = "no-filename", help = "Suppress the file name prefix on output")]
    pub no_filename: bool,

    #[arg(short = 'o', long = "only-matching", help = "Show only nonempty parts of lines that match")]
    pub only_matching: bool,

    #[arg(short = 'q', long = "quiet", visible_alias = "silent", help = "Suppress all normal output")]
    pub quiet: bool,

    #[arg(short = 'r', short_alias = 'R', long, help = "Search directories recursively")]
    pub recursive: bool,

    #[arg(short = 'A', long = "after-context", default_value = "0", help = "Print NUM lines of trailing context")]
    pub after_context: usize,

    #[arg(short = 'B', long = "before-context", default_value = "0", help = "Print NUM lines of leading context")]
    pub before_context: usize,

    #[arg(short = 'C', long = "context", default_value = "0", help = "Print NUM lines of output context")]
    pub context: usize,

    #[arg(short = 'm', long = "max-count", help = "Stop after NUM selected lines")]
    pub max_count: Option<usize>,

    #[arg(long, num_args = 0..=1, default_missing_value = "always", help = "Use markers to highlight the matching strings")]
    pub color: Option<String>,

    #[arg(short = 'F', long = "fixed-strings", help = "PATTERN is a set of newline-separated strings")]
    pub fixed_strings: bool,

    #[arg(short = 'E', long = "extended-regexp", help = "PATTERN is an extended regular expression")]
    pub extended_regexp: bool,

    #[arg(long = "include", help = "Search only files that match GLOB")]
    pub include_glob: Vec<String>,

    #[arg(long = "exclude", help = "Skip files that match GLOB")]
    pub exclude_glob: Vec<String>,

    #[arg(long = "exclude-dir", help = "Skip directories that match DIR")]
    pub exclude_dir: Vec<String>,
}

impl GrepConfig {
    pub fn color_enabled(&self) -> bool {
        match self.color.as_deref() {
            Some("always") => true,
            Some("never") => false,
            Some("auto") => false,
            Some(_) => false,
            None => false,
        }
    }
}
