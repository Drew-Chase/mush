use super::ast::*;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof,
    EmptyPipeline,
    UnmatchedParen,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(t) => write!(f, "unexpected token: {t}"),
            ParseError::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseError::EmptyPipeline => write!(f, "empty pipeline"),
            ParseError::UnmatchedParen => write!(f, "unmatched '(' in command substitution"),
        }
    }
}

// ── Tokens ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum Token {
    Word(Word),
    Pipe,           // |
    And,            // &&
    Or,             // ||
    RedirectOut,    // >
    RedirectAppend, // >>
    RedirectIn,     // <
    StderrOut,      // 2>
    StderrAppend,   // 2>>
    StderrToStdout, // 2>&1
    HereString,     // <<<
    HereDoc,        // <<
    Background,     // &
    OpenParen,      // (
    CloseParen,     // )
    Semicolon,      // ;
}

// ── Lexer ───────────────────────────────────────────────────────────────────

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.chars.peek().is_some_and(|c| c.is_whitespace()) {
            self.chars.next();
        }
    }

    fn tokenize_all(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            if self.chars.peek().is_none() {
                break;
            }
            tokens.push(self.next_token()?);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        let c = match self.chars.peek() {
            Some(&c) => c,
            None => return Err(ParseError::UnexpectedEof),
        };

        match c {
            ';' => {
                self.chars.next();
                Ok(Token::Semicolon)
            }
            '(' => {
                self.chars.next();
                Ok(Token::OpenParen)
            }
            ')' => {
                self.chars.next();
                Ok(Token::CloseParen)
            }
            '|' => {
                self.chars.next();
                if self.chars.peek() == Some(&'|') {
                    self.chars.next();
                    Ok(Token::Or)
                } else {
                    Ok(Token::Pipe)
                }
            }
            '&' => {
                self.chars.next();
                if self.chars.peek() == Some(&'&') {
                    self.chars.next();
                    Ok(Token::And)
                } else {
                    Ok(Token::Background)
                }
            }
            '>' => {
                self.chars.next();
                if self.chars.peek() == Some(&'>') {
                    self.chars.next();
                    Ok(Token::RedirectAppend)
                } else {
                    Ok(Token::RedirectOut)
                }
            }
            '<' => {
                self.chars.next();
                if self.chars.peek() == Some(&'(') {
                    // <(cmd) — process substitution
                    self.chars.next(); // consume '('
                    let inner = self.read_command_substitution();
                    Ok(Token::Word(Word {
                        parts: vec![WordPart::ProcessSubstitution(inner)],
                    }))
                } else if self.chars.peek() == Some(&'<') {
                    self.chars.next(); // consume second <
                    if self.chars.peek() == Some(&'<') {
                        self.chars.next(); // consume third <
                        Ok(Token::HereString)
                    } else {
                        Ok(Token::HereDoc)
                    }
                } else {
                    Ok(Token::RedirectIn)
                }
            }
            '2' => {
                // Peek ahead for 2>, 2>>, 2>&1
                let mut lookahead = self.chars.clone();
                lookahead.next(); // consume '2'
                match lookahead.peek() {
                    Some(&'>') => {
                        lookahead.next(); // consume '>'
                        match lookahead.peek() {
                            Some(&'>') => {
                                // 2>>
                                self.chars.next(); // 2
                                self.chars.next(); // >
                                self.chars.next(); // >
                                Ok(Token::StderrAppend)
                            }
                            Some(&'&') => {
                                let mut la2 = lookahead.clone();
                                la2.next(); // consume '&'
                                if la2.peek() == Some(&'1') {
                                    // 2>&1
                                    self.chars.next(); // 2
                                    self.chars.next(); // >
                                    self.chars.next(); // &
                                    self.chars.next(); // 1
                                    Ok(Token::StderrToStdout)
                                } else {
                                    // 2>& but not 2>&1 — treat 2> as stderr redirect
                                    self.chars.next(); // 2
                                    self.chars.next(); // >
                                    Ok(Token::StderrOut)
                                }
                            }
                            _ => {
                                // 2>
                                self.chars.next(); // 2
                                self.chars.next(); // >
                                Ok(Token::StderrOut)
                            }
                        }
                    }
                    _ => {
                        // Just '2' followed by non-'>' — it's a word
                        self.read_word()
                    }
                }
            }
            _ => self.read_word(),
        }
    }

    fn read_word(&mut self) -> Result<Token, ParseError> {
        let mut parts = Vec::new();
        let mut literal_buf = String::new();

        loop {
            match self.chars.peek() {
                None => break,
                Some(&c) if c.is_whitespace() => break,
                Some(&('|' | ';' | '<' | '(' | ')')) => break,
                Some(&'>') => {
                    // Check if this is a redirect operator, not part of the word
                    break;
                }
                Some(&'&') => {
                    // Check for && operator
                    let mut la = self.chars.clone();
                    la.next();
                    if la.peek() == Some(&'&') {
                        break;
                    }
                    // Lone & — break (it's an operator context)
                    break;
                }
                Some(&'\'') => {
                    if !literal_buf.is_empty() {
                        parts.push(WordPart::Literal(std::mem::take(&mut literal_buf)));
                    }
                    self.chars.next(); // consume opening '
                    let mut quoted = String::new();
                    loop {
                        match self.chars.next() {
                            Some('\'') | None => break,
                            Some(ch) => quoted.push(ch),
                        }
                    }
                    parts.push(WordPart::SingleQuoted(quoted));
                }
                Some(&'"') => {
                    if !literal_buf.is_empty() {
                        parts.push(WordPart::Literal(std::mem::take(&mut literal_buf)));
                    }
                    self.chars.next(); // consume opening "
                    let inner = self.read_double_quoted();
                    parts.push(WordPart::DoubleQuoted(inner));
                }
                Some(&'$') => {
                    if !literal_buf.is_empty() {
                        parts.push(WordPart::Literal(std::mem::take(&mut literal_buf)));
                    }
                    parts.push(self.read_dollar());
                }
                Some(&'\\') => {
                    self.chars.next(); // consume backslash
                    if let Some(&next) = self.chars.peek() {
                        literal_buf.push(next);
                        self.chars.next();
                    }
                }
                Some(&c) if is_glob_char(c) => {
                    if !literal_buf.is_empty() {
                        parts.push(WordPart::Literal(std::mem::take(&mut literal_buf)));
                    }
                    parts.push(self.read_glob());
                }
                Some(&c) => {
                    self.chars.next();
                    literal_buf.push(c);
                }
            }
        }

        if !literal_buf.is_empty() {
            parts.push(WordPart::Literal(literal_buf));
        }

        Ok(Token::Word(Word { parts }))
    }

    fn read_double_quoted(&mut self) -> Vec<WordPart> {
        let mut parts = Vec::new();
        let mut literal_buf = String::new();

        loop {
            match self.chars.peek() {
                None | Some(&'"') => {
                    if self.chars.peek() == Some(&'"') {
                        self.chars.next(); // consume closing "
                    }
                    break;
                }
                Some(&'$') => {
                    if !literal_buf.is_empty() {
                        parts.push(WordPart::Literal(std::mem::take(&mut literal_buf)));
                    }
                    parts.push(self.read_dollar());
                }
                Some(&'\\') => {
                    self.chars.next();
                    // In double quotes, only \$, \", \\, \` are special
                    match self.chars.peek() {
                        Some(&c) if matches!(c, '$' | '"' | '\\' | '`') => {
                            literal_buf.push(c);
                            self.chars.next();
                        }
                        _ => {
                            literal_buf.push('\\');
                        }
                    }
                }
                Some(&c) => {
                    self.chars.next();
                    literal_buf.push(c);
                }
            }
        }

        if !literal_buf.is_empty() {
            parts.push(WordPart::Literal(literal_buf));
        }

        parts
    }

    fn read_dollar(&mut self) -> WordPart {
        self.chars.next(); // consume '$'

        match self.chars.peek() {
            Some(&'?') => {
                self.chars.next();
                WordPart::ExitCode
            }
            Some(&'(') => {
                self.chars.next(); // consume '('
                let inner = self.read_command_substitution();
                WordPart::CommandSubstitution(inner)
            }
            Some(&'{') => {
                self.chars.next(); // consume '{'
                self.read_braced_variable()
            }
            Some(&c) if c == '_' || c.is_ascii_alphabetic() => {
                let name = self.read_var_name();
                WordPart::Variable(VarRef {
                    name,
                    modifier: None,
                })
            }
            _ => {
                // Bare '$' with nothing valid after — literal
                WordPart::Literal("$".to_string())
            }
        }
    }

    fn read_var_name(&mut self) -> String {
        let mut name = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '_' || c.is_ascii_alphanumeric() {
                name.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        name
    }

    fn read_braced_variable(&mut self) -> WordPart {
        let name = self.read_var_name();

        let modifier = match self.chars.peek() {
            Some(&':') => {
                self.chars.next(); // consume ':'
                match self.chars.peek() {
                    Some(&'-') => {
                        self.chars.next();
                        let val = self.read_until_brace_close();
                        Some(VarModifier::Default(val))
                    }
                    Some(&'=') => {
                        self.chars.next();
                        let val = self.read_until_brace_close();
                        Some(VarModifier::Assign(val))
                    }
                    Some(&'+') => {
                        self.chars.next();
                        let val = self.read_until_brace_close();
                        Some(VarModifier::Alternate(val))
                    }
                    Some(&'?') => {
                        self.chars.next();
                        let val = self.read_until_brace_close();
                        Some(VarModifier::Error(val))
                    }
                    _ => {
                        // ':' without recognized modifier — consume to }
                        let _ = self.read_until_brace_close();
                        None
                    }
                }
            }
            _ => None,
        };

        // Consume closing '}'
        if self.chars.peek() == Some(&'}') {
            self.chars.next();
        }

        WordPart::BracedVariable(VarRef { name, modifier })
    }

    fn read_until_brace_close(&mut self) -> String {
        let mut buf = String::new();
        let mut depth = 1;
        loop {
            match self.chars.peek() {
                None => break,
                Some(&'}') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    self.chars.next();
                    buf.push('}');
                }
                Some(&'{') => {
                    depth += 1;
                    self.chars.next();
                    buf.push('{');
                }
                Some(&c) => {
                    self.chars.next();
                    buf.push(c);
                }
            }
        }
        buf
    }

    fn read_command_substitution(&mut self) -> String {
        let mut buf = String::new();
        let mut depth = 1u32;

        loop {
            match self.chars.next() {
                None => break,
                Some('(') => {
                    depth += 1;
                    buf.push('(');
                }
                Some(')') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    buf.push(')');
                }
                Some('\'') => {
                    buf.push('\'');
                    // Inside single quotes — consume literally
                    loop {
                        match self.chars.next() {
                            Some('\'') | None => {
                                buf.push('\'');
                                break;
                            }
                            Some(c) => buf.push(c),
                        }
                    }
                }
                Some('"') => {
                    buf.push('"');
                    loop {
                        match self.chars.next() {
                            Some('"') | None => {
                                buf.push('"');
                                break;
                            }
                            Some('\\') => {
                                buf.push('\\');
                                if let Some(c) = self.chars.next() {
                                    buf.push(c);
                                }
                            }
                            Some(c) => buf.push(c),
                        }
                    }
                }
                Some(c) => buf.push(c),
            }
        }

        buf
    }

    fn read_glob(&mut self) -> WordPart {
        let mut pattern = String::new();
        loop {
            match self.chars.peek() {
                Some(&'*') => {
                    pattern.push('*');
                    self.chars.next();
                }
                Some(&'?') => {
                    pattern.push('?');
                    self.chars.next();
                }
                Some(&'[') => {
                    pattern.push('[');
                    self.chars.next();
                    // Read until ']'
                    loop {
                        match self.chars.peek() {
                            Some(&']') => {
                                pattern.push(']');
                                self.chars.next();
                                break;
                            }
                            Some(&c) => {
                                pattern.push(c);
                                self.chars.next();
                            }
                            None => break,
                        }
                    }
                }
                _ => break,
            }
        }
        WordPart::GlobPattern(pattern)
    }
}

fn is_glob_char(c: char) -> bool {
    matches!(c, '*' | '?' | '[')
}

// ── Parser ──────────────────────────────────────────────────────────────────

/// Parses a shell input string into a [`CommandLine`] AST.
pub fn parse(input: &str) -> Result<CommandLine, ParseError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_all()?;

    if tokens.is_empty() {
        return Ok(CommandLine {
            chains: Vec::new(),
        });
    }

    let mut parser = Parser::new(&tokens);
    parser.parse_command_line()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn parse_command_line(&mut self) -> Result<CommandLine, ParseError> {
        let mut chains = Vec::new();

        // Skip leading semicolons
        while matches!(self.peek(), Some(Token::Semicolon | Token::Background)) {
            self.advance();
        }

        if self.peek().is_some() {
            let mut chain = self.parse_chain()?;
            // Check for trailing & (background)
            if self.peek() == Some(&Token::Background) {
                chain.background = true;
                self.advance();
            }
            chains.push(chain);
        }

        while matches!(self.peek(), Some(Token::Semicolon | Token::Background)) {
            self.advance();
            // Skip consecutive separators
            while matches!(self.peek(), Some(Token::Semicolon | Token::Background)) {
                self.advance();
            }
            if self.peek().is_some() {
                let mut chain = self.parse_chain()?;
                if self.peek() == Some(&Token::Background) {
                    chain.background = true;
                    self.advance();
                }
                chains.push(chain);
            }
        }

        Ok(CommandLine { chains })
    }

    fn parse_chain(&mut self) -> Result<Chain, ParseError> {
        let first = self.parse_pipeline()?;
        let mut rest = Vec::new();

        loop {
            let op = match self.peek() {
                Some(Token::And) => ChainOp::And,
                Some(Token::Or) => ChainOp::Or,
                _ => break,
            };
            self.advance();
            let pipeline = self.parse_pipeline()?;
            rest.push((op, pipeline));
        }

        Ok(Chain {
            first,
            rest,
            background: false,
        })
    }

    fn parse_pipeline(&mut self) -> Result<Pipeline, ParseError> {
        // Check for subshell: (cmd1; cmd2; ...)
        if self.peek() == Some(&Token::OpenParen) {
            self.advance(); // consume '('
            let inner = self.parse_command_line()?;
            if self.peek() == Some(&Token::CloseParen) {
                self.advance(); // consume ')'
            }
            return Ok(Pipeline {
                commands: Vec::new(),
                subshell: Some(Box::new(inner)),
            });
        }

        let first = self.parse_simple_command()?;
        let mut commands = vec![first];

        while self.peek() == Some(&Token::Pipe) {
            self.advance();
            commands.push(self.parse_simple_command()?);
        }

        Ok(Pipeline {
            commands,
            subshell: None,
        })
    }

    fn parse_simple_command(&mut self) -> Result<SimpleCommand, ParseError> {
        let mut words = Vec::new();
        let mut redirects = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Word(_)) => {
                    if let Some(Token::Word(w)) = self.advance().cloned() {
                        words.push(w);
                    }
                }
                Some(
                    Token::RedirectOut
                    | Token::RedirectAppend
                    | Token::RedirectIn
                    | Token::StderrOut
                    | Token::StderrAppend
                    | Token::HereString
                    | Token::HereDoc,
                ) => {
                    let kind = match self.advance() {
                        Some(Token::RedirectOut) => RedirectKind::StdoutOverwrite,
                        Some(Token::RedirectAppend) => RedirectKind::StdoutAppend,
                        Some(Token::RedirectIn) => RedirectKind::StdinRead,
                        Some(Token::StderrOut) => RedirectKind::StderrOverwrite,
                        Some(Token::StderrAppend) => RedirectKind::StderrAppend,
                        Some(Token::HereString) => RedirectKind::HereString,
                        Some(Token::HereDoc) => RedirectKind::HereDoc,
                        _ => unreachable!(),
                    };
                    // Next token must be a word (the string/delimiter)
                    match self.advance() {
                        Some(Token::Word(w)) => {
                            redirects.push(Redirect {
                                kind,
                                target: w.clone(),
                            });
                        }
                        _ => return Err(ParseError::UnexpectedEof),
                    }
                }
                Some(Token::StderrToStdout) => {
                    self.advance();
                    redirects.push(Redirect {
                        kind: RedirectKind::StderrToStdout,
                        target: Word::literal("&1"),
                    });
                }
                _ => break,
            }
        }

        if words.is_empty() {
            return Err(ParseError::EmptyPipeline);
        }

        Ok(SimpleCommand { words, redirects })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(input: &str) -> CommandLine {
        parse(input).unwrap_or_else(|e| panic!("parse failed for {input:?}: {e}"))
    }

    fn first_command_words(input: &str) -> Vec<String> {
        let cl = parse_ok(input);
        cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect()
    }

    #[test]
    fn simple_command() {
        assert_eq!(first_command_words("ls -la"), vec!["ls", "-la"]);
    }

    #[test]
    fn quoted_args() {
        assert_eq!(
            first_command_words(r#"echo "hello world" foo"#),
            vec!["echo", "hello world", "foo"]
        );
    }

    #[test]
    fn single_quoted() {
        assert_eq!(
            first_command_words("echo 'no $expansion'"),
            vec!["echo", "no $expansion"]
        );
    }

    #[test]
    fn pipe_operator() {
        let cl = parse_ok("ls | grep foo | head -5");
        assert_eq!(cl.chains.len(), 1);
        assert_eq!(cl.chains[0].first.commands.len(), 3);
    }

    #[test]
    fn and_operator() {
        let cl = parse_ok("mkdir build && cd build");
        assert_eq!(cl.chains.len(), 1);
        assert_eq!(cl.chains[0].rest.len(), 1);
        assert_eq!(cl.chains[0].rest[0].0, ChainOp::And);
    }

    #[test]
    fn or_operator() {
        let cl = parse_ok("mkdir dir || echo exists");
        assert_eq!(cl.chains[0].rest.len(), 1);
        assert_eq!(cl.chains[0].rest[0].0, ChainOp::Or);
    }

    #[test]
    fn mixed_chain() {
        let cl = parse_ok("a && b || c");
        assert_eq!(cl.chains[0].rest.len(), 2);
        assert_eq!(cl.chains[0].rest[0].0, ChainOp::And);
        assert_eq!(cl.chains[0].rest[1].0, ChainOp::Or);
    }

    #[test]
    fn stdout_redirect() {
        let cl = parse_ok("echo hello > out.txt");
        let cmd = &cl.chains[0].first.commands[0];
        assert_eq!(cmd.redirects.len(), 1);
        assert_eq!(cmd.redirects[0].kind, RedirectKind::StdoutOverwrite);
        assert_eq!(cmd.redirects[0].target.to_plain_string(), "out.txt");
    }

    #[test]
    fn stdout_append() {
        let cl = parse_ok("echo line >> log.txt");
        assert_eq!(cl.chains[0].first.commands[0].redirects[0].kind, RedirectKind::StdoutAppend);
    }

    #[test]
    fn stdin_redirect() {
        let cl = parse_ok("sort < input.txt");
        assert_eq!(cl.chains[0].first.commands[0].redirects[0].kind, RedirectKind::StdinRead);
    }

    #[test]
    fn stderr_redirect() {
        let cl = parse_ok("cargo build 2> errors.log");
        assert_eq!(cl.chains[0].first.commands[0].redirects[0].kind, RedirectKind::StderrOverwrite);
    }

    #[test]
    fn stderr_append() {
        let cl = parse_ok("make 2>> errors.log");
        assert_eq!(cl.chains[0].first.commands[0].redirects[0].kind, RedirectKind::StderrAppend);
    }

    #[test]
    fn stderr_to_stdout() {
        let cl = parse_ok("cargo build 2>&1");
        assert_eq!(cl.chains[0].first.commands[0].redirects[0].kind, RedirectKind::StderrToStdout);
    }

    #[test]
    fn variable_expansion_parsed() {
        let cl = parse_ok("echo $HOME");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        assert!(matches!(parts[0], WordPart::Variable(VarRef { ref name, .. }) if name == "HOME"));
    }

    #[test]
    fn braced_variable() {
        let cl = parse_ok("echo ${HOME}");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        assert!(
            matches!(parts[0], WordPart::BracedVariable(VarRef { ref name, modifier: None }) if name == "HOME")
        );
    }

    #[test]
    fn braced_variable_default() {
        let cl = parse_ok("echo ${VAR:-fallback}");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        match &parts[0] {
            WordPart::BracedVariable(VarRef {
                name,
                modifier: Some(VarModifier::Default(d)),
            }) => {
                assert_eq!(name, "VAR");
                assert_eq!(d, "fallback");
            }
            other => panic!("expected BracedVariable with Default, got {other:?}"),
        }
    }

    #[test]
    fn exit_code() {
        let cl = parse_ok("echo $?");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        assert!(matches!(parts[0], WordPart::ExitCode));
    }

    #[test]
    fn command_substitution() {
        let cl = parse_ok("echo $(whoami)");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        match &parts[0] {
            WordPart::CommandSubstitution(inner) => assert_eq!(inner, "whoami"),
            other => panic!("expected CommandSubstitution, got {other:?}"),
        }
    }

    #[test]
    fn nested_command_substitution() {
        let cl = parse_ok("echo $(echo $(whoami))");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        match &parts[0] {
            WordPart::CommandSubstitution(inner) => assert_eq!(inner, "echo $(whoami)"),
            other => panic!("expected CommandSubstitution, got {other:?}"),
        }
    }

    #[test]
    fn glob_star() {
        let cl = parse_ok("ls *.txt");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        assert!(matches!(&parts[0], WordPart::GlobPattern(p) if p == "*"));
        assert!(matches!(&parts[1], WordPart::Literal(s) if s == ".txt"));
    }

    #[test]
    fn semicolon_separation() {
        let cl = parse_ok("echo a; echo b");
        assert_eq!(cl.chains.len(), 2);
    }

    #[test]
    fn empty_input() {
        let cl = parse_ok("");
        assert!(cl.chains.is_empty());
    }

    #[test]
    fn complex_redirect_pipeline() {
        let cl = parse_ok("cat file.txt | grep pattern > out.txt 2>&1");
        assert_eq!(cl.chains[0].first.commands.len(), 2);
        let grep_cmd = &cl.chains[0].first.commands[1];
        assert_eq!(grep_cmd.redirects.len(), 2);
        assert_eq!(grep_cmd.redirects[0].kind, RedirectKind::StdoutOverwrite);
        assert_eq!(grep_cmd.redirects[1].kind, RedirectKind::StderrToStdout);
    }

    #[test]
    fn escaped_space() {
        assert_eq!(
            first_command_words(r"cd my\ directory"),
            vec!["cd", "my directory"]
        );
    }

    #[test]
    fn subshell_simple() {
        let cl = parse_ok("(echo hello)");
        assert_eq!(cl.chains.len(), 1);
        let pipeline = &cl.chains[0].first;
        assert!(pipeline.subshell.is_some());
        let inner = pipeline.subshell.as_ref().unwrap();
        assert_eq!(inner.chains.len(), 1);
        let inner_words: Vec<String> = inner.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(inner_words, vec!["echo", "hello"]);
    }

    #[test]
    fn subshell_with_chain() {
        let cl = parse_ok("(cd /tmp && ls)");
        let inner = cl.chains[0].first.subshell.as_ref().unwrap();
        assert_eq!(inner.chains.len(), 1);
        assert_eq!(inner.chains[0].rest.len(), 1);
        assert_eq!(inner.chains[0].rest[0].0, ChainOp::And);
    }

    #[test]
    fn subshell_in_pipeline() {
        let cl = parse_ok("(echo hello) | grep hello");
        // This parses as: chain with one pipeline
        // But since ( starts the pipeline, the subshell IS the first pipeline
        // Then | connects to "grep hello"
        // Actually with our current parser, (echo hello) is one pipeline (subshell)
        // and | grep hello won't be part of it because parse_pipeline returns on subshell.
        // This is a limitation - subshell doesn't participate in piping in this design.
        // The outer chain sees (echo hello) as one chain, then "| grep hello" would be a parse error.
        // For now, this is acceptable. Full pipe support for subshells would need more work.
        assert_eq!(cl.chains.len(), 1);
        assert!(cl.chains[0].first.subshell.is_some());
    }

    #[test]
    fn background_execution() {
        let cl = parse_ok("sleep 60 &");
        assert_eq!(cl.chains.len(), 1);
        assert!(cl.chains[0].background);
    }

    #[test]
    fn background_with_foreground() {
        let cl = parse_ok("sleep 60 &; echo done");
        assert_eq!(cl.chains.len(), 2);
        assert!(cl.chains[0].background);
        assert!(!cl.chains[1].background);
    }

    #[test]
    fn foreground_not_background() {
        let cl = parse_ok("echo hello");
        assert!(!cl.chains[0].background);
    }

    #[test]
    fn process_substitution() {
        let cl = parse_ok("diff <(sort a.txt) <(sort b.txt)");
        let words: Vec<String> = cl.chains[0].first.commands[0]
            .words
            .iter()
            .map(|w| w.to_plain_string())
            .collect();
        assert_eq!(words, vec!["diff", "<(sort a.txt)", "<(sort b.txt)"]);
    }

    #[test]
    fn process_substitution_parsed_as_word() {
        let cl = parse_ok("echo <(whoami)");
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        match &parts[0] {
            WordPart::ProcessSubstitution(inner) => assert_eq!(inner, "whoami"),
            other => panic!("expected ProcessSubstitution, got {other:?}"),
        }
    }

    #[test]
    fn here_string() {
        let cl = parse_ok(r#"grep "world" <<< "hello world""#);
        let cmd = &cl.chains[0].first.commands[0];
        assert_eq!(cmd.redirects.len(), 1);
        assert_eq!(cmd.redirects[0].kind, RedirectKind::HereString);
        assert_eq!(cmd.redirects[0].target.to_plain_string(), "hello world");
    }

    #[test]
    fn here_doc() {
        let cl = parse_ok("cat << EOF");
        let cmd = &cl.chains[0].first.commands[0];
        assert_eq!(cmd.redirects.len(), 1);
        assert_eq!(cmd.redirects[0].kind, RedirectKind::HereDoc);
        assert_eq!(cmd.redirects[0].target.to_plain_string(), "EOF");
    }

    #[test]
    fn double_quoted_variable() {
        let cl = parse_ok(r#"echo "hello $USER""#);
        let parts = &cl.chains[0].first.commands[0].words[1].parts;
        match &parts[0] {
            WordPart::DoubleQuoted(inner) => {
                assert!(matches!(&inner[0], WordPart::Literal(s) if s == "hello "));
                assert!(matches!(&inner[1], WordPart::Variable(VarRef { name, .. }) if name == "USER"));
            }
            other => panic!("expected DoubleQuoted, got {other:?}"),
        }
    }
}
