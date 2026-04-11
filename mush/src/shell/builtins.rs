use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum BuiltinCommand {
    Cd,
    Clear,
    Exit,
    Scripts,
}

pub struct BuiltinResult {
    pub output: Vec<String>,
    pub exit_app: bool,
    pub change_dir: Option<PathBuf>,
    pub exit_code: i32,
}

pub fn lookup(name: &str) -> Option<BuiltinCommand> {
    match name.to_lowercase().as_str() {
        "cd" => Some(BuiltinCommand::Cd),
        "clear" | "cls" => Some(BuiltinCommand::Clear),
        "exit" => Some(BuiltinCommand::Exit),
        "scripts" => Some(BuiltinCommand::Scripts),
        _ => None,
    }
}

pub fn execute(cmd: BuiltinCommand, args: &[String]) -> BuiltinResult {
    match cmd {
        BuiltinCommand::Cd => execute_cd(args),
        BuiltinCommand::Clear => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: None,
            exit_code: 0,
        },
        BuiltinCommand::Exit => BuiltinResult {
            output: Vec::new(),
            exit_app: true,
            change_dir: None,
            exit_code: 0,
        },
        BuiltinCommand::Scripts => execute_scripts(args),
    }
}

fn execute_cd(args: &[String]) -> BuiltinResult {
    let err = |msg: String| BuiltinResult {
        output: vec![msg],
        exit_app: false,
        change_dir: None,
        exit_code: 1,
    };

    let target = if args.is_empty() {
        match home_dir() {
            Some(home) => home,
            None => return err("cd: could not determine home directory".to_string()),
        }
    } else {
        let path_str = &args[0];
        if path_str == "~" {
            match home_dir() {
                Some(home) => home,
                None => return err("cd: could not determine home directory".to_string()),
            }
        } else if let Some(rest) = path_str.strip_prefix("~/").or_else(|| path_str.strip_prefix("~\\")) {
            match home_dir() {
                Some(home) => home.join(rest),
                None => return err("cd: could not determine home directory".to_string()),
            }
        } else {
            PathBuf::from(path_str)
        }
    };

    match std::env::set_current_dir(&target) {
        Ok(()) => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: Some(std::env::current_dir().unwrap_or(target)),
            exit_code: 0,
        },
        Err(e) => err(format!("cd: {}: {}", target.display(), e)),
    }
}

fn ok(output: Vec<String>) -> BuiltinResult {
    BuiltinResult { output, exit_app: false, change_dir: None, exit_code: 0 }
}

fn fail(msg: String) -> BuiltinResult {
    BuiltinResult { output: vec![msg], exit_app: false, change_dir: None, exit_code: 1 }
}

fn execute_scripts(args: &[String]) -> BuiltinResult {
    let subcommand = args.first().map(|s| s.as_str()).unwrap_or("");

    match subcommand {
        "new" => {
            let name = match args.get(1) {
                Some(n) if !n.is_empty() => n,
                _ => return fail("Usage: scripts new <name>".to_string()),
            };

            let scripts_dir = crate::get_appdata_path().join("scripts");
            let project_dir = scripts_dir.join(name);

            if project_dir.exists() {
                return fail(format!("scripts: '{}' already exists", name));
            }

            if let Err(e) = std::fs::create_dir_all(&project_dir) {
                return fail(format!("scripts: failed to create directory: {e}"));
            }

            let package_json = format!(
                r#"{{
  "name": "{name}",
  "description": "A mush script",
  "main": "index.ts",
  "scripts": {{
    "start": "bun run index.ts"
  }},
  "dependencies": {{
    "commander": "^13.0.0"
  }},
  "devDependencies": {{
    "@types/node": "^22.0.0",
    "typescript": "^5.0.0"
  }}
}}"#
            );

            let index_ts = format!(
                r#"import {{ Command }} from "commander";

const program = new Command();

program
  .name("{name}")
  .description("A mush script")
  .version("1.0.0")
  .option("-m, --message <text>", "message to display")
  .action((options) => {{
    console.log(options.message ?? "Hello from {name}!");
  }});

program.parse();
"#
            );

            if let Err(e) = std::fs::write(project_dir.join("package.json"), &package_json) {
                return fail(format!("scripts: failed to write package.json: {e}"));
            }

            if let Err(e) = std::fs::write(project_dir.join("index.ts"), &index_ts) {
                return fail(format!("scripts: failed to write index.ts: {e}"));
            }

            let mut output = vec![
                format!("Created script '{}' at {}", name, project_dir.display()),
            ];

            // Run bun install if bun is available
            match super::path_resolver::find_in_path("bun") {
                Some(bun_path) => {
                    output.push("Installing dependencies...".to_string());
                    match std::process::Command::new(&bun_path)
                        .arg("install")
                        .current_dir(&project_dir)
                        .output()
                    {
                        Ok(result) => {
                            if result.status.success() {
                                output.push("Dependencies installed.".to_string());
                            } else {
                                let stderr = String::from_utf8_lossy(&result.stderr);
                                output.push(format!("Warning: bun install failed: {}", stderr.trim()));
                            }
                        }
                        Err(e) => {
                            output.push(format!("Warning: failed to run bun install: {e}"));
                        }
                    }
                }
                None => {
                    output.push("Bun not found on PATH — run 'bun install' manually in the script directory.".to_string());
                }
            }

            // Re-scan scripts so the new one is immediately available
            super::script_registry::scan_scripts(&scripts_dir);
            output.push(format!("Script '{}' is now available as a command.", name));

            ok(output)
        }
        "reload" => {
            let scripts_dir = crate::get_appdata_path().join("scripts");
            super::script_registry::scan_scripts(&scripts_dir);
            let count = super::script_registry::list_scripts().len();
            ok(vec![format!("Reloaded {count} script(s).")])
        }
        _ => ok(vec![
            "Usage: scripts <command>".to_string(),
            "".to_string(),
            "Commands:".to_string(),
            "  new <name>   Create a new script from template".to_string(),
            "  reload       Reload all scripts from the scripts directory".to_string(),
        ]),
    }
}

pub(crate) fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
