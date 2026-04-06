use std::io::{self, Write};
use std::process::ExitCode;

use echo::cli::EchoConfig;
use echo::escape::process_escapes;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = EchoConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for (i, arg) in config.args.iter().enumerate() {
        if i > 0 {
            let _ = write!(out, " ");
        }

        if config.interpret_escapes {
            let (processed, stop) = process_escapes(arg);
            let _ = write!(out, "{processed}");
            if stop {
                let _ = out.flush();
                return ExitCode::SUCCESS;
            }
        } else {
            let _ = write!(out, "{arg}");
        }
    }

    if !config.no_newline {
        let _ = writeln!(out);
    }

    let _ = out.flush();
    ExitCode::SUCCESS
}
