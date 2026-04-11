use std::process::ExitCode;
use whoami::cli::WhoamiConfig;
use whoami::ops::current_username;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(_config) = WhoamiConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

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
