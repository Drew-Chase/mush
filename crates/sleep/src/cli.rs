use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "sleep", about = "Pause for NUMBER seconds", version, disable_help_flag = true)]
pub struct SleepConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    pub args: Vec<String>,
}

impl SleepConfig {
    pub fn parse_duration(&self) -> f64 {
        if self.args.is_empty() {
            eprintln!("sleep: missing operand");
            return 0.0;
        }

        let mut total = 0.0;

        for arg in &self.args {
            let (num_str, multiplier) = if let Some(n) = arg.strip_suffix('d') {
                (n, 86400.0)
            } else if let Some(n) = arg.strip_suffix('h') {
                (n, 3600.0)
            } else if let Some(n) = arg.strip_suffix('m') {
                (n, 60.0)
            } else if let Some(n) = arg.strip_suffix('s') {
                (n, 1.0)
            } else {
                (arg.as_str(), 1.0)
            };

            let value: f64 = match num_str.parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("sleep: invalid time interval '{arg}'");
                    return 0.0;
                }
            };

            total += value * multiplier;
        }

        total
    }
}
