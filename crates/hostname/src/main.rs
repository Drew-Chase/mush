use std::process::ExitCode;

use clap::Parser;

use hostname::cli::HostnameConfig;
use hostname::ops::get_hostname;

fn main() -> ExitCode {
    let config = HostnameConfig::parse();

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
