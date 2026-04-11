/// AST types for the mush shell parser.
///
/// The hierarchy is:
///   CommandLine -> Chain+ -> Pipeline+ -> SimpleCommand+
///
/// A `CommandLine` is the top-level unit representing one line of user input.
/// A complete parsed command line (one line of user input).
#[derive(Debug, Clone)]
pub struct CommandLine {
    pub chains: Vec<Chain>,
}

/// A sequence of pipelines connected by `&&` or `||`.
/// Example: `ls | grep foo && echo done || echo failed`
#[derive(Debug, Clone)]
pub struct Chain {
    pub first: Pipeline,
    pub rest: Vec<(ChainOp, Pipeline)>,
    /// If true, the chain runs in the background (`&` suffix).
    pub background: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainOp {
    And, // &&
    Or,  // ||
}

/// One or more simple commands connected by `|`.
/// If `subshell` is `Some`, the commands vec is empty and the inner
/// CommandLine is executed in an isolated environment instead.
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub commands: Vec<SimpleCommand>,
    pub subshell: Option<Box<CommandLine>>,
}

/// A single command with arguments and I/O redirections.
#[derive(Debug, Clone)]
pub struct SimpleCommand {
    pub words: Vec<Word>,
    pub redirects: Vec<Redirect>,
}

/// A word (command name or argument) that may contain multiple segments.
/// Segments track quoting context for correct expansion behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub parts: Vec<WordPart>,
}

impl Word {
    pub fn literal(s: &str) -> Self {
        Self {
            parts: vec![WordPart::Literal(s.to_string())],
        }
    }

    /// Assemble the word into a plain string (no expansion, just concatenate literals).
    pub fn to_plain_string(&self) -> String {
        let mut out = String::new();
        for part in &self.parts {
            match part {
                WordPart::Literal(s) | WordPart::SingleQuoted(s) => out.push_str(s),
                WordPart::DoubleQuoted(inner) => {
                    for p in inner {
                        if let WordPart::Literal(s) = p {
                            out.push_str(s);
                        }
                    }
                }
                WordPart::Variable(var) => {
                    out.push('$');
                    out.push_str(&var.name);
                }
                WordPart::BracedVariable(var) => {
                    out.push_str("${");
                    out.push_str(&var.name);
                    out.push('}');
                }
                WordPart::ExitCode => out.push_str("$?"),
                WordPart::CommandSubstitution(s) => {
                    out.push_str("$(");
                    out.push_str(s);
                    out.push(')');
                }
                WordPart::GlobPattern(s) => out.push_str(s),
                WordPart::ProcessSubstitution(s) => {
                    out.push_str("<(");
                    out.push_str(s);
                    out.push(')');
                }
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WordPart {
    /// Plain text.
    Literal(String),
    /// Contents of `'...'` — no expansion.
    SingleQuoted(String),
    /// Contents of `"..."` — may contain variable/substitution parts.
    DoubleQuoted(Vec<WordPart>),
    /// `$NAME` — simple variable reference.
    Variable(VarRef),
    /// `${NAME}` or `${NAME:-default}` etc.
    BracedVariable(VarRef),
    /// `$?` — last exit code.
    ExitCode,
    /// `$(cmd)` — the inner command string (to be parsed and executed).
    CommandSubstitution(String),
    /// Glob metacharacters: `*`, `?`, `[...]`.
    GlobPattern(String),
    /// `<(cmd)` — process substitution. The inner command string.
    ProcessSubstitution(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarRef {
    pub name: String,
    pub modifier: Option<VarModifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarModifier {
    /// `${VAR:-default}` — use default if unset or empty.
    Default(String),
    /// `${VAR:=default}` — assign default if unset or empty.
    Assign(String),
    /// `${VAR:+alternate}` — use alternate if set and non-empty.
    Alternate(String),
    /// `${VAR:?message}` — error if unset or empty.
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Redirect {
    pub kind: RedirectKind,
    pub target: Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectKind {
    /// `>`
    StdoutOverwrite,
    /// `>>`
    StdoutAppend,
    /// `<`
    StdinRead,
    /// `2>`
    StderrOverwrite,
    /// `2>>`
    StderrAppend,
    /// `2>&1`
    StderrToStdout,
    /// `<<<`
    HereString,
    /// `<< DELIMITER`
    HereDoc,
}
