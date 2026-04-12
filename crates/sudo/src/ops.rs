use std::process::ExitCode;

use crate::cli::SudoConfig;

pub fn build_args(config: &SudoConfig) -> Vec<String> {
    let mut args = Vec::new();

    if let Some(ref user) = config.user {
        args.push("-u".to_string());
        args.push(user.clone());
    }

    if config.login {
        args.push("-i".to_string());
    }

    if config.shell {
        args.push("-s".to_string());
    }

    if config.preserve_env {
        args.push("-E".to_string());
    }

    args.extend(config.command.clone());
    args
}

pub fn execute(config: &SudoConfig) -> ExitCode {
    #[cfg(unix)]
    {
        let args = build_args(config);
        match std::process::Command::new("sudo").args(&args).status() {
            Ok(status) => {
                ExitCode::from(status.code().unwrap_or(1) as u8)
            }
            Err(e) => {
                eprintln!("sudo: {e}");
                ExitCode::from(1)
            }
        }
    }

    #[cfg(windows)]
    {
        let _ = config;
        eprintln!("sudo: not supported on Windows. Use an elevated terminal.");
        ExitCode::from(1)
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = config;
        eprintln!("sudo: not supported on this platform");
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_args_simple_command() {
        let config = SudoConfig {
            command: vec!["ls".to_string(), "-la".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["ls", "-la"]);
    }

    #[test]
    fn test_build_args_with_user() {
        let config = SudoConfig {
            user: Some("root".to_string()),
            command: vec!["whoami".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-u", "root", "whoami"]);
    }

    #[test]
    fn test_build_args_with_login() {
        let config = SudoConfig {
            login: true,
            command: vec!["bash".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-i", "bash"]);
    }

    #[test]
    fn test_build_args_with_shell() {
        let config = SudoConfig {
            shell: true,
            command: vec!["echo".to_string(), "hello".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-s", "echo", "hello"]);
    }

    #[test]
    fn test_build_args_with_preserve_env() {
        let config = SudoConfig {
            preserve_env: true,
            command: vec!["env".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-E", "env"]);
    }

    #[test]
    fn test_build_args_all_options() {
        let config = SudoConfig {
            user: Some("admin".to_string()),
            login: true,
            shell: true,
            preserve_env: true,
            command: vec!["cmd".to_string()],
            ..Default::default()
        };
        let args = build_args(&config);
        assert_eq!(args, vec!["-u", "admin", "-i", "-s", "-E", "cmd"]);
    }

    #[test]
    fn test_build_args_no_command() {
        let config = SudoConfig::default();
        let args = build_args(&config);
        assert!(args.is_empty());
    }
}
