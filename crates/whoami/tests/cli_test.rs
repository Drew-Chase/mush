use clap::Parser;

use whoami::cli::WhoamiConfig;

#[test]
fn no_args() {
    let _config = WhoamiConfig::parse_from(["whoami"]);
}

#[test]
fn help_returns_err() {
    let result = WhoamiConfig::try_parse_from(["whoami", "--help"]);
    assert!(result.is_err());
}

#[test]
fn version_returns_err() {
    let result = WhoamiConfig::try_parse_from(["whoami", "--version"]);
    assert!(result.is_err());
}
