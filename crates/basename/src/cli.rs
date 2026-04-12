use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "basename",
    about = "Print NAME with any leading directory components removed.",
    version,
    disable_help_flag = true
)]
pub struct BasenameConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'a', long = "multiple", help = "Support multiple arguments and treat each as a NAME")]
    pub multiple: bool,

    #[arg(short = 's', long = "suffix", help = "Remove a trailing SUFFIX; implies -a")]
    pub suffix: Option<String>,

    #[arg(short = 'z', long = "zero", help = "End each output line with NUL, not newline")]
    pub zero: bool,

    pub names: Vec<String>,
}

impl BasenameConfig {
    /// Post-parse fixups to match traditional basename behavior:
    /// - When --suffix is given, imply --multiple
    /// - When not in multiple mode and two positional args given, second is suffix
    pub fn fixup(&mut self) {
        if self.suffix.is_some() {
            self.multiple = true;
        }
        if !self.multiple && self.names.len() == 2 {
            self.suffix = Some(self.names.remove(1));
        }
    }
}
