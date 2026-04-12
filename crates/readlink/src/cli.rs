use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "readlink",
    about = "Print value of a symbolic link or canonical file name.",
    version,
    disable_help_flag = true
)]
pub struct ReadlinkConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'f', long = "canonicalize", help = "Canonicalize by following every symlink recursively; all but the last component must exist")]
    pub canonicalize: bool,

    #[arg(short = 'e', long = "canonicalize-existing", help = "Canonicalize by following every symlink recursively, all components must exist")]
    pub canonicalize_existing: bool,

    #[arg(short = 'm', long = "canonicalize-missing", help = "Canonicalize by following every symlink recursively, without requirements on components existence")]
    pub canonicalize_missing: bool,

    #[arg(short = 'n', long = "no-newline", help = "Do not output the trailing delimiter")]
    pub no_newline: bool,

    #[arg(short = 'z', long = "zero", help = "End each output line with NUL, not newline")]
    pub zero: bool,

    pub files: Vec<String>,
}
