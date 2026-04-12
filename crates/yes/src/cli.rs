use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(name = "yes", about = "Repeatedly output a line with all specified STRING(s), or 'y'", version, disable_help_flag = true)]
pub struct YesConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    pub args: Vec<String>,
}

impl YesConfig {
    pub fn string(&self) -> String {
        if self.args.is_empty() {
            "y".to_string()
        } else {
            self.args.join(" ")
        }
    }
}
