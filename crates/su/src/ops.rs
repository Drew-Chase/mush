use std::process::ExitCode;

use crate::cli::SuConfig;

pub fn build_args(config: &SuConfig) -> Vec<String> {
    let mut args = Vec::new();

    if config.login {
        args.push("-l".to_string());
    }

    if let Some(ref shell) = config.shell {
        args.push("-s".to_string());
        args.push(shell.clone());
    }

    if let Some(ref cmd) = config.command {
        args.push("-c".to_string());
        args.push(cmd.clone());
    }

    args.push(config.user.clone());
    args
}

pub fn execute(config: &SuConfig) -> ExitCode {
    #[cfg(unix)]
    {
        let args = build_args(config);
        match std::process::Command::new("su").args(&args).status() {
            Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
            Err(e) => {
                eprintln!("su: {e}");
                ExitCode::from(1)
            }
        }
    }

    #[cfg(windows)]
    {
        let _ = config;
        eprintln!("su: not supported on Windows. Use 'runas' or an elevated terminal.");
        ExitCode::from(1)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = config;
        eprintln!("su: not supported on this platform");
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_args_default() {
        let config = SuConfig::default();
        let args = build_args(&config);
        assert_eq!(args, vec!["root"]);
    }

    #[test]
    fn test_build_args_with_login() {
        let config = SuConfig {
            login: true,
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-l", "root"]);
    }

    #[test]
    fn test_build_args_with_command() {
        let config = SuConfig {
            command: Some("whoami".to_string()),
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-c", "whoami", "root"]);
    }

    #[test]
    fn test_build_args_with_shell() {
        let config = SuConfig {
            shell: Some("/bin/zsh".to_string()),
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-s", "/bin/zsh", "root"]);
    }

    #[test]
    fn test_build_args_custom_user() {
        let config = SuConfig {
            user: "admin".to_string(),
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["admin"]);
    }

    #[test]
    fn test_build_args_all_options() {
        let config = SuConfig {
            command: Some("id".to_string()),
            login: true,
            shell: Some("/bin/bash".to_string()),
            user: "nobody".to_string(),
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-l", "-s", "/bin/bash", "-c", "id", "nobody"]);
    }
}
