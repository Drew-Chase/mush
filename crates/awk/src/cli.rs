use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(
    name = "awk",
    about = "Pattern scanning and processing language.\n\nIf no file is given, or file is -, read standard input.",
    version,
    disable_help_flag = true
)]
pub struct AwkConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Use fs for the input field separator
    #[arg(short = 'F', long = "field-separator", default_value = "")]
    pub field_separator: String,

    /// Assign variable var=val (can be repeated)
    #[arg(short = 'v', long = "assign")]
    pub variables: Vec<String>,

    /// Read the AWK program source from progfile
    #[arg(short = 'f', long = "file")]
    pub program_file: Option<String>,

    /// Positional arguments: program text (first, if no -f) followed by input files
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub positionals: Vec<String>,
}

impl AwkConfig {
    /// Get the program text from positionals (first arg when no -f is given)
    pub fn program(&self) -> Option<&str> {
        if self.program_file.is_none() {
            self.positionals.first().map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get the input files from positionals
    pub fn files(&self) -> &[String] {
        if self.program_file.is_none() {
            // First positional is the program, rest are files
            if self.positionals.len() > 1 {
                &self.positionals[1..]
            } else {
                &[]
            }
        } else {
            &self.positionals
        }
    }
}
