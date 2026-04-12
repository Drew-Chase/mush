use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "file", about = "Determine type of FILEs", version, disable_help_flag = true)]
pub struct FileConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'b', long, help = "Do not prepend filenames to output lines")]
    pub brief: bool,

    #[arg(short = 'i', long, help = "Output MIME type strings")]
    pub mime: bool,

    #[arg(long, help = "Output the MIME type only")]
    pub mime_type: bool,

    #[arg(short = 'L', long, help = "Follow symlinks")]
    pub dereference: bool,

    #[arg(required = true)]
    pub files: Vec<String>,
}
