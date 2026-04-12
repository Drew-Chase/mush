use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "column",
    about = "Columnate lists",
    version,
    disable_help_flag = true
)]
pub struct ColumnConfig {
    #[arg(long = "help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(short = 't', long = "table")]
    pub table: bool,

    #[arg(short = 's', long = "separator")]
    pub separator: Option<String>,

    #[arg(short = 'o', long = "output-separator", default_value = "  ")]
    pub output_separator: String,

    #[arg(short = 'c', long = "output-width")]
    pub width: Option<usize>,

    #[arg(short = 'N', long = "table-columns")]
    pub column_names: Option<String>,

    #[arg(short = 'R', long = "table-right")]
    pub right_align: Option<String>,

    #[arg(short = 'J', long = "json")]
    pub json: bool,

    pub files: Vec<String>,
}
