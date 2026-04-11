use std::process::ExitCode;

use hostname::cli::HostnameConfig;
use hostname::ops::get_hostname;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = HostnameConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    match get_hostname(&config) {
        Ok(name) => {
            println!("{name}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("hostname: {e}");
            ExitCode::FAILURE
        }
    }
}
