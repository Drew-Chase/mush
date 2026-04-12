use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "shuf",
    about = "Write a random permutation of the input lines.",
    version,
    disable_help_flag = true
)]
pub struct ShufConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Treat each ARG as an input line (resolved after parse)
    #[arg(skip)]
    pub echo_args: Vec<String>,

    /// Treat each ARG as an input line
    #[arg(short = 'e', long = "echo")]
    pub echo_mode: bool,

    /// Treat each number LO through HI as an input line (resolved after parse)
    #[arg(skip)]
    pub range: Option<(u64, u64)>,

    /// Range LO-HI (e.g. 1-10)
    #[arg(short = 'i', long = "input-range")]
    pub range_str: Option<String>,

    /// Output at most COUNT lines
    #[arg(short = 'n', long = "head-count")]
    pub head_count: Option<usize>,

    /// Output lines can be repeated (with -n)
    #[arg(short = 'r', long = "repeat")]
    pub repeat: bool,

    /// File to read (resolved after parse)
    #[arg(skip)]
    pub file: Option<String>,

    /// Positional arguments
    #[arg(trailing_var_arg = true)]
    pub positional: Vec<String>,
}

impl ShufConfig {
    pub fn resolve(&mut self) -> Result<(), String> {
        if let Some(ref range_str) = self.range_str {
            let (lo, hi) = range_str
                .split_once('-')
                .ok_or_else(|| format!("shuf: invalid input range: '{range_str}'"))?;
            let lo: u64 = lo
                .parse()
                .map_err(|_| format!("shuf: invalid input range: '{range_str}'"))?;
            let hi: u64 = hi
                .parse()
                .map_err(|_| format!("shuf: invalid input range: '{range_str}'"))?;
            self.range = Some((lo, hi));
        }

        if self.echo_mode {
            self.echo_args = std::mem::take(&mut self.positional);
        } else {
            self.file = self.positional.first().cloned();
        }

        Ok(())
    }
}
