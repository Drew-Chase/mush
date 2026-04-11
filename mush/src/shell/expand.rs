use super::ast::*;
use std::fmt;
use std::io::Write as IoWrite;
use std::path::PathBuf;

/// Shell state needed for expansion.
pub struct ShellEnv {
    pub last_exit_code: i32,
    /// Temp files created by process substitution, to be cleaned up after execution.
    pub temp_files: Vec<PathBuf>,
}

const MAX_SUBSTITUTION_DEPTH: u32 = 64;

#[derive(Debug)]
pub enum ExpansionError {
    /// `${VAR:?msg}` when VAR is unset or empty.
    UnsetVariable { var: String, message: String },
    /// Command substitution failed.
    CommandSubstitutionFailed(String),
    /// Too many nested substitutions.
    MaxDepthExceeded,
}

impl fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpansionError::UnsetVariable { var, message } => {
                write!(f, "{var}: {message}")
            }
            ExpansionError::CommandSubstitutionFailed(msg) => {
                write!(f, "command substitution failed: {msg}")
            }
            ExpansionError::MaxDepthExceeded => {
                write!(f, "maximum command substitution nesting depth exceeded")
            }
        }
    }
}

/// Expand all variable references, `$?`, `$(cmd)`, globs, and process substitutions.
/// Returns a new CommandLine with all expansions resolved to literals.
/// Temp files created by process substitution are tracked in `env.temp_files`.
pub fn expand(cmd_line: &CommandLine, env: &mut ShellEnv) -> Result<CommandLine, ExpansionError> {
    expand_with_depth(cmd_line, env, 0)
}

fn expand_with_depth(
    cmd_line: &CommandLine,
    env: &mut ShellEnv,
    depth: u32,
) -> Result<CommandLine, ExpansionError> {
    if depth > MAX_SUBSTITUTION_DEPTH {
        return Err(ExpansionError::MaxDepthExceeded);
    }
    let mut chains = Vec::with_capacity(cmd_line.chains.len());
    for chain in &cmd_line.chains {
        chains.push(expand_chain(chain, env, depth)?);
    }
    Ok(CommandLine { chains })
}

fn expand_chain(chain: &Chain, env: &mut ShellEnv, depth: u32) -> Result<Chain, ExpansionError> {
    let first = expand_pipeline(&chain.first, env, depth)?;
    let mut rest = Vec::with_capacity(chain.rest.len());
    for (op, pipeline) in &chain.rest {
        rest.push((*op, expand_pipeline(pipeline, env, depth)?));
    }
    Ok(Chain {
        first,
        rest,
        background: chain.background,
    })
}

fn expand_pipeline(
    pipeline: &Pipeline,
    env: &mut ShellEnv,
    depth: u32,
) -> Result<Pipeline, ExpansionError> {
    if let Some(ref inner) = pipeline.subshell {
        let expanded_inner = expand_with_depth(inner, env, depth)?;
        return Ok(Pipeline {
            commands: Vec::new(),
            subshell: Some(Box::new(expanded_inner)),
        });
    }

    let mut commands = Vec::with_capacity(pipeline.commands.len());
    for cmd in &pipeline.commands {
        commands.push(expand_simple_command(cmd, env, depth)?);
    }
    Ok(Pipeline {
        commands,
        subshell: None,
    })
}

fn expand_simple_command(
    cmd: &SimpleCommand,
    env: &mut ShellEnv,
    depth: u32,
) -> Result<SimpleCommand, ExpansionError> {
    let mut words = Vec::new();
    for word in &cmd.words {
        let expanded = expand_word(word, env, depth)?;
        // Glob expansion: if the word has unquoted glob patterns, expand them
        let glob_expanded = expand_globs(&expanded);
        words.extend(glob_expanded);
    }

    let mut redirects = Vec::with_capacity(cmd.redirects.len());
    for redir in &cmd.redirects {
        redirects.push(Redirect {
            kind: redir.kind,
            target: expand_word(&redir.target, env, depth)?,
        });
    }

    Ok(SimpleCommand { words, redirects })
}

/// Check if a word contains unquoted glob metacharacters.
fn has_unquoted_globs(word: &Word) -> bool {
    word.parts.iter().any(|p| matches!(p, WordPart::GlobPattern(_)))
}

/// Expand glob patterns in a word. If the word has unquoted glob patterns,
/// try to match them against the filesystem. If matches are found, return
/// one Word per match. If no matches (or no glob patterns), return the
/// original word as-is.
fn expand_globs(word: &Word) -> Vec<Word> {
    if !has_unquoted_globs(word) {
        return vec![word.clone()];
    }

    // Build the pattern string by concatenating all parts
    let pattern = word.to_plain_string();

    if pattern.is_empty() {
        return vec![word.clone()];
    }

    match glob::glob(&pattern) {
        Ok(paths) => {
            let mut matches: Vec<Word> = paths
                .filter_map(|p| p.ok())
                .map(|p| Word::literal(&p.to_string_lossy()))
                .collect();
            matches.sort_by_key(|w| w.to_plain_string());
            if matches.is_empty() {
                // No matches — return pattern literally (bash default behavior)
                vec![word.clone()]
            } else {
                matches
            }
        }
        Err(_) => vec![word.clone()],
    }
}

fn expand_word(word: &Word, env: &mut ShellEnv, depth: u32) -> Result<Word, ExpansionError> {
    let mut new_parts = Vec::new();
    for part in &word.parts {
        expand_word_part(part, env, depth, &mut new_parts)?;
    }
    Ok(Word { parts: new_parts })
}

fn expand_word_part(
    part: &WordPart,
    env: &mut ShellEnv,
    depth: u32,
    out: &mut Vec<WordPart>,
) -> Result<(), ExpansionError> {
    match part {
        WordPart::Literal(s) => {
            out.push(WordPart::Literal(s.clone()));
        }
        WordPart::SingleQuoted(s) => {
            // No expansion inside single quotes
            out.push(WordPart::SingleQuoted(s.clone()));
        }
        WordPart::DoubleQuoted(inner) => {
            // Expand variables inside double quotes, but keep result as a single word
            let mut expanded_inner = Vec::new();
            for p in inner {
                expand_word_part(p, env, depth, &mut expanded_inner)?;
            }
            out.push(WordPart::DoubleQuoted(expanded_inner));
        }
        WordPart::Variable(var) => {
            let value = resolve_variable(&var.name, env);
            out.push(WordPart::Literal(value));
        }
        WordPart::BracedVariable(var) => {
            let value = resolve_braced_variable(var, env)?;
            out.push(WordPart::Literal(value));
        }
        WordPart::ExitCode => {
            out.push(WordPart::Literal(env.last_exit_code.to_string()));
        }
        WordPart::CommandSubstitution(inner_cmd) => {
            let result = execute_command_substitution(inner_cmd, env, depth)?;
            out.push(WordPart::Literal(result));
        }
        WordPart::GlobPattern(s) => {
            out.push(WordPart::GlobPattern(s.clone()));
        }
        WordPart::ProcessSubstitution(inner_cmd) => {
            let path = execute_process_substitution(inner_cmd, env, depth)?;
            out.push(WordPart::Literal(path));
        }
    }
    Ok(())
}

/// Execute a process substitution: run command, write output to temp file, return path.
fn execute_process_substitution(
    inner_cmd: &str,
    env: &mut ShellEnv,
    depth: u32,
) -> Result<String, ExpansionError> {
    if depth + 1 > MAX_SUBSTITUTION_DEPTH {
        return Err(ExpansionError::MaxDepthExceeded);
    }

    // Parse the inner command
    let cmd_line = super::parser::parse(inner_cmd)
        .map_err(|e| ExpansionError::CommandSubstitutionFailed(format!("parse error: {e}")))?;

    // Recursively expand
    let expanded = expand_with_depth(&cmd_line, env, depth + 1)?;

    // Execute all chains synchronously and collect output
    let mut all_output = Vec::new();
    for chain in &expanded.chains {
        let result = super::pipeline::execute_chain_sync(chain);
        all_output.extend(result.output);
    }

    // Write output to a temp file
    let mut temp_file = tempfile::NamedTempFile::new()
        .map_err(|e| ExpansionError::CommandSubstitutionFailed(format!("temp file: {e}")))?;

    let content = all_output.join("\n");
    temp_file
        .write_all(content.as_bytes())
        .map_err(|e| ExpansionError::CommandSubstitutionFailed(format!("write: {e}")))?;

    // Persist the temp file so it outlives this function
    let (_, path) = temp_file
        .keep()
        .map_err(|e| ExpansionError::CommandSubstitutionFailed(format!("persist: {e}")))?;

    let path_str = path.to_string_lossy().to_string();
    env.temp_files.push(path);

    Ok(path_str)
}

/// Execute a command substitution: parse, expand, execute, capture stdout.
fn execute_command_substitution(
    inner_cmd: &str,
    env: &mut ShellEnv,
    depth: u32,
) -> Result<String, ExpansionError> {
    if depth + 1 > MAX_SUBSTITUTION_DEPTH {
        return Err(ExpansionError::MaxDepthExceeded);
    }

    // Parse the inner command
    let cmd_line = super::parser::parse(inner_cmd)
        .map_err(|e| ExpansionError::CommandSubstitutionFailed(format!("parse error: {e}")))?;

    // Recursively expand
    let expanded = expand_with_depth(&cmd_line, env, depth + 1)?;

    // Execute all chains synchronously and collect output
    let mut all_output = Vec::new();
    for chain in &expanded.chains {
        let result = super::pipeline::execute_chain_sync(chain);
        all_output.extend(result.output);
    }

    // Join output lines and strip trailing newlines (bash convention)
    let mut output = all_output.join("\n");
    while output.ends_with('\n') || output.ends_with('\r') {
        output.pop();
    }

    Ok(output)
}

fn resolve_variable(name: &str, _env: &mut ShellEnv) -> String {
    std::env::var(name).unwrap_or_default()
}

fn resolve_braced_variable(var: &VarRef, _env: &mut ShellEnv) -> Result<String, ExpansionError> {
    let value = std::env::var(&var.name).ok();
    let is_set_and_nonempty = value.as_ref().is_some_and(|v| !v.is_empty());

    match &var.modifier {
        None => Ok(value.unwrap_or_default()),
        Some(VarModifier::Default(default)) => {
            if is_set_and_nonempty {
                Ok(value.unwrap())
            } else {
                Ok(default.clone())
            }
        }
        Some(VarModifier::Assign(default)) => {
            if is_set_and_nonempty {
                Ok(value.unwrap())
            } else {
                // SAFETY: mush is single-threaded for command execution
                unsafe { std::env::set_var(&var.name, default) };
                Ok(default.clone())
            }
        }
        Some(VarModifier::Alternate(alt)) => {
            if is_set_and_nonempty {
                Ok(alt.clone())
            } else {
                Ok(String::new())
            }
        }
        Some(VarModifier::Error(msg)) => {
            if is_set_and_nonempty {
                Ok(value.unwrap())
            } else {
                Err(ExpansionError::UnsetVariable {
                    var: var.name.clone(),
                    message: msg.clone(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::parser;

    fn expand_input(input: &str, exit_code: i32) -> Result<CommandLine, ExpansionError> {
        let cl = parser::parse(input).unwrap();
        let mut env = ShellEnv {
            last_exit_code: exit_code,
            temp_files: Vec::new(),
        };
        expand(&cl, &mut env)
    }

    fn first_words(input: &str) -> Vec<String> {
        let cl = expand_input(input, 0).unwrap();
        cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect()
    }

    #[test]
    fn expand_simple_var() {
        unsafe { std::env::set_var("MUSH_TEST_VAR", "hello") };
        assert_eq!(first_words("echo $MUSH_TEST_VAR"), vec!["echo", "hello"]);
        unsafe { std::env::remove_var("MUSH_TEST_VAR") };
    }

    #[test]
    fn expand_braced_var() {
        unsafe { std::env::set_var("MUSH_TEST_B", "world") };
        assert_eq!(first_words("echo ${MUSH_TEST_B}"), vec!["echo", "world"]);
        unsafe { std::env::remove_var("MUSH_TEST_B") };
    }

    #[test]
    fn expand_exit_code() {
        let cl = expand_input("echo $?", 42).unwrap();
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(words, vec!["echo", "42"]);
    }

    #[test]
    fn expand_default_modifier() {
        unsafe { std::env::remove_var("MUSH_UNSET_VAR") };
        assert_eq!(
            first_words("echo ${MUSH_UNSET_VAR:-fallback}"),
            vec!["echo", "fallback"]
        );
    }

    #[test]
    fn expand_default_when_set() {
        unsafe { std::env::set_var("MUSH_SET_VAR", "existing") };
        assert_eq!(
            first_words("echo ${MUSH_SET_VAR:-fallback}"),
            vec!["echo", "existing"]
        );
        unsafe { std::env::remove_var("MUSH_SET_VAR") };
    }

    #[test]
    fn expand_assign_modifier() {
        unsafe { std::env::remove_var("MUSH_ASSIGN_VAR") };
        assert_eq!(
            first_words("echo ${MUSH_ASSIGN_VAR:=assigned}"),
            vec!["echo", "assigned"]
        );
        assert_eq!(std::env::var("MUSH_ASSIGN_VAR").unwrap(), "assigned");
        unsafe { std::env::remove_var("MUSH_ASSIGN_VAR") };
    }

    #[test]
    fn expand_alternate_modifier() {
        unsafe { std::env::set_var("MUSH_ALT_VAR", "exists") };
        assert_eq!(
            first_words("echo ${MUSH_ALT_VAR:+alt_value}"),
            vec!["echo", "alt_value"]
        );
        unsafe { std::env::remove_var("MUSH_ALT_VAR") };
    }

    #[test]
    fn expand_alternate_when_unset() {
        unsafe { std::env::remove_var("MUSH_ALT_UNSET") };
        assert_eq!(
            first_words("echo ${MUSH_ALT_UNSET:+alt_value}"),
            vec!["echo", ""]
        );
    }

    #[test]
    fn expand_error_modifier() {
        unsafe { std::env::remove_var("MUSH_ERR_VAR") };
        let result = expand_input("echo ${MUSH_ERR_VAR:?not set}", 0);
        assert!(result.is_err());
        match result.unwrap_err() {
            ExpansionError::UnsetVariable { var, message } => {
                assert_eq!(var, "MUSH_ERR_VAR");
                assert_eq!(message, "not set");
            }
            other => panic!("expected UnsetVariable, got {other:?}"),
        }
    }

    #[test]
    fn single_quotes_no_expansion() {
        unsafe { std::env::set_var("MUSH_SQ", "value") };
        let words = first_words("echo '$MUSH_SQ'");
        assert_eq!(words, vec!["echo", "$MUSH_SQ"]);
        unsafe { std::env::remove_var("MUSH_SQ") };
    }

    #[test]
    fn double_quotes_expand() {
        unsafe { std::env::set_var("MUSH_DQ", "val") };
        let words = first_words(r#"echo "hello $MUSH_DQ""#);
        assert_eq!(words, vec!["echo", "hello val"]);
        unsafe { std::env::remove_var("MUSH_DQ") };
    }

    #[test]
    fn unset_var_expands_empty() {
        unsafe { std::env::remove_var("MUSH_NOPE") };
        assert_eq!(first_words("echo $MUSH_NOPE"), vec!["echo", ""]);
    }

    #[test]
    fn glob_no_match_passes_literally() {
        // A pattern that won't match anything
        let words = first_words("echo zzz_no_match_*.xyz");
        assert_eq!(words, vec!["echo", "zzz_no_match_*.xyz"]);
    }

    #[test]
    fn glob_in_single_quotes_no_expansion() {
        let words = first_words("echo '*.txt'");
        assert_eq!(words, vec!["echo", "*.txt"]);
    }

    fn ensure_config() {
        use crate::config::Config;
        // Initialize config if not already done (idempotent via OnceLock)
        let _ = Config::load_or_default(
            std::env::temp_dir().join("mush_test_config.toml"),
        );
    }

    #[test]
    fn command_substitution_echo() {
        ensure_config();
        // $(echo hello) should capture "hello"
        let cl = expand_input("echo $(echo hello)", 0).unwrap();
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(words, vec!["echo", "hello"]);
    }

    #[test]
    fn command_substitution_in_double_quotes() {
        ensure_config();
        let cl = expand_input(r#"echo "$(echo world)""#, 0).unwrap();
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(words, vec!["echo", "world"]);
    }

    #[test]
    fn command_substitution_in_single_quotes_not_expanded() {
        let words = first_words("echo '$(echo nope)'");
        assert_eq!(words, vec!["echo", "$(echo nope)"]);
    }

    #[test]
    fn command_substitution_nested() {
        ensure_config();
        let cl = expand_input("echo $(echo $(echo deep))", 0).unwrap();
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(words, vec!["echo", "deep"]);
    }

    #[test]
    fn glob_matches_real_files() {
        // This test relies on Cargo.toml existing in the current working directory
        // which it does when tests are run from the workspace root
        let cl = expand_input("ls Cargo.*", 0).unwrap();
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        // Should at least have "ls" and "Cargo.toml"
        assert!(words.len() >= 2);
        assert_eq!(words[0], "ls");
        assert!(words.iter().any(|w| w.contains("Cargo")));
    }
}
