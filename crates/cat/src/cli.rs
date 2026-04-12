use clap::Parser;

#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(name = "cat", about = "Concatenate FILE(s) to standard output", version, disable_help_flag = true)]
pub struct CatConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    #[arg(short = 'A', long = "show-all", help = "Equivalent to -vET")]
    pub show_all: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "Number nonempty output lines, overrides -n")]
    pub number_nonblank: bool,

    #[arg(short = 'e', help = "Equivalent to -vE")]
    pub e_compound: bool,

    #[arg(short = 'E', long = "show-ends", help = "Display $ at end of each line")]
    pub show_ends: bool,

    #[arg(short = 'n', long = "number", help = "Number all output lines")]
    pub number: bool,

    #[arg(short = 's', long = "squeeze-blank", help = "Suppress repeated empty output lines")]
    pub squeeze_blank: bool,

    #[arg(short = 't', help = "Equivalent to -vT")]
    pub t_compound: bool,

    #[arg(short = 'T', long = "show-tabs", help = "Display TAB characters as ^I")]
    pub show_tabs: bool,

    #[arg(short = 'v', long = "show-nonprinting", help = "Use ^ and M- notation, except for LFD and TAB")]
    pub show_nonprinting: bool,

    pub files: Vec<String>,
}

impl CatConfig {
    pub fn resolve(&mut self) {
        if self.show_all {
            self.show_nonprinting = true;
            self.show_ends = true;
            self.show_tabs = true;
        }
        if self.e_compound {
            self.show_nonprinting = true;
            self.show_ends = true;
        }
        if self.t_compound {
            self.show_nonprinting = true;
            self.show_tabs = true;
        }
    }
}
