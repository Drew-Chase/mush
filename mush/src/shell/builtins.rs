use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static DIR_STACK: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
static SHELL_OPTIONS: Mutex<Option<HashSet<String>>> = Mutex::new(None);

#[derive(Debug, Clone, Copy)]
pub enum BuiltinCommand {
    Cd,
    Clear,
    Exit,
    Scripts,
    Pwd,
    Export,
    Unset,
    Printf,
    Env,
    Alias,
    Unalias,
    Type,
    History,
    Source,
    Read,
    Test,
    True,
    False,
    Printenv,
    Pushd,
    Popd,
    Set,
    Jobs,
    Fg,
    Bg,
    Dirs,
    Wait,
    Expr,
    Umask,
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
        "pwd" => Some(BuiltinCommand::Pwd),
        "export" => Some(BuiltinCommand::Export),
        "unset" => Some(BuiltinCommand::Unset),
        "printf" => Some(BuiltinCommand::Printf),
        "env" => Some(BuiltinCommand::Env),
        "alias" => Some(BuiltinCommand::Alias),
        "unalias" => Some(BuiltinCommand::Unalias),
        "type" | "which" => Some(BuiltinCommand::Type),
        "history" => Some(BuiltinCommand::History),
        "source" | "." => Some(BuiltinCommand::Source),
        "read" => Some(BuiltinCommand::Read),
        "test" | "[" => Some(BuiltinCommand::Test),
        "true" => Some(BuiltinCommand::True),
        "false" => Some(BuiltinCommand::False),
        "printenv" => Some(BuiltinCommand::Printenv),
        "pushd" => Some(BuiltinCommand::Pushd),
        "popd" => Some(BuiltinCommand::Popd),
        "set" => Some(BuiltinCommand::Set),
        "jobs" => Some(BuiltinCommand::Jobs),
        "fg" => Some(BuiltinCommand::Fg),
        "bg" => Some(BuiltinCommand::Bg),
        "dirs" => Some(BuiltinCommand::Dirs),
        "wait" => Some(BuiltinCommand::Wait),
        "expr" => Some(BuiltinCommand::Expr),
        "umask" => Some(BuiltinCommand::Umask),
        _ => None,
    }
}

pub fn execute(cmd: BuiltinCommand, args: &[String]) -> BuiltinResult {
    match cmd {
        BuiltinCommand::Cd => execute_cd(args),
        BuiltinCommand::Clear => ok(Vec::new()),
        BuiltinCommand::Exit => BuiltinResult {
            output: Vec::new(),
            exit_app: true,
            change_dir: None,
            exit_code: 0,
        },
        BuiltinCommand::Scripts => execute_scripts(args),
        BuiltinCommand::Pwd => execute_pwd(args),
        BuiltinCommand::Export => execute_export(args),
        BuiltinCommand::Unset => execute_unset(args),
        BuiltinCommand::Printf => execute_printf(args),
        BuiltinCommand::Env => execute_env(args),
        BuiltinCommand::Alias => execute_alias(args),
        BuiltinCommand::Unalias => execute_unalias(args),
        BuiltinCommand::Type => execute_type(args),
        BuiltinCommand::History => execute_history(args),
        BuiltinCommand::Source => execute_source(args),
        BuiltinCommand::Read => execute_read(args),
        BuiltinCommand::Test => execute_test(args),
        BuiltinCommand::True => ok(Vec::new()),
        BuiltinCommand::False => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: None,
            exit_code: 1,
        },
        BuiltinCommand::Printenv => execute_printenv(args),
        BuiltinCommand::Pushd => execute_pushd(args),
        BuiltinCommand::Popd => execute_popd(args),
        BuiltinCommand::Set => execute_set(args),
        // Jobs/fg/bg/wait are handled as special cases in widgets/mod.rs
        BuiltinCommand::Jobs | BuiltinCommand::Fg | BuiltinCommand::Bg | BuiltinCommand::Wait => {
            ok(Vec::new())
        }
        BuiltinCommand::Dirs => execute_dirs(args),
        BuiltinCommand::Expr => execute_expr(args),
        BuiltinCommand::Umask => execute_umask(args),
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

// ── pwd ─────────────────────────────────────────────────────────────────────

fn execute_pwd(args: &[String]) -> BuiltinResult {
    let physical = args.iter().any(|a| a == "-P" || a == "--physical");
    match std::env::current_dir() {
        Ok(cwd) => {
            let path = if physical {
                std::fs::canonicalize(&cwd).unwrap_or(cwd)
            } else {
                cwd
            };
            ok(vec![path.to_string_lossy().to_string()])
        }
        Err(e) => fail(format!("pwd: {e}")),
    }
}

// ── export ──────────────────────────────────────────────────────────────────

fn execute_export(args: &[String]) -> BuiltinResult {
    if args.is_empty() || args.iter().any(|a| a == "-p") {
        let mut vars: Vec<(String, String)> = std::env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        let lines: Vec<String> = vars
            .iter()
            .map(|(k, v)| format!("declare -x {k}=\"{v}\""))
            .collect();
        return ok(lines);
    }

    for arg in args {
        if arg == "-p" || arg == "-n" {
            continue;
        }
        if let Some((key, value)) = arg.split_once('=')
            && !key.is_empty()
        {
            // SAFETY: mush is single-threaded for command execution
            unsafe { std::env::set_var(key, value) };
        }
        // `export VAR` without = is a no-op (var is already in env if set)
    }
    ok(Vec::new())
}

// ── unset ───────────────────────────────────────────────────────────────────

fn execute_unset(args: &[String]) -> BuiltinResult {
    for arg in args {
        if arg == "-v" || arg == "-f" {
            continue;
        }
        // SAFETY: mush is single-threaded for command execution
        unsafe { std::env::remove_var(arg) };
    }
    ok(Vec::new())
}

// ── printf ──────────────────────────────────────────────────────────────────

fn execute_printf(args: &[String]) -> BuiltinResult {
    if args.is_empty() {
        return fail("printf: usage: printf FORMAT [ARGUMENT]...".to_string());
    }

    let format = &args[0];
    let params = &args[1..];
    let mut param_idx = 0;
    let mut output = String::new();
    let mut chars = format.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                match chars.next() {
                    Some('n') => output.push('\n'),
                    Some('t') => output.push('\t'),
                    Some('r') => output.push('\r'),
                    Some('\\') => output.push('\\'),
                    Some('0') => {
                        // Octal: \0NNN
                        let mut oct = String::new();
                        for _ in 0..3 {
                            if let Some(&d) = chars.peek() {
                                if d.is_ascii_digit() && d != '8' && d != '9' {
                                    oct.push(d);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                        }
                        if let Ok(val) = u8::from_str_radix(&oct, 8) {
                            output.push(val as char);
                        }
                    }
                    Some(other) => {
                        output.push('\\');
                        output.push(other);
                    }
                    None => output.push('\\'),
                }
            }
            '%' => {
                let param = if param_idx < params.len() {
                    &params[param_idx]
                } else {
                    ""
                };
                param_idx += 1;

                match chars.next() {
                    Some('s') => output.push_str(param),
                    Some('d') | Some('i') => {
                        let n: i64 = param.parse().unwrap_or(0);
                        output.push_str(&n.to_string());
                    }
                    Some('u') => {
                        let n: u64 = param.parse().unwrap_or(0);
                        output.push_str(&n.to_string());
                    }
                    Some('f') => {
                        let f: f64 = param.parse().unwrap_or(0.0);
                        output.push_str(&format!("{f:.6}"));
                    }
                    Some('e') => {
                        let f: f64 = param.parse().unwrap_or(0.0);
                        output.push_str(&format!("{f:e}"));
                    }
                    Some('x') => {
                        let n: i64 = param.parse().unwrap_or(0);
                        output.push_str(&format!("{n:x}"));
                    }
                    Some('X') => {
                        let n: i64 = param.parse().unwrap_or(0);
                        output.push_str(&format!("{n:X}"));
                    }
                    Some('o') => {
                        let n: i64 = param.parse().unwrap_or(0);
                        output.push_str(&format!("{n:o}"));
                    }
                    Some('c') => {
                        if let Some(ch) = param.chars().next() {
                            output.push(ch);
                        }
                    }
                    Some('b') => {
                        // Interpret backslash escapes in the argument
                        output.push_str(&interpret_escapes(param));
                    }
                    Some('%') => {
                        param_idx -= 1; // %% doesn't consume a parameter
                        output.push('%');
                    }
                    Some(other) => {
                        output.push('%');
                        output.push(other);
                        param_idx -= 1;
                    }
                    None => output.push('%'),
                }
            }
            _ => output.push(c),
        }
    }

    let lines: Vec<String> = output.lines().map(String::from).collect();
    ok(if lines.is_empty() && !output.is_empty() {
        vec![output]
    } else {
        lines
    })
}

fn interpret_escapes(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

// ── env ─────────────────────────────────────────────────────────────────────

fn execute_env(args: &[String]) -> BuiltinResult {
    let mut unset_vars = Vec::new();
    let mut ignore_env = false;
    let mut set_vars = Vec::new();
    let mut cmd_start = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-i" || arg == "--ignore-environment" {
            ignore_env = true;
        } else if arg == "-u" || arg == "--unset" {
            i += 1;
            if i < args.len() {
                unset_vars.push(args[i].clone());
            }
        } else if let Some((key, value)) = arg.split_once('=') {
            if !key.is_empty() {
                set_vars.push((key.to_string(), value.to_string()));
            }
        } else {
            cmd_start = Some(i);
            break;
        }
        i += 1;
    }

    if let Some(start) = cmd_start {
        // env VAR=val command args... — run command with modified env
        let saved_env: Vec<(String, String)> = std::env::vars().collect();
        let saved_cwd = std::env::current_dir().ok();

        if ignore_env {
            for (k, _) in &saved_env {
                unsafe { std::env::remove_var(k) };
            }
        }
        for name in &unset_vars {
            unsafe { std::env::remove_var(name) };
        }
        for (k, v) in &set_vars {
            unsafe { std::env::set_var(k, v) };
        }

        // Build and execute the command
        let cmd_str = args[start..].join(" ");
        let result = match super::parser::parse(&cmd_str) {
            Ok(cl) => {
                let mut all_output = Vec::new();
                let mut exit_code = 0;
                for chain in &cl.chains {
                    let r = super::pipeline::execute_chain_sync(chain);
                    all_output.extend(r.output);
                    exit_code = r.exit_code;
                }
                BuiltinResult {
                    output: all_output,
                    exit_app: false,
                    change_dir: None,
                    exit_code,
                }
            }
            Err(e) => fail(format!("env: {e}")),
        };

        // Restore environment
        let current_env: std::collections::HashSet<String> =
            std::env::vars().map(|(k, _)| k).collect();
        let saved_keys: std::collections::HashSet<String> =
            saved_env.iter().map(|(k, _)| k.clone()).collect();
        for key in &current_env {
            if !saved_keys.contains(key) {
                unsafe { std::env::remove_var(key) };
            }
        }
        for (key, value) in &saved_env {
            unsafe { std::env::set_var(key, value) };
        }
        if let Some(cwd) = saved_cwd {
            let _ = std::env::set_current_dir(cwd);
        }

        result
    } else {
        // No command — just print environment
        let mut vars: Vec<(String, String)> = if ignore_env {
            Vec::new()
        } else {
            std::env::vars().collect()
        };
        // Apply unsets
        vars.retain(|(k, _)| !unset_vars.contains(k));
        // Apply sets
        for (k, v) in &set_vars {
            if let Some(entry) = vars.iter_mut().find(|(ek, _)| ek == k) {
                entry.1 = v.clone();
            } else {
                vars.push((k.clone(), v.clone()));
            }
        }
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        let lines: Vec<String> = vars.iter().map(|(k, v)| format!("{k}={v}")).collect();
        ok(lines)
    }
}

// ── alias / unalias ─────────────────────────────────────────────────────────

fn execute_alias(args: &[String]) -> BuiltinResult {
    use crate::config::Config;

    if args.is_empty() {
        // List all aliases
        let config = Config::get();
        let mut entries: Vec<(&String, &String)> = config.alias.entries.iter().collect();
        entries.sort_by_key(|(k, _)| (*k).clone());
        let lines: Vec<String> = entries
            .iter()
            .map(|(name, value)| format!("alias {name}='{value}'"))
            .collect();
        return ok(lines);
    }

    let mut output = Vec::new();
    for arg in args {
        if let Some((name, value)) = arg.split_once('=') {
            // Define alias
            let name = name.to_string();
            let value = value.to_string();
            match Config::write_with(|c| {
                c.alias.entries.insert(name.clone(), value);
            }) {
                Ok(()) => {}
                Err(e) => output.push(format!("alias: {e}")),
            }
        } else {
            // Show specific alias
            let config = Config::get();
            match config.alias.entries.get(arg) {
                Some(value) => output.push(format!("alias {arg}='{value}'")),
                None => {
                    output.push(format!("alias: {arg}: not found"));
                    return BuiltinResult {
                        output,
                        exit_app: false,
                        change_dir: None,
                        exit_code: 1,
                    };
                }
            }
        }
    }
    ok(output)
}

fn execute_unalias(args: &[String]) -> BuiltinResult {
    use crate::config::Config;

    if args.is_empty() {
        return fail("unalias: usage: unalias [-a] name [name ...]".to_string());
    }

    if args.iter().any(|a| a == "-a") {
        // Remove all aliases
        match Config::write_with(|c| {
            c.alias.entries.clear();
        }) {
            Ok(()) => return ok(Vec::new()),
            Err(e) => return fail(format!("unalias: {e}")),
        }
    }

    let mut exit_code = 0;
    let mut output = Vec::new();
    for arg in args {
        match crate::config::Config::write_with(|c| c.alias.entries.remove(arg).is_some()) {
            Ok(true) => {}
            Ok(false) => {
                output.push(format!("unalias: {arg}: not found"));
                exit_code = 1;
            }
            Err(e) => {
                output.push(format!("unalias: {e}"));
                exit_code = 1;
            }
        }
    }
    BuiltinResult {
        output,
        exit_app: false,
        change_dir: None,
        exit_code,
    }
}

// ── type / which ────────────────────────────────────────────────────────────

fn execute_type(args: &[String]) -> BuiltinResult {
    let mut output = Vec::new();
    let mut exit_code = 0;
    let show_all = args.iter().any(|a| a == "-a" || a == "--all");
    let type_only = args.iter().any(|a| a == "-t" || a == "--type");

    for arg in args {
        if arg.starts_with('-') {
            continue;
        }
        let mut found = false;

        // Check builtin
        if lookup(arg).is_some() {
            if type_only {
                output.push("builtin".to_string());
            } else {
                output.push(format!("{arg} is a shell builtin"));
            }
            found = true;
            if !show_all {
                continue;
            }
        }

        // Check alias
        {
            let config = crate::config::Config::get();
            if let Some(value) = config.alias.entries.get(arg) {
                if type_only {
                    output.push("alias".to_string());
                } else {
                    output.push(format!("{arg} is aliased to '{value}'"));
                }
                found = true;
                if !show_all {
                    continue;
                }
            }
        }

        // Check script
        if let Some(entry) = super::script_registry::find_script(arg) {
            if type_only {
                output.push("script".to_string());
            } else {
                output.push(format!("{arg} is a mush script ({})", entry.entry_point.display()));
            }
            found = true;
            if !show_all {
                continue;
            }
        }

        // Check external
        if let Some(path) = super::path_resolver::find_in_path(arg) {
            if type_only {
                output.push("file".to_string());
            } else {
                output.push(format!("{arg} is {}", path.display()));
            }
            found = true;
        }

        if !found {
            if !type_only {
                output.push(format!("{arg}: not found"));
            }
            exit_code = 1;
        }
    }

    BuiltinResult {
        output,
        exit_app: false,
        change_dir: None,
        exit_code,
    }
}

// ── history ─────────────────────────────────────────────────────────────────

fn execute_history(args: &[String]) -> BuiltinResult {
    let db = crate::db::HistoryDb::global();

    if args.is_empty() {
        match db.list(50) {
            Ok(entries) => {
                let lines: Vec<String> = entries
                    .iter()
                    .enumerate()
                    .map(|(i, r)| format!("{:5}  {}", i + 1, r.command))
                    .collect();
                ok(lines)
            }
            Err(e) => fail(format!("history: {e}")),
        }
    } else if args[0] == "-c" || args[0] == "--clear" {
        match db.clear() {
            Ok(()) => ok(vec!["History cleared.".to_string()]),
            Err(e) => fail(format!("history: {e}")),
        }
    } else if let Ok(n) = args[0].parse::<usize>() {
        match db.list(n) {
            Ok(entries) => {
                let lines: Vec<String> = entries
                    .iter()
                    .enumerate()
                    .map(|(i, r)| format!("{:5}  {}", i + 1, r.command))
                    .collect();
                ok(lines)
            }
            Err(e) => fail(format!("history: {e}")),
        }
    } else {
        fail("history: usage: history [-c|--clear] [N]".to_string())
    }
}

// ── source ──────────────────────────────────────────────────────────────────

fn execute_source(args: &[String]) -> BuiltinResult {
    if args.is_empty() {
        return fail("source: usage: source FILE [ARGUMENTS]...".to_string());
    }

    let file_path = Path::new(&args[0]);
    let content = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => return fail(format!("source: {}: {e}", args[0])),
    };

    let mut all_output = Vec::new();
    let mut exit_code = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        match super::parser::parse(trimmed) {
            Ok(cl) => {
                for chain in &cl.chains {
                    let result = super::pipeline::execute_chain_sync(chain);
                    all_output.extend(result.output);
                    exit_code = result.exit_code;
                }
            }
            Err(e) => {
                all_output.push(format!("source: parse error: {e}"));
                exit_code = 1;
            }
        }
    }

    BuiltinResult {
        output: all_output,
        exit_app: false,
        change_dir: None,
        exit_code,
    }
}

// ── read ────────────────────────────────────────────────────────────────────

fn execute_read(args: &[String]) -> BuiltinResult {
    // In a TUI shell, read works with piped input only.
    // Interactive prompting is not supported yet.
    let mut var_names = Vec::new();
    let mut prompt = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--prompt" => {
                i += 1;
                if i < args.len() {
                    prompt = Some(args[i].clone());
                }
            }
            "-r" | "--raw" | "-s" | "--silent" => {
                // Accepted but no special handling in piped mode
            }
            arg if !arg.starts_with('-') => {
                var_names.push(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    if var_names.is_empty() {
        var_names.push("REPLY".to_string());
    }

    // Show prompt in output if specified
    let mut output = Vec::new();
    if let Some(p) = prompt {
        output.push(p);
    }

    // Since we don't have stdin_data passed to builtins yet,
    // read from environment-available sources.
    // For piped input, the pipeline would need to pass stdin_data.
    // For now, return exit code 1 (no input available).
    BuiltinResult {
        output,
        exit_app: false,
        change_dir: None,
        exit_code: 1,
    }
}

// ── test / [ ────────────────────────────────────────────────────────────────

fn execute_test(args: &[String]) -> BuiltinResult {
    let mut test_args = args.to_vec();

    // If invoked as [, strip trailing ]
    if !test_args.is_empty() && test_args.last().map(|s| s.as_str()) == Some("]") {
        test_args.pop();
    }

    if test_args.is_empty() {
        // Empty expression is false
        return BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: None,
            exit_code: 1,
        };
    }

    match eval_test_expr(&test_args) {
        Ok(true) => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: None,
            exit_code: 0,
        },
        Ok(false) => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: None,
            exit_code: 1,
        },
        Err(msg) => BuiltinResult {
            output: vec![format!("test: {msg}")],
            exit_app: false,
            change_dir: None,
            exit_code: 2,
        },
    }
}

fn eval_test_expr(args: &[String]) -> Result<bool, String> {
    if args.is_empty() {
        return Ok(false);
    }

    // Handle negation: ! expr
    if args[0] == "!" {
        return eval_test_expr(&args[1..]).map(|r| !r);
    }

    // Single arg: true if non-empty string
    if args.len() == 1 {
        return Ok(!args[0].is_empty());
    }

    // Two args: unary operator
    if args.len() == 2 {
        return eval_unary(&args[0], &args[1]);
    }

    // Three args: binary operator or compound with -a/-o
    if args.len() == 3 {
        return eval_binary(&args[0], &args[1], &args[2]);
    }

    // More args: look for -a (AND) and -o (OR) at the top level
    // -o has lower precedence than -a
    for i in (0..args.len()).rev() {
        if args[i] == "-o" {
            let left = eval_test_expr(&args[..i])?;
            let right = eval_test_expr(&args[i + 1..])?;
            return Ok(left || right);
        }
    }
    for i in (0..args.len()).rev() {
        if args[i] == "-a" {
            let left = eval_test_expr(&args[..i])?;
            let right = eval_test_expr(&args[i + 1..])?;
            return Ok(left && right);
        }
    }

    Err("too many arguments".to_string())
}

fn eval_unary(op: &str, operand: &str) -> Result<bool, String> {
    let path = Path::new(operand);
    match op {
        "-e" => Ok(path.exists()),
        "-f" => Ok(path.is_file()),
        "-d" => Ok(path.is_dir()),
        "-L" | "-h" => Ok(path.symlink_metadata().is_ok_and(|m| m.is_symlink())),
        "-r" => Ok(path.exists() && is_readable(path)),
        "-w" => Ok(path.exists() && is_writable(path)),
        "-x" => Ok(path.exists() && is_executable(path)),
        "-s" => Ok(path.metadata().is_ok_and(|m| m.len() > 0)),
        "-z" => Ok(operand.is_empty()),
        "-n" => Ok(!operand.is_empty()),
        _ => Err(format!("unknown unary operator: {op}")),
    }
}

fn eval_binary(left: &str, op: &str, right: &str) -> Result<bool, String> {
    match op {
        // String comparison
        "=" | "==" => Ok(left == right),
        "!=" => Ok(left != right),
        // Numeric comparison
        "-eq" => num_cmp(left, right, |a, b| a == b),
        "-ne" => num_cmp(left, right, |a, b| a != b),
        "-lt" => num_cmp(left, right, |a, b| a < b),
        "-le" => num_cmp(left, right, |a, b| a <= b),
        "-gt" => num_cmp(left, right, |a, b| a > b),
        "-ge" => num_cmp(left, right, |a, b| a >= b),
        _ => Err(format!("unknown binary operator: {op}")),
    }
}

fn num_cmp(left: &str, right: &str, f: fn(i64, i64) -> bool) -> Result<bool, String> {
    let a: i64 = left.parse().map_err(|_| format!("integer expression expected: {left}"))?;
    let b: i64 = right.parse().map_err(|_| format!("integer expression expected: {right}"))?;
    Ok(f(a, b))
}

fn is_readable(_path: &Path) -> bool {
    // On all platforms, if we can open for reading, it's readable
    std::fs::File::open(_path).is_ok()
}

fn is_writable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(meta) = path.metadata() {
            let mode = meta.mode();
            // Check user write bit
            mode & 0o200 != 0
        } else {
            false
        }
    }
    #[cfg(not(unix))]
    {
        path.metadata().is_ok_and(|m| !m.permissions().readonly())
    }
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(meta) = path.metadata() {
            meta.mode() & 0o111 != 0
        } else {
            false
        }
    }
    #[cfg(not(unix))]
    {
        // On Windows, check if extension is in PATHEXT
        if let Some(ext) = path.extension() {
            let ext = format!(".{}", ext.to_string_lossy().to_uppercase());
            let pathext = std::env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
            pathext.split(';').any(|pe| pe.to_uppercase() == ext)
        } else {
            false
        }
    }
}

// ── printenv ────────────────────────────────────────────────────────────────

fn execute_printenv(args: &[String]) -> BuiltinResult {
    let names: Vec<&str> = args.iter().map(|s| s.as_str()).filter(|s| !s.starts_with('-')).collect();

    if names.is_empty() {
        // Print all env vars
        let mut vars: Vec<(String, String)> = std::env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        let lines: Vec<String> = vars.iter().map(|(k, v)| format!("{k}={v}")).collect();
        return ok(lines);
    }

    let mut output = Vec::new();
    let mut exit_code = 0;
    for name in &names {
        match std::env::var(name) {
            Ok(value) => output.push(value),
            Err(_) => exit_code = 1,
        }
    }
    BuiltinResult { output, exit_app: false, change_dir: None, exit_code }
}

// ── pushd / popd ────────────────────────────────────────────────────────────

fn execute_pushd(args: &[String]) -> BuiltinResult {
    let current = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => return fail(format!("pushd: {e}")),
    };

    if args.is_empty() {
        // No args: swap top two directories
        let mut stack = DIR_STACK.lock().unwrap();
        if let Some(top) = stack.pop() {
            stack.push(current);
            match std::env::set_current_dir(&top) {
                Ok(()) => {
                    let new_cwd = std::env::current_dir().unwrap_or(top);
                    BuiltinResult {
                        output: vec![new_cwd.to_string_lossy().to_string()],
                        exit_app: false,
                        change_dir: Some(new_cwd),
                        exit_code: 0,
                    }
                }
                Err(e) => fail(format!("pushd: {e}")),
            }
        } else {
            fail("pushd: no other directory".to_string())
        }
    } else {
        let target = PathBuf::from(&args[0]);
        DIR_STACK.lock().unwrap().push(current);
        match std::env::set_current_dir(&target) {
            Ok(()) => {
                let new_cwd = std::env::current_dir().unwrap_or(target);
                BuiltinResult {
                    output: vec![new_cwd.to_string_lossy().to_string()],
                    exit_app: false,
                    change_dir: Some(new_cwd),
                    exit_code: 0,
                }
            }
            Err(e) => {
                // Undo the push since cd failed
                DIR_STACK.lock().unwrap().pop();
                fail(format!("pushd: {}: {e}", args[0]))
            }
        }
    }
}

fn execute_popd(_args: &[String]) -> BuiltinResult {
    let mut stack = DIR_STACK.lock().unwrap();
    match stack.pop() {
        Some(dir) => {
            drop(stack); // release lock before set_current_dir
            match std::env::set_current_dir(&dir) {
                Ok(()) => {
                    let new_cwd = std::env::current_dir().unwrap_or(dir);
                    BuiltinResult {
                        output: vec![new_cwd.to_string_lossy().to_string()],
                        exit_app: false,
                        change_dir: Some(new_cwd),
                        exit_code: 0,
                    }
                }
                Err(e) => fail(format!("popd: {e}")),
            }
        }
        None => fail("popd: directory stack empty".to_string()),
    }
}

// ── set ─────────────────────────────────────────────────────────────────────

const KNOWN_OPTIONS: &[(&str, &str)] = &[
    ("errexit", "e"),
    ("nounset", "u"),
    ("xtrace", "x"),
    ("pipefail", ""),
];

fn get_shell_options() -> HashSet<String> {
    let guard = SHELL_OPTIONS.lock().unwrap();
    guard.clone().unwrap_or_default()
}

fn set_shell_option(name: &str, enabled: bool) {
    let mut guard = SHELL_OPTIONS.lock().unwrap();
    let opts = guard.get_or_insert_with(HashSet::new);
    if enabled {
        opts.insert(name.to_string());
    } else {
        opts.remove(name);
    }
}

/// Check if a shell option is enabled (for use by other modules).
#[allow(dead_code)]
pub fn has_option(name: &str) -> bool {
    let guard = SHELL_OPTIONS.lock().unwrap();
    guard.as_ref().is_some_and(|opts| opts.contains(name))
}

fn execute_set(args: &[String]) -> BuiltinResult {
    if args.is_empty() {
        // No args: print all env vars sorted
        let mut vars: Vec<(String, String)> = std::env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        let lines: Vec<String> = vars.iter().map(|(k, v)| format!("{k}={v}")).collect();
        return ok(lines);
    }

    let mut i = 0;
    let mut output = Vec::new();
    while i < args.len() {
        let arg = &args[i];
        if arg == "-o" {
            i += 1;
            if i < args.len() {
                set_shell_option(&args[i], true);
            } else {
                // -o with no arg: list all options
                let opts = get_shell_options();
                for (name, _) in KNOWN_OPTIONS {
                    let state = if opts.contains(*name) { "on" } else { "off" };
                    output.push(format!("{name:15} {state}"));
                }
            }
        } else if arg == "+o" {
            i += 1;
            if i < args.len() {
                set_shell_option(&args[i], false);
            }
        } else if let Some(flag) = arg.strip_prefix('-') {
            // -e, -u, -x etc.
            for ch in flag.chars() {
                match ch {
                    'e' => set_shell_option("errexit", true),
                    'u' => set_shell_option("nounset", true),
                    'x' => set_shell_option("xtrace", true),
                    _ => {
                        output.push(format!("set: unknown option: -{ch}"));
                        return BuiltinResult { output, exit_app: false, change_dir: None, exit_code: 1 };
                    }
                }
            }
        } else if let Some(flag) = arg.strip_prefix('+') {
            for ch in flag.chars() {
                match ch {
                    'e' => set_shell_option("errexit", false),
                    'u' => set_shell_option("nounset", false),
                    'x' => set_shell_option("xtrace", false),
                    _ => {
                        output.push(format!("set: unknown option: +{ch}"));
                        return BuiltinResult { output, exit_app: false, change_dir: None, exit_code: 1 };
                    }
                }
            }
        }
        i += 1;
    }

    ok(output)
}

// ── dirs ────────────────────────────────────────────────────────────────────

fn execute_dirs(args: &[String]) -> BuiltinResult {
    let clear = args.iter().any(|a| a == "-c" || a == "--clear");
    let verbose = args.iter().any(|a| a == "-v");
    let long = args.iter().any(|a| a == "-l" || a == "--long");
    let per_line = args.iter().any(|a| a == "-p") || verbose;

    if clear {
        DIR_STACK.lock().unwrap().clear();
        return ok(Vec::new());
    }

    let stack = DIR_STACK.lock().unwrap();
    let cwd = std::env::current_dir().unwrap_or_default();
    let home = home_dir();

    let abbreviate = |p: &Path| -> String {
        if long {
            return p.to_string_lossy().to_string();
        }
        if let Some(ref h) = home
            && let Ok(rest) = p.strip_prefix(h)
        {
            if rest.as_os_str().is_empty() {
                return "~".to_string();
            }
            return format!("~/{}", rest.to_string_lossy());
        }
        p.to_string_lossy().to_string()
    };

    let mut entries = vec![abbreviate(&cwd)];
    for dir in stack.iter().rev() {
        entries.push(abbreviate(dir));
    }

    if verbose {
        let lines: Vec<String> = entries
            .iter()
            .enumerate()
            .map(|(i, d)| format!(" {i}  {d}"))
            .collect();
        ok(lines)
    } else if per_line {
        ok(entries)
    } else {
        ok(vec![entries.join(" ")])
    }
}

// ── expr ────────────────────────────────────────────────────────────────────

fn execute_expr(args: &[String]) -> BuiltinResult {
    if args.is_empty() {
        return fail("expr: missing operand".to_string());
    }

    match eval_expr(args) {
        Ok(value) => {
            let exit_code = if value == "0" || value.is_empty() { 1 } else { 0 };
            BuiltinResult {
                output: vec![value],
                exit_app: false,
                change_dir: None,
                exit_code,
            }
        }
        Err(msg) => BuiltinResult {
            output: vec![format!("expr: {msg}")],
            exit_app: false,
            change_dir: None,
            exit_code: 2,
        },
    }
}

fn eval_expr(args: &[String]) -> Result<String, String> {
    let mut pos = 0;
    let result = parse_expr_or(args, &mut pos)?;
    if pos < args.len() {
        return Err(format!("syntax error near '{}'", args[pos]));
    }
    Ok(result)
}

// OR level: expr1 | expr2
fn parse_expr_or(args: &[String], pos: &mut usize) -> Result<String, String> {
    let mut left = parse_expr_and(args, pos)?;
    while *pos < args.len() && args[*pos] == "|" {
        *pos += 1;
        let right = parse_expr_and(args, pos)?;
        left = if !left.is_empty() && left != "0" {
            left
        } else {
            right
        };
    }
    Ok(left)
}

// AND level: expr1 & expr2
fn parse_expr_and(args: &[String], pos: &mut usize) -> Result<String, String> {
    let mut left = parse_expr_compare(args, pos)?;
    while *pos < args.len() && args[*pos] == "&" {
        *pos += 1;
        let right = parse_expr_compare(args, pos)?;
        left = if (!left.is_empty() && left != "0") && (!right.is_empty() && right != "0") {
            left
        } else {
            "0".to_string()
        };
    }
    Ok(left)
}

// Comparison level: =, !=, <, <=, >, >=
fn parse_expr_compare(args: &[String], pos: &mut usize) -> Result<String, String> {
    let left = parse_expr_add(args, pos)?;
    if *pos < args.len() {
        let op = &args[*pos];
        match op.as_str() {
            "=" | "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                let op = op.clone();
                *pos += 1;
                let right = parse_expr_add(args, pos)?;
                // Try numeric comparison first
                let result = if let (Ok(l), Ok(r)) = (left.parse::<i64>(), right.parse::<i64>()) {
                    match op.as_str() {
                        "=" | "==" => l == r,
                        "!=" => l != r,
                        "<" => l < r,
                        "<=" => l <= r,
                        ">" => l > r,
                        ">=" => l >= r,
                        _ => false,
                    }
                } else {
                    match op.as_str() {
                        "=" | "==" => left == right,
                        "!=" => left != right,
                        "<" => left < right,
                        "<=" => left <= right,
                        ">" => left > right,
                        ">=" => left >= right,
                        _ => false,
                    }
                };
                return Ok(if result { "1".to_string() } else { "0".to_string() });
            }
            _ => {}
        }
    }
    Ok(left)
}

// Addition level: +, -
fn parse_expr_add(args: &[String], pos: &mut usize) -> Result<String, String> {
    let mut left = parse_expr_mul(args, pos)?;
    while *pos < args.len() {
        match args[*pos].as_str() {
            "+" => {
                *pos += 1;
                let right = parse_expr_mul(args, pos)?;
                let l: i64 = left.parse().map_err(|_| format!("non-integer argument: {left}"))?;
                let r: i64 = right.parse().map_err(|_| format!("non-integer argument: {right}"))?;
                left = (l + r).to_string();
            }
            "-" => {
                *pos += 1;
                let right = parse_expr_mul(args, pos)?;
                let l: i64 = left.parse().map_err(|_| format!("non-integer argument: {left}"))?;
                let r: i64 = right.parse().map_err(|_| format!("non-integer argument: {right}"))?;
                left = (l - r).to_string();
            }
            _ => break,
        }
    }
    Ok(left)
}

// Multiplication level: *, /, %
fn parse_expr_mul(args: &[String], pos: &mut usize) -> Result<String, String> {
    let mut left = parse_expr_primary(args, pos)?;
    while *pos < args.len() {
        match args[*pos].as_str() {
            "*" => {
                *pos += 1;
                let right = parse_expr_primary(args, pos)?;
                let l: i64 = left.parse().map_err(|_| format!("non-integer argument: {left}"))?;
                let r: i64 = right.parse().map_err(|_| format!("non-integer argument: {right}"))?;
                left = (l * r).to_string();
            }
            "/" => {
                *pos += 1;
                let right = parse_expr_primary(args, pos)?;
                let l: i64 = left.parse().map_err(|_| format!("non-integer argument: {left}"))?;
                let r: i64 = right.parse().map_err(|_| format!("non-integer argument: {right}"))?;
                if r == 0 {
                    return Err("division by zero".to_string());
                }
                left = (l / r).to_string();
            }
            "%" => {
                *pos += 1;
                let right = parse_expr_primary(args, pos)?;
                let l: i64 = left.parse().map_err(|_| format!("non-integer argument: {left}"))?;
                let r: i64 = right.parse().map_err(|_| format!("non-integer argument: {right}"))?;
                if r == 0 {
                    return Err("division by zero".to_string());
                }
                left = (l % r).to_string();
            }
            _ => break,
        }
    }
    Ok(left)
}

// Primary: literal, string functions, parenthesized expr
fn parse_expr_primary(args: &[String], pos: &mut usize) -> Result<String, String> {
    if *pos >= args.len() {
        return Err("missing operand".to_string());
    }

    let token = &args[*pos];

    // String functions
    match token.as_str() {
        "length" => {
            *pos += 1;
            if *pos >= args.len() {
                return Err("missing operand for length".to_string());
            }
            let s = &args[*pos];
            *pos += 1;
            return Ok(s.len().to_string());
        }
        "substr" => {
            *pos += 1;
            if *pos + 2 >= args.len() {
                return Err("missing operands for substr".to_string());
            }
            let s = &args[*pos];
            *pos += 1;
            let start: usize = args[*pos]
                .parse()
                .map_err(|_| "non-integer argument".to_string())?;
            *pos += 1;
            let len: usize = args[*pos]
                .parse()
                .map_err(|_| "non-integer argument".to_string())?;
            *pos += 1;
            if start == 0 || start > s.len() {
                return Ok(String::new());
            }
            let result: String = s.chars().skip(start - 1).take(len).collect();
            return Ok(result);
        }
        "index" => {
            *pos += 1;
            if *pos + 1 >= args.len() {
                return Err("missing operands for index".to_string());
            }
            let s = &args[*pos];
            *pos += 1;
            let chars = &args[*pos];
            *pos += 1;
            for (i, ch) in s.chars().enumerate() {
                if chars.contains(ch) {
                    return Ok((i + 1).to_string());
                }
            }
            return Ok("0".to_string());
        }
        "(" => {
            *pos += 1;
            let result = parse_expr_or(args, pos)?;
            if *pos < args.len() && args[*pos] == ")" {
                *pos += 1;
            } else {
                return Err("missing ')'".to_string());
            }
            return Ok(result);
        }
        _ => {}
    }

    // Match operator: STRING : REGEX
    *pos += 1;
    if *pos < args.len() && args[*pos] == ":" {
        *pos += 1;
        if *pos >= args.len() {
            return Err("missing operand for match".to_string());
        }
        let pattern = &args[*pos];
        *pos += 1;
        // Simple prefix match (not full regex)
        if token.starts_with(pattern.as_str()) {
            return Ok(pattern.len().to_string());
        }
        return Ok("0".to_string());
    }

    // Plain value
    Ok(token.clone())
}

// ── umask ───────────────────────────────────────────────────────────────────

fn execute_umask(args: &[String]) -> BuiltinResult {
    #[cfg(unix)]
    {
        execute_umask_unix(args)
    }
    #[cfg(not(unix))]
    {
        let _ = args;
        fail("umask: not supported on this platform".to_string())
    }
}

#[cfg(unix)]
fn execute_umask_unix(args: &[String]) -> BuiltinResult {
    let symbolic = args.iter().any(|a| a == "-S");
    let reusable = args.iter().any(|a| a == "-p");

    // Filter out flags to find mode arg
    let mode_arg: Option<&str> = args.iter()
        .find(|a| !a.starts_with('-'))
        .map(|s| s.as_str());

    if let Some(mode_str) = mode_arg {
        // Set umask
        match u32::from_str_radix(mode_str, 8) {
            Ok(mode) => {
                unsafe { libc::umask(mode as libc::mode_t) };
                ok(Vec::new())
            }
            Err(_) => fail(format!("umask: '{}': invalid octal number", mode_str)),
        }
    } else {
        // Display current umask
        let current = unsafe {
            let old = libc::umask(0o022);
            libc::umask(old);
            old
        };

        if symbolic {
            let u = 7 - ((current >> 6) & 7);
            let g = 7 - ((current >> 3) & 7);
            let o = 7 - (current & 7);
            let perm = |bits: u32| -> String {
                let mut s = String::new();
                if bits & 4 != 0 { s.push('r'); }
                if bits & 2 != 0 { s.push('w'); }
                if bits & 1 != 0 { s.push('x'); }
                if s.is_empty() { s.push('-'); }
                s
            };
            ok(vec![format!("u={},g={},o={}", perm(u), perm(g), perm(o))])
        } else if reusable {
            ok(vec![format!("umask {:04o}", current)])
        } else {
            ok(vec![format!("{:04o}", current)])
        }
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
