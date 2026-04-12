use clap::Parser;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SortKey {
    pub start_field: usize,
    pub end_field: Option<usize>,
}

impl SortKey {
    pub fn parse(s: &str) -> Option<Self> {
        if let Some((start, end)) = s.split_once(',') {
            let start_field = start.parse::<usize>().ok()?;
            let end_field = end.parse::<usize>().ok()?;
            if start_field == 0 || end_field == 0 {
                return None;
            }
            Some(SortKey { start_field, end_field: Some(end_field) })
        } else {
            let start_field = s.parse::<usize>().ok()?;
            if start_field == 0 {
                return None;
            }
            Some(SortKey { start_field, end_field: None })
        }
    }
}

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "sort",
    about = "Write sorted concatenation of all FILE(s) to standard output.",
    version,
    disable_help_flag = true
)]
pub struct SortConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Reverse the result of comparisons
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// Compare according to string numerical value
    #[arg(short = 'n', long = "numeric-sort")]
    pub numeric: bool,

    /// Compare human readable numbers (e.g., 2K 1G)
    #[arg(short = 'h', long = "human-numeric-sort")]
    pub human_numeric: bool,

    /// Fold lower case to upper case characters
    #[arg(short = 'f', long = "ignore-case")]
    pub ignore_case: bool,

    /// Consider only blanks and alphanumeric characters
    #[arg(short = 'd', long = "dictionary-order")]
    pub dictionary: bool,

    /// Ignore leading blanks
    #[arg(short = 'b', long = "ignore-leading-blanks")]
    pub ignore_blanks: bool,

    /// Sort via a key; KEYDEF gives location and type
    #[arg(short = 'k', long = "key")]
    pub key_strs: Vec<String>,

    /// Resolved sort keys (from key_strs)
    #[arg(skip)]
    pub key: Vec<SortKey>,

    /// Use SEP instead of non-blank to blank transition
    #[arg(short = 't', long = "field-separator")]
    pub separator: Option<char>,

    /// Output only the first of an equal run
    #[arg(short = 'u', long = "unique")]
    pub unique: bool,

    /// Stabilize sort by disabling last-resort comparison
    #[arg(short = 's', long = "stable")]
    pub stable: bool,

    /// Write result to FILE instead of standard output
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Check for sorted input; do not sort
    #[arg(short = 'c', long = "check")]
    pub check: bool,

    /// Merge already sorted files; do not sort
    #[arg(short = 'm', long = "merge")]
    pub merge: bool,

    /// Files to sort
    pub files: Vec<String>,
}

impl SortConfig {
    pub fn resolve(&mut self) -> Result<(), String> {
        for s in &self.key_strs {
            match SortKey::parse(s) {
                Some(k) => self.key.push(k),
                None => return Err(format!("sort: invalid key specification: '{s}'")),
            }
        }
        Ok(())
    }
}
