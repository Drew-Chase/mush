use super::ast::*;
use std::fmt;

/// Shell state needed for expansion.
pub struct ShellEnv {
    pub last_exit_code: i32,
}

#[derive(Debug)]
pub enum ExpansionError {
    /// `${VAR:?msg}` when VAR is unset or empty.
    UnsetVariable { var: String, message: String },
}

impl fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpansionError::UnsetVariable { var, message } => {
                write!(f, "{var}: {message}")
            }
        }
    }
}

/// Expand all variable references and `$?` in a CommandLine AST.
/// Returns a new CommandLine with all expansions resolved to literals.
pub fn expand(cmd_line: &CommandLine, env: &ShellEnv) -> Result<CommandLine, ExpansionError> {
    let mut chains = Vec::with_capacity(cmd_line.chains.len());
    for chain in &cmd_line.chains {
        chains.push(expand_chain(chain, env)?);
    }
    Ok(CommandLine { chains })
}

fn expand_chain(chain: &Chain, env: &ShellEnv) -> Result<Chain, ExpansionError> {
    let first = expand_pipeline(&chain.first, env)?;
    let mut rest = Vec::with_capacity(chain.rest.len());
    for (op, pipeline) in &chain.rest {
        rest.push((*op, expand_pipeline(pipeline, env)?));
    }
    Ok(Chain { first, rest })
}

fn expand_pipeline(pipeline: &Pipeline, env: &ShellEnv) -> Result<Pipeline, ExpansionError> {
    let mut commands = Vec::with_capacity(pipeline.commands.len());
    for cmd in &pipeline.commands {
        commands.push(expand_simple_command(cmd, env)?);
    }
    Ok(Pipeline { commands })
}

fn expand_simple_command(
    cmd: &SimpleCommand,
    env: &ShellEnv,
) -> Result<SimpleCommand, ExpansionError> {
    let mut words = Vec::new();
    for word in &cmd.words {
        let expanded = expand_word(word, env)?;
        // Glob expansion: if the word has unquoted glob patterns, expand them
        let glob_expanded = expand_globs(&expanded);
        words.extend(glob_expanded);
    }

    let mut redirects = Vec::with_capacity(cmd.redirects.len());
    for redir in &cmd.redirects {
        redirects.push(Redirect {
            kind: redir.kind,
            target: expand_word(&redir.target, env)?,
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

fn expand_word(word: &Word, env: &ShellEnv) -> Result<Word, ExpansionError> {
    let mut new_parts = Vec::new();
    for part in &word.parts {
        expand_word_part(part, env, &mut new_parts)?;
    }
    Ok(Word { parts: new_parts })
}

fn expand_word_part(
    part: &WordPart,
    env: &ShellEnv,
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
                expand_word_part(p, env, &mut expanded_inner)?;
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
        WordPart::CommandSubstitution(s) => {
            // Command substitution expansion is handled separately (Step 6)
            // For now, pass through as literal
            out.push(WordPart::CommandSubstitution(s.clone()));
        }
        WordPart::GlobPattern(s) => {
            // Glob expansion is handled separately (Step 5)
            out.push(WordPart::GlobPattern(s.clone()));
        }
    }
    Ok(())
}

fn resolve_variable(name: &str, _env: &ShellEnv) -> String {
    std::env::var(name).unwrap_or_default()
}

fn resolve_braced_variable(var: &VarRef, _env: &ShellEnv) -> Result<String, ExpansionError> {
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
        let env = ShellEnv {
            last_exit_code: exit_code,
        };
        expand(&cl, &env)
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
