use clap::Parser;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "seq",
    about = "Print numbers from FIRST to LAST, in steps of INCREMENT.",
    version,
    disable_help_flag = true
)]
pub struct SeqConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// First value (resolved from positional args)
    #[arg(skip = 1.0)]
    pub first: f64,

    /// Increment value (resolved from positional args)
    #[arg(skip = 1.0)]
    pub increment: f64,

    /// Last value (resolved from positional args)
    #[arg(skip)]
    pub last: f64,

    /// Use STRING to separate numbers (default: \n)
    #[arg(short = 's', long = "separator", default_value = "\n")]
    pub separator: String,

    /// Use printf style floating-point FORMAT
    #[arg(short = 'f', long = "format")]
    pub format: Option<String>,

    /// Equalize width by padding with leading zeroes
    #[arg(short = 'w', long = "equal-width")]
    pub equal_width: bool,

    /// Positional arguments: LAST, FIRST LAST, or FIRST INCREMENT LAST
    #[arg(required = true)]
    pub args: Vec<String>,
}

impl SeqConfig {
    pub fn resolve(&mut self) -> Result<(), String> {
        let (first, increment, last) = match self.args.len() {
            1 => {
                let last = parse_number(&self.args[0])?;
                (1.0, 1.0, last)
            }
            2 => {
                let first = parse_number(&self.args[0])?;
                let last = parse_number(&self.args[1])?;
                (first, 1.0, last)
            }
            3 => {
                let first = parse_number(&self.args[0])?;
                let increment = parse_number(&self.args[1])?;
                let last = parse_number(&self.args[2])?;
                (first, increment, last)
            }
            0 => return Err("seq: missing operand".to_string()),
            _ => return Err("seq: extra operand".to_string()),
        };
        self.first = first;
        self.increment = increment;
        self.last = last;
        Ok(())
    }
}

fn parse_number(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| format!("seq: invalid floating point argument: '{s}'"))
}
