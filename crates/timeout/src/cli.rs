use clap::Parser;

/// Parse a duration string like "5", "1m", "0.5s", "2h", "1d" into seconds.
pub fn parse_duration(s: &str) -> Option<f64> {
    let (num_str, multiplier) = if let Some(n) = s.strip_suffix('d') {
        (n, 86400.0)
    } else if let Some(n) = s.strip_suffix('h') {
        (n, 3600.0)
    } else if let Some(n) = s.strip_suffix('m') {
        (n, 60.0)
    } else if let Some(n) = s.strip_suffix('s') {
        (n, 1.0)
    } else {
        (s, 1.0)
    };

    num_str.parse::<f64>().ok().map(|v| v * multiplier)
}

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "timeout",
    about = "Start COMMAND, and kill it if still running after DURATION.",
    version,
    disable_help_flag = true
)]
pub struct TimeoutConfig {
    #[arg(long = "help", action = clap::ArgAction::Help, help = "Print help")]
    pub help: Option<bool>,

    /// Duration in seconds (resolved from positional duration arg)
    #[arg(skip)]
    pub duration_secs: f64,

    /// Specify the signal to be sent on timeout
    #[arg(short = 's', long = "signal", default_value = "TERM")]
    pub signal: String,

    /// Also send a KILL signal if COMMAND is still running this long after initial signal
    #[arg(short = 'k', long = "kill-after")]
    pub kill_after_str: Option<String>,

    /// Resolved kill-after duration in seconds
    #[arg(skip)]
    pub kill_after: Option<f64>,

    /// Exit with the same status as COMMAND, even when it times out
    #[arg(long = "preserve-status")]
    pub preserve_status: bool,

    /// Diagnose to stderr any signal sent upon timeout
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Resolved command (after duration)
    #[arg(skip)]
    pub command: Vec<String>,

    /// Positional: DURATION COMMAND [ARG]...
    #[arg(required = true, trailing_var_arg = true)]
    pub args: Vec<String>,
}

impl TimeoutConfig {
    pub fn resolve(&mut self) -> Result<(), String> {
        if let Some(ref ka) = self.kill_after_str {
            self.kill_after = Some(
                parse_duration(ka)
                    .ok_or_else(|| format!("timeout: invalid duration '{ka}'"))?,
            );
        }

        if self.args.is_empty() {
            return Err("timeout: missing operand".to_string());
        }

        self.duration_secs = parse_duration(&self.args[0])
            .ok_or_else(|| format!("timeout: invalid time interval '{}'", self.args[0]))?;

        if self.args.len() < 2 {
            return Err("timeout: missing operand".to_string());
        }

        self.command = self.args[1..].to_vec();
        Ok(())
    }
}
