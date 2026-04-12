use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    Single(usize),
    FromTo(usize, usize),
    From(usize),
    To(usize),
}

impl Range {
    /// Check whether a 1-based index is included in this range.
    pub fn contains(&self, idx: usize) -> bool {
        match self {
            Range::Single(n) => idx == *n,
            Range::FromTo(a, b) => idx >= *a && idx <= *b,
            Range::From(a) => idx >= *a,
            Range::To(b) => idx >= 1 && idx <= *b,
        }
    }
}

/// Parse a range specification like "1,3-5,7-,-3"
pub fn parse_ranges(spec: &str) -> Option<Vec<Range>> {
    let mut ranges = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return None;
        }
        if let Some(dash_pos) = part.find('-') {
            let left = &part[..dash_pos];
            let right = &part[dash_pos + 1..];
            if left.is_empty() && right.is_empty() {
                return None;
            } else if left.is_empty() {
                let b: usize = right.parse().ok()?;
                ranges.push(Range::To(b));
            } else if right.is_empty() {
                let a: usize = left.parse().ok()?;
                ranges.push(Range::From(a));
            } else {
                let a: usize = left.parse().ok()?;
                let b: usize = right.parse().ok()?;
                ranges.push(Range::FromTo(a, b));
            }
        } else {
            let n: usize = part.parse().ok()?;
            ranges.push(Range::Single(n));
        }
    }
    if ranges.is_empty() { None } else { Some(ranges) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CutMode {
    Bytes(Vec<Range>),
    Characters(Vec<Range>),
    Fields(Vec<Range>),
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "cut",
    about = "Print selected parts of lines from each FILE to standard output",
    version,
    disable_help_flag = true
)]
pub struct CutConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 'b', long = "bytes", allow_hyphen_values = true)]
    pub bytes_spec: Option<String>,

    #[arg(short = 'c', long = "characters", allow_hyphen_values = true)]
    pub characters_spec: Option<String>,

    #[arg(short = 'f', long = "fields", allow_hyphen_values = true)]
    pub fields_spec: Option<String>,

    #[arg(short = 'd', long = "delimiter", default_value = "\t")]
    pub delimiter: String,

    #[arg(long = "output-delimiter")]
    pub output_delimiter: Option<String>,

    #[arg(short = 's', long = "only-delimited")]
    pub only_delimited: bool,

    #[arg(long = "complement")]
    pub complement: bool,

    pub files: Vec<String>,

    /// Resolved cut mode (not set by clap, use resolve() after parsing)
    #[arg(skip)]
    pub mode: Option<CutMode>,
}

impl CutConfig {
    /// Resolve the cut mode from the raw parsed fields.
    /// Returns None if no mode was specified or if range parsing fails.
    pub fn resolve(&mut self) -> Option<()> {
        let mode = if let Some(ref spec) = self.bytes_spec {
            Some(CutMode::Bytes(parse_ranges(spec)?))
        } else if let Some(ref spec) = self.characters_spec {
            Some(CutMode::Characters(parse_ranges(spec)?))
        } else if let Some(ref spec) = self.fields_spec {
            Some(CutMode::Fields(parse_ranges(spec)?))
        } else {
            eprintln!("cut: you must specify a list of bytes, characters, or fields");
            return None;
        };
        self.mode = mode;
        Some(())
    }

    /// Get the delimiter as a char.
    pub fn delimiter_char(&self) -> char {
        self.delimiter.chars().next().unwrap_or('\t')
    }
}
