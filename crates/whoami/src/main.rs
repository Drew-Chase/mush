use std::process::ExitCode;

use clap::Parser;

use whoami::cli::WhoamiConfig;
use whoami::ops::current_username;

fn main() -> ExitCode {
    let _config = WhoamiConfig::parse();

    match current_username() {
        Ok(name) => {
            println!("{name}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("whoami: {e}");
            ExitCode::FAILURE
        }
    }
}
