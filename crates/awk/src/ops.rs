use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use regex::Regex;

use crate::cli::AwkConfig;

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Literals
    Number(f64),
    Str(String),
    Regex(String),
    // Identifiers
    Ident(String),
    FieldRef, // $
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Match,
    NotMatch,
    And,
    Or,
    Not,
    Incr,
    Decr,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    Newline,
    // Keywords
    Begin,
    End,
    If,
    Else,
    While,
    For,
    Print,
    Printf,
    Next,
    Exit,
    // Special
    Eof,
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> char {
        if self.pos < self.chars.len() {
            self.chars[self.pos]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.chars.len() {
            let c = self.peek();
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else if c == '#' {
                // Comment until end of line
                while self.pos < self.chars.len() && self.peek() != '\n' {
                    self.advance();
                }
            } else if c == '\\' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '\n' {
                // Line continuation
                self.advance();
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Track whether the previous token could end an expression.
    /// Used to decide if `/` starts a regex or is division.
    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            if self.pos >= self.chars.len() {
                tokens.push(Token::Eof);
                break;
            }

            let c = self.peek();

            // Newline
            if c == '\n' {
                self.advance();
                // Collapse multiple newlines and skip newlines after certain tokens
                if let Some(last) = tokens.last() {
                    match last {
                        Token::Newline
                        | Token::Semicolon
                        | Token::LBrace
                        | Token::Comma
                        | Token::And
                        | Token::Or => {}
                        _ => tokens.push(Token::Newline),
                    }
                }
                continue;
            }

            // String literal
            if c == '"' {
                tokens.push(self.read_string()?);
                continue;
            }

            // Number
            if c.is_ascii_digit() || (c == '.' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1].is_ascii_digit()) {
                tokens.push(self.read_number());
                continue;
            }

            // Regex literal – only when `/` cannot be division
            if c == '/' && !Self::prev_could_be_value(&tokens) {
                tokens.push(self.read_regex()?);
                continue;
            }

            // Identifier / keyword
            if c.is_ascii_alphabetic() || c == '_' {
                tokens.push(self.read_ident());
                continue;
            }

            // Operators and delimiters
            self.advance();
            let tok = match c {
                '+' => {
                    if self.peek() == '+' {
                        self.advance();
                        Token::Incr
                    } else if self.peek() == '=' {
                        self.advance();
                        Token::PlusAssign
                    } else {
                        Token::Plus
                    }
                }
                '-' => {
                    if self.peek() == '-' {
                        self.advance();
                        Token::Decr
                    } else if self.peek() == '=' {
                        self.advance();
                        Token::MinusAssign
                    } else {
                        Token::Minus
                    }
                }
                '*' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::StarAssign
                    } else {
                        Token::Star
                    }
                }
                '/' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::SlashAssign
                    } else {
                        Token::Slash
                    }
                }
                '%' => Token::Percent,
                '^' => Token::Caret,
                '=' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::Eq
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::Ne
                    } else if self.peek() == '~' {
                        self.advance();
                        Token::NotMatch
                    } else {
                        Token::Not
                    }
                }
                '<' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::Le
                    } else {
                        Token::Lt
                    }
                }
                '>' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::Ge
                    } else {
                        Token::Gt
                    }
                }
                '~' => Token::Match,
                '&' => {
                    if self.peek() == '&' {
                        self.advance();
                        Token::And
                    } else {
                        return Err("unexpected '&' (use '&&' for logical AND)".to_string());
                    }
                }
                '|' => {
                    if self.peek() == '|' {
                        self.advance();
                        Token::Or
                    } else {
                        return Err("unexpected '|' (use '||' for logical OR)".to_string());
                    }
                }
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ';' => Token::Semicolon,
                ',' => Token::Comma,
                '$' => Token::FieldRef,
                _ => return Err(format!("unexpected character: '{c}'")),
            };
            tokens.push(tok);
        }

        Ok(tokens)
    }

    fn prev_could_be_value(tokens: &[Token]) -> bool {
        matches!(
            tokens.last(),
            Some(Token::Number(_))
                | Some(Token::Str(_))
                | Some(Token::Ident(_))
                | Some(Token::RParen)
                | Some(Token::Incr)
                | Some(Token::Decr)
        )
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening "
        let mut s = String::new();
        loop {
            if self.pos >= self.chars.len() {
                return Err("unterminated string".to_string());
            }
            let c = self.advance();
            if c == '"' {
                break;
            }
            if c == '\\' {
                if self.pos >= self.chars.len() {
                    return Err("unterminated string escape".to_string());
                }
                let esc = self.advance();
                match esc {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    '/' => s.push('/'),
                    _ => {
                        s.push('\\');
                        s.push(esc);
                    }
                }
            } else {
                s.push(c);
            }
        }
        Ok(Token::Str(s))
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
            self.pos += 1;
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        Token::Number(s.parse::<f64>().unwrap_or(0.0))
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_alphanumeric() || self.chars[self.pos] == '_') {
            self.pos += 1;
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        match s.as_str() {
            "BEGIN" => Token::Begin,
            "END" => Token::End,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "print" => Token::Print,
            "printf" => Token::Printf,
            "next" => Token::Next,
            "exit" => Token::Exit,
            _ => Token::Ident(s),
        }
    }

    fn read_regex(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening /
        let mut s = String::new();
        loop {
            if self.pos >= self.chars.len() {
                return Err("unterminated regex".to_string());
            }
            let c = self.advance();
            if c == '/' {
                break;
            }
            if c == '\\' {
                if self.pos >= self.chars.len() {
                    return Err("unterminated regex escape".to_string());
                }
                let esc = self.advance();
                s.push('\\');
                s.push(esc);
            } else {
                s.push(c);
            }
        }
        Ok(Token::Regex(s))
    }
}

// ---------------------------------------------------------------------------
// AST
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Match,
    NotMatch,
    Not,
    Neg,
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Str(String),
    Field(Box<Expr>),
    Var(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    UnaryOp(Op, Box<Expr>),
    Assign(String, Box<Expr>),
    FieldAssign(Box<Expr>, Box<Expr>),
    CompoundAssign(String, Op, Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    Regex(String),
    Call(String, Vec<Expr>),
    PreIncr(String),
    PreDecr(String),
    PostIncr(String),
    PostDecr(String),
}

#[derive(Debug, Clone)]
enum Stmt {
    Print(Vec<Expr>, Option<Box<Expr>>),
    Printf(Vec<Expr>),
    Expression(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Box<Stmt>, Expr, Box<Stmt>, Box<Stmt>),
    Block(Vec<Stmt>),
    Next,
    Exit(Option<Expr>),
}

#[derive(Debug, Clone)]
enum Pattern {
    Begin,
    End,
    Expr(Expr),
}

#[derive(Debug, Clone)]
struct Rule {
    pattern: Option<Pattern>,
    action: Stmt,
}

#[derive(Debug, Clone)]
struct Program {
    rules: Vec<Rule>,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::Eof
        }
    }

    fn advance(&mut self) -> Token {
        let tok = if self.pos < self.tokens.len() {
            self.tokens[self.pos].clone()
        } else {
            Token::Eof
        };
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let tok = self.advance();
        if std::mem::discriminant(&tok) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(format!("expected {expected:?}, got {tok:?}"))
        }
    }

    fn skip_terminators(&mut self) {
        while matches!(self.peek(), Token::Newline | Token::Semicolon) {
            self.advance();
        }
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut rules = Vec::new();
        self.skip_terminators();

        while !matches!(self.peek(), Token::Eof) {
            rules.push(self.parse_rule()?);
            self.skip_terminators();
        }

        Ok(Program { rules })
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        match self.peek() {
            Token::Begin => {
                self.advance();
                self.skip_terminators();
                let action = self.parse_block()?;
                Ok(Rule {
                    pattern: Some(Pattern::Begin),
                    action,
                })
            }
            Token::End => {
                self.advance();
                self.skip_terminators();
                let action = self.parse_block()?;
                Ok(Rule {
                    pattern: Some(Pattern::End),
                    action,
                })
            }
            Token::LBrace => {
                let action = self.parse_block()?;
                Ok(Rule {
                    pattern: None,
                    action,
                })
            }
            _ => {
                let expr = self.parse_expr()?;
                self.skip_terminators();
                if matches!(self.peek(), Token::LBrace) {
                    let action = self.parse_block()?;
                    Ok(Rule {
                        pattern: Some(Pattern::Expr(expr)),
                        action,
                    })
                } else {
                    // Pattern with default action (print $0)
                    Ok(Rule {
                        pattern: Some(Pattern::Expr(expr)),
                        action: Stmt::Print(vec![Expr::Field(Box::new(Expr::Number(0.0)))], None),
                    })
                }
            }
        }
    }

    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::LBrace)?;
        self.skip_terminators();
        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            stmts.push(self.parse_stmt()?);
            self.skip_terminators();
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::Block(stmts))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Print => {
                self.advance();
                self.parse_print_stmt()
            }
            Token::Printf => {
                self.advance();
                self.parse_printf_stmt()
            }
            Token::If => {
                self.advance();
                self.parse_if_stmt()
            }
            Token::While => {
                self.advance();
                self.parse_while_stmt()
            }
            Token::For => {
                self.advance();
                self.parse_for_stmt()
            }
            Token::LBrace => self.parse_block(),
            Token::Next => {
                self.advance();
                Ok(Stmt::Next)
            }
            Token::Exit => {
                self.advance();
                if matches!(
                    self.peek(),
                    Token::Newline | Token::Semicolon | Token::RBrace | Token::Eof
                ) {
                    Ok(Stmt::Exit(None))
                } else {
                    let expr = self.parse_expr()?;
                    Ok(Stmt::Exit(Some(expr)))
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt, String> {
        let mut args = Vec::new();
        let mut dest = None;

        if !matches!(
            self.peek(),
            Token::Newline | Token::Semicolon | Token::RBrace | Token::Eof
        ) {
            args.push(self.parse_non_assign_expr()?);
            while matches!(self.peek(), Token::Comma) {
                self.advance();
                args.push(self.parse_non_assign_expr()?);
            }
        }

        // Handle > redirect (simple file output)
        if matches!(self.peek(), Token::Gt) {
            self.advance();
            dest = Some(Box::new(self.parse_primary()?));
        }

        Ok(Stmt::Print(args, dest))
    }

    fn parse_printf_stmt(&mut self) -> Result<Stmt, String> {
        let mut args = Vec::new();
        args.push(self.parse_non_assign_expr()?);
        while matches!(self.peek(), Token::Comma) {
            self.advance();
            args.push(self.parse_non_assign_expr()?);
        }
        Ok(Stmt::Printf(args))
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        self.skip_terminators();
        let then_branch = self.parse_stmt()?;
        self.skip_terminators();
        let else_branch = if matches!(self.peek(), Token::Else) {
            self.advance();
            self.skip_terminators();
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        Ok(Stmt::If(cond, Box::new(then_branch), else_branch))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        self.skip_terminators();
        let body = self.parse_stmt()?;
        Ok(Stmt::While(cond, Box::new(body)))
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, String> {
        self.expect(&Token::LParen)?;
        let init = if matches!(self.peek(), Token::Semicolon) {
            Stmt::Block(vec![])
        } else {
            let e = self.parse_expr()?;
            Stmt::Expression(e)
        };
        self.expect(&Token::Semicolon)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        let update = if matches!(self.peek(), Token::RParen) {
            Stmt::Block(vec![])
        } else {
            let e = self.parse_expr()?;
            Stmt::Expression(e)
        };
        self.expect(&Token::RParen)?;
        self.skip_terminators();
        let body = self.parse_stmt()?;
        Ok(Stmt::For(
            Box::new(init),
            cond,
            Box::new(update),
            Box::new(body),
        ))
    }

    // Expression parsing with precedence climbing

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;

        match self.peek() {
            Token::Assign => {
                self.advance();
                let rhs = self.parse_assignment()?;
                match expr {
                    Expr::Var(name) => Ok(Expr::Assign(name, Box::new(rhs))),
                    Expr::Field(idx) => Ok(Expr::FieldAssign(idx, Box::new(rhs))),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            Token::PlusAssign => {
                self.advance();
                let rhs = self.parse_assignment()?;
                match expr {
                    Expr::Var(name) => Ok(Expr::CompoundAssign(name, Op::Add, Box::new(rhs))),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            Token::MinusAssign => {
                self.advance();
                let rhs = self.parse_assignment()?;
                match expr {
                    Expr::Var(name) => Ok(Expr::CompoundAssign(name, Op::Sub, Box::new(rhs))),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            Token::StarAssign => {
                self.advance();
                let rhs = self.parse_assignment()?;
                match expr {
                    Expr::Var(name) => Ok(Expr::CompoundAssign(name, Op::Mul, Box::new(rhs))),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            Token::SlashAssign => {
                self.advance();
                let rhs = self.parse_assignment()?;
                match expr {
                    Expr::Var(name) => Ok(Expr::CompoundAssign(name, Op::Div, Box::new(rhs))),
                    _ => Err("invalid assignment target".to_string()),
                }
            }
            _ => Ok(expr),
        }
    }

    /// Non-assignment expression (for print args where `>` is redirect, not comparison).
    fn parse_non_assign_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp(Box::new(left), Op::Or, Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_match()?;
        while matches!(self.peek(), Token::And) {
            self.advance();
            let right = self.parse_match()?;
            left = Expr::BinOp(Box::new(left), Op::And, Box::new(right));
        }
        Ok(left)
    }

    fn parse_match(&mut self) -> Result<Expr, String> {
        let left = self.parse_comparison()?;
        match self.peek() {
            Token::Match => {
                self.advance();
                let right = self.parse_comparison()?;
                Ok(Expr::BinOp(Box::new(left), Op::Match, Box::new(right)))
            }
            Token::NotMatch => {
                self.advance();
                let right = self.parse_comparison()?;
                Ok(Expr::BinOp(Box::new(left), Op::NotMatch, Box::new(right)))
            }
            _ => Ok(left),
        }
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let left = self.parse_concat()?;
        let op = match self.peek() {
            Token::Lt => Op::Lt,
            Token::Le => Op::Le,
            Token::Gt => Op::Gt,
            Token::Ge => Op::Ge,
            Token::Eq => Op::Eq,
            Token::Ne => Op::Ne,
            _ => return Ok(left),
        };
        self.advance();
        let right = self.parse_concat()?;
        Ok(Expr::BinOp(Box::new(left), op, Box::new(right)))
    }

    fn parse_concat(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;
        // String concatenation: two values adjacent with no operator
        while matches!(
            self.peek(),
            Token::Number(_)
                | Token::Str(_)
                | Token::Ident(_)
                | Token::FieldRef
                | Token::LParen
                | Token::Not
        ) {
            let right = self.parse_addition()?;
            left = Expr::Concat(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;
        loop {
            let op = match self.peek() {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek() {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                Token::Percent => Op::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let base = self.parse_unary()?;
        if matches!(self.peek(), Token::Caret) {
            self.advance();
            let exp = self.parse_power()?; // right-associative
            Ok(Expr::BinOp(Box::new(base), Op::Pow, Box::new(exp)))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp(Op::Not, Box::new(expr)))
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp(Op::Neg, Box::new(expr)))
            }
            Token::Plus => {
                self.advance();
                // unary plus is a no-op
                self.parse_unary()
            }
            Token::Incr => {
                self.advance();
                if let Token::Ident(name) = self.peek().clone() {
                    self.advance();
                    Ok(Expr::PreIncr(name))
                } else {
                    Err("expected variable after ++".to_string())
                }
            }
            Token::Decr => {
                self.advance();
                if let Token::Ident(name) = self.peek().clone() {
                    self.advance();
                    Ok(Expr::PreDecr(name))
                } else {
                    Err("expected variable after --".to_string())
                }
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let expr = self.parse_primary()?;
        match self.peek() {
            Token::Incr => {
                self.advance();
                if let Expr::Var(name) = expr {
                    Ok(Expr::PostIncr(name))
                } else {
                    Err("expected variable before ++".to_string())
                }
            }
            Token::Decr => {
                self.advance();
                if let Expr::Var(name) = expr {
                    Ok(Expr::PostDecr(name))
                } else {
                    Err("expected variable before --".to_string())
                }
            }
            _ => Ok(expr),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::Str(s) => {
                self.advance();
                Ok(Expr::Str(s))
            }
            Token::Regex(r) => {
                self.advance();
                Ok(Expr::Regex(r))
            }
            Token::FieldRef => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expr::Field(Box::new(expr)))
            }
            Token::Ident(name) => {
                self.advance();
                // Check for function call
                if matches!(self.peek(), Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RParen) {
                        args.push(self.parse_expr()?);
                        while matches!(self.peek(), Token::Comma) {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("unexpected token: {tok:?}")),
        }
    }
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

enum ControlFlow {
    Continue,
    Next,
    Exit(i32),
}

struct Env {
    vars: HashMap<String, String>,
    fields: Vec<String>,
    nr: usize,
    nf: usize,
    fs: String,
    ofs: String,
    rs: String,
    ors: String,
    filename: String,
}

impl Env {
    fn new(config: &AwkConfig) -> Self {
        let fs = if config.field_separator.is_empty() {
            " ".to_string()
        } else {
            config.field_separator.clone()
        };

        let mut vars = HashMap::new();
        for (k, v) in &config.variables {
            vars.insert(k.clone(), v.clone());
        }

        // If FS was set via -v, use that; otherwise use -F value
        let effective_fs = vars
            .get("FS")
            .cloned()
            .unwrap_or_else(|| fs.clone());

        let ofs = vars.get("OFS").cloned().unwrap_or_else(|| " ".to_string());
        let rs = vars.get("RS").cloned().unwrap_or_else(|| "\n".to_string());
        let ors = vars.get("ORS").cloned().unwrap_or_else(|| "\n".to_string());

        Self {
            vars,
            fields: vec![String::new()], // $0
            nr: 0,
            nf: 0,
            fs: effective_fs,
            ofs,
            rs,
            ors,
            filename: String::new(),
        }
    }

    fn get_var(&self, name: &str) -> String {
        match name {
            "NR" => self.nr.to_string(),
            "NF" => self.nf.to_string(),
            "FS" => self.fs.clone(),
            "OFS" => self.ofs.clone(),
            "RS" => self.rs.clone(),
            "ORS" => self.ors.clone(),
            "FILENAME" => self.filename.clone(),
            _ => self.vars.get(name).cloned().unwrap_or_default(),
        }
    }

    fn set_var(&mut self, name: &str, value: String) {
        match name {
            "NR" => self.nr = str_to_f64(&value) as usize,
            "NF" => self.nf = str_to_f64(&value) as usize,
            "FS" => self.fs = value.clone(),
            "OFS" => self.ofs = value.clone(),
            "RS" => self.rs = value.clone(),
            "ORS" => self.ors = value.clone(),
            "FILENAME" => self.filename = value.clone(),
            _ => {}
        }
        self.vars.insert(name.to_string(), value);
    }

    fn get_field(&self, idx: usize) -> String {
        if idx < self.fields.len() {
            self.fields[idx].clone()
        } else {
            String::new()
        }
    }

    fn set_field(&mut self, idx: usize, value: String) {
        while self.fields.len() <= idx {
            self.fields.push(String::new());
        }
        self.fields[idx] = value;
        // Rebuild $0 from fields
        if idx > 0 {
            self.fields[0] = self.fields[1..].join(&self.ofs);
            self.nf = self.fields.len() - 1;
        }
    }

    fn set_record(&mut self, line: &str) {
        self.fields.clear();
        self.fields.push(line.to_string()); // $0

        let parts: Vec<String> = if self.fs == " " {
            // Default: split on whitespace runs
            line.split_whitespace().map(|s| s.to_string()).collect()
        } else if self.fs.len() == 1 {
            line.split(self.fs.chars().next().unwrap())
                .map(|s| s.to_string())
                .collect()
        } else {
            // FS as regex
            match Regex::new(&self.fs) {
                Ok(re) => re.split(line).map(|s| s.to_string()).collect(),
                Err(_) => vec![line.to_string()],
            }
        };

        self.fields.extend(parts);
        self.nf = self.fields.len() - 1;
    }
}

fn str_to_f64(s: &str) -> f64 {
    let s = s.trim();
    if s.is_empty() {
        return 0.0;
    }
    // Parse leading numeric portion
    let mut end = 0;
    let bytes = s.as_bytes();
    if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
        end += 1;
    }
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b'.' {
        end += 1;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
    }
    if end == 0 {
        return 0.0;
    }
    s[..end].parse::<f64>().unwrap_or(0.0)
}

fn format_number(n: f64) -> String {
    if n == n.floor() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{:.6}", n).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn eval_expr(expr: &Expr, env: &mut Env) -> String {
    match expr {
        Expr::Number(n) => format_number(*n),
        Expr::Str(s) => s.clone(),
        Expr::Regex(r) => {
            // Bare regex in expression context: match against $0
            let s = env.get_field(0);
            match Regex::new(r) {
                Ok(re) => bool_str(re.is_match(&s)),
                Err(_) => "0".to_string(),
            }
        }
        Expr::Field(idx_expr) => {
            let idx = eval_as_number(idx_expr, env) as usize;
            env.get_field(idx)
        }
        Expr::Var(name) => env.get_var(name),
        Expr::BinOp(left, op, right) => eval_binop(left, op, right, env),
        Expr::UnaryOp(op, operand) => {
            match op {
                Op::Not => {
                    if eval_as_bool(operand, env) {
                        "0".to_string()
                    } else {
                        "1".to_string()
                    }
                }
                Op::Neg => format_number(-eval_as_number(operand, env)),
                _ => String::new(),
            }
        }
        Expr::Assign(name, val_expr) => {
            let val = eval_expr(val_expr, env);
            env.set_var(name, val.clone());
            val
        }
        Expr::FieldAssign(idx_expr, val_expr) => {
            let idx = eval_as_number(idx_expr, env) as usize;
            let val = eval_expr(val_expr, env);
            env.set_field(idx, val.clone());
            val
        }
        Expr::CompoundAssign(name, op, val_expr) => {
            let current = str_to_f64(&env.get_var(name));
            let rhs = eval_as_number(val_expr, env);
            let result = match op {
                Op::Add => current + rhs,
                Op::Sub => current - rhs,
                Op::Mul => current * rhs,
                Op::Div => {
                    if rhs == 0.0 {
                        eprintln!("awk: division by zero");
                        0.0
                    } else {
                        current / rhs
                    }
                }
                _ => current,
            };
            let s = format_number(result);
            env.set_var(name, s.clone());
            s
        }
        Expr::Concat(left, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            format!("{l}{r}")
        }
        Expr::Call(name, args) => eval_builtin(name, args, env),
        Expr::PreIncr(name) => {
            let v = str_to_f64(&env.get_var(name)) + 1.0;
            let s = format_number(v);
            env.set_var(name, s.clone());
            s
        }
        Expr::PreDecr(name) => {
            let v = str_to_f64(&env.get_var(name)) - 1.0;
            let s = format_number(v);
            env.set_var(name, s.clone());
            s
        }
        Expr::PostIncr(name) => {
            let old = env.get_var(name);
            let v = str_to_f64(&old) + 1.0;
            env.set_var(name, format_number(v));
            old
        }
        Expr::PostDecr(name) => {
            let old = env.get_var(name);
            let v = str_to_f64(&old) - 1.0;
            env.set_var(name, format_number(v));
            old
        }
    }
}

fn eval_binop(left: &Expr, op: &Op, right: &Expr, env: &mut Env) -> String {
    match op {
        Op::Add => format_number(eval_as_number(left, env) + eval_as_number(right, env)),
        Op::Sub => format_number(eval_as_number(left, env) - eval_as_number(right, env)),
        Op::Mul => format_number(eval_as_number(left, env) * eval_as_number(right, env)),
        Op::Div => {
            let l = eval_as_number(left, env);
            let r = eval_as_number(right, env);
            if r == 0.0 {
                eprintln!("awk: division by zero");
                format_number(0.0)
            } else {
                format_number(l / r)
            }
        }
        Op::Mod => {
            let l = eval_as_number(left, env);
            let r = eval_as_number(right, env);
            if r == 0.0 {
                eprintln!("awk: division by zero");
                format_number(0.0)
            } else {
                format_number(l % r)
            }
        }
        Op::Pow => format_number(eval_as_number(left, env).powf(eval_as_number(right, env))),
        Op::Eq => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) == std::cmp::Ordering::Equal)
        }
        Op::Ne => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) != std::cmp::Ordering::Equal)
        }
        Op::Lt => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) == std::cmp::Ordering::Less)
        }
        Op::Le => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) != std::cmp::Ordering::Greater)
        }
        Op::Gt => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) == std::cmp::Ordering::Greater)
        }
        Op::Ge => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            bool_str(compare_values(&l, &r) != std::cmp::Ordering::Less)
        }
        Op::And => {
            if eval_as_bool(left, env) && eval_as_bool(right, env) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Op::Or => {
            if eval_as_bool(left, env) || eval_as_bool(right, env) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Op::Match => {
            let s = eval_expr(left, env);
            let pat = match right {
                Expr::Regex(r) => r.clone(),
                _ => eval_expr(right, env),
            };
            match Regex::new(&pat) {
                Ok(re) => bool_str(re.is_match(&s)),
                Err(_) => {
                    eprintln!("awk: invalid regex: {pat}");
                    "0".to_string()
                }
            }
        }
        Op::NotMatch => {
            let s = eval_expr(left, env);
            let pat = match right {
                Expr::Regex(r) => r.clone(),
                _ => eval_expr(right, env),
            };
            match Regex::new(&pat) {
                Ok(re) => bool_str(!re.is_match(&s)),
                Err(_) => {
                    eprintln!("awk: invalid regex: {pat}");
                    "0".to_string()
                }
            }
        }
        Op::Not | Op::Neg => unreachable!("unary ops handled elsewhere"),
    }
}

fn compare_values(a: &str, b: &str) -> std::cmp::Ordering {
    // If both look numeric, compare as numbers
    let an = a.parse::<f64>();
    let bn = b.parse::<f64>();
    if let (Ok(av), Ok(bv)) = (an, bn) {
        av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
    } else {
        a.cmp(b)
    }
}

fn bool_str(b: bool) -> String {
    if b { "1".to_string() } else { "0".to_string() }
}

fn eval_as_number(expr: &Expr, env: &mut Env) -> f64 {
    let s = eval_expr(expr, env);
    str_to_f64(&s)
}

fn eval_as_bool(expr: &Expr, env: &mut Env) -> bool {
    let s = eval_expr(expr, env);
    // Non-zero number or non-empty string is true
    if s.is_empty() {
        return false;
    }
    if let Ok(n) = s.parse::<f64>() {
        n != 0.0
    } else {
        true
    }
}

fn eval_builtin(name: &str, args: &[Expr], env: &mut Env) -> String {
    match name {
        "length" => {
            let s = if args.is_empty() {
                env.get_field(0)
            } else {
                eval_expr(&args[0], env)
            };
            format_number(s.len() as f64)
        }
        "substr" => {
            if args.is_empty() {
                return String::new();
            }
            let s = eval_expr(&args[0], env);
            let start = if args.len() > 1 {
                (eval_as_number(&args[1], env) as isize - 1).max(0) as usize
            } else {
                0
            };
            if start >= s.len() {
                return String::new();
            }
            if args.len() > 2 {
                let len = eval_as_number(&args[2], env) as usize;
                let end = (start + len).min(s.len());
                s[start..end].to_string()
            } else {
                s[start..].to_string()
            }
        }
        "index" => {
            if args.len() < 2 {
                return "0".to_string();
            }
            let s = eval_expr(&args[0], env);
            let target = eval_expr(&args[1], env);
            match s.find(&target) {
                Some(pos) => format_number((pos + 1) as f64),
                None => "0".to_string(),
            }
        }
        "split" => {
            // Simplified: just return the count of fields
            if args.is_empty() {
                return "0".to_string();
            }
            let s = eval_expr(&args[0], env);
            let sep = if args.len() > 2 {
                eval_expr(&args[2], env)
            } else {
                env.fs.clone()
            };
            let count = if sep == " " {
                s.split_whitespace().count()
            } else if sep.len() == 1 {
                s.split(sep.chars().next().unwrap()).count()
            } else {
                match Regex::new(&sep) {
                    Ok(re) => re.split(&s).count(),
                    Err(_) => 1,
                }
            };
            format_number(count as f64)
        }
        "tolower" => {
            let s = if args.is_empty() {
                env.get_field(0)
            } else {
                eval_expr(&args[0], env)
            };
            s.to_lowercase()
        }
        "toupper" => {
            let s = if args.is_empty() {
                env.get_field(0)
            } else {
                eval_expr(&args[0], env)
            };
            s.to_uppercase()
        }
        "sub" => {
            if args.len() < 2 {
                return "0".to_string();
            }
            let pattern = match &args[0] {
                Expr::Regex(r) => r.clone(),
                _ => eval_expr(&args[0], env),
            };
            let replacement = eval_expr(&args[1], env);
            let target = if args.len() > 2 {
                eval_expr(&args[2], env)
            } else {
                env.get_field(0)
            };
            match Regex::new(&pattern) {
                Ok(re) => {
                    if re.is_match(&target) {
                        let result = re.replacen(&target, 1, replacement.as_str()).to_string();
                        if args.len() > 2 {
                            // Try to set back to the variable
                            if let Expr::Var(ref name) = args[2] {
                                env.set_var(name, result);
                            } else if let Expr::Field(ref idx_expr) = args[2] {
                                let idx = eval_as_number(idx_expr, env) as usize;
                                env.set_field(idx, result);
                            }
                        } else {
                            let r = result.clone();
                            env.set_field(0, r);
                            // Also re-split
                        }
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                }
                Err(_) => "0".to_string(),
            }
        }
        "gsub" => {
            if args.len() < 2 {
                return "0".to_string();
            }
            let pattern = match &args[0] {
                Expr::Regex(r) => r.clone(),
                _ => eval_expr(&args[0], env),
            };
            let replacement = eval_expr(&args[1], env);
            let target = if args.len() > 2 {
                eval_expr(&args[2], env)
            } else {
                env.get_field(0)
            };
            match Regex::new(&pattern) {
                Ok(re) => {
                    let count = re.find_iter(&target).count();
                    if count > 0 {
                        let result = re.replace_all(&target, replacement.as_str()).to_string();
                        if args.len() > 2 {
                            if let Expr::Var(ref name) = args[2] {
                                env.set_var(name, result);
                            } else if let Expr::Field(ref idx_expr) = args[2] {
                                let idx = eval_as_number(idx_expr, env) as usize;
                                env.set_field(idx, result);
                            }
                        } else {
                            env.set_field(0, result);
                        }
                    }
                    format_number(count as f64)
                }
                Err(_) => "0".to_string(),
            }
        }
        "int" => {
            if args.is_empty() {
                return "0".to_string();
            }
            let n = eval_as_number(&args[0], env);
            format_number(n.trunc())
        }
        "sqrt" => {
            if args.is_empty() {
                return "0".to_string();
            }
            let n = eval_as_number(&args[0], env);
            format_number(n.sqrt())
        }
        "sin" => {
            if args.is_empty() {
                return "0".to_string();
            }
            format_number(eval_as_number(&args[0], env).sin())
        }
        "cos" => {
            if args.is_empty() {
                return "0".to_string();
            }
            format_number(eval_as_number(&args[0], env).cos())
        }
        "log" => {
            if args.is_empty() {
                return "0".to_string();
            }
            format_number(eval_as_number(&args[0], env).ln())
        }
        "exp" => {
            if args.is_empty() {
                return "0".to_string();
            }
            format_number(eval_as_number(&args[0], env).exp())
        }
        "sprintf" => {
            if args.is_empty() {
                return String::new();
            }
            let fmt = eval_expr(&args[0], env);
            let vals: Vec<String> = args[1..].iter().map(|a| eval_expr(a, env)).collect();
            awk_sprintf(&fmt, &vals)
        }
        _ => {
            eprintln!("awk: unknown function: {name}");
            String::new()
        }
    }
}

fn awk_sprintf(fmt: &str, args: &[String]) -> String {
    let mut result = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    let mut arg_idx = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 1 < chars.len() {
            i += 1;
            if chars[i] == '%' {
                result.push('%');
                i += 1;
                continue;
            }
            // Parse format specifier: flags, width, precision, conversion
            let mut spec = String::from('%');
            // Flags
            while i < chars.len() && "-+ #0".contains(chars[i]) {
                spec.push(chars[i]);
                i += 1;
            }
            // Width
            while i < chars.len() && chars[i].is_ascii_digit() {
                spec.push(chars[i]);
                i += 1;
            }
            // Precision
            if i < chars.len() && chars[i] == '.' {
                spec.push('.');
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    spec.push(chars[i]);
                    i += 1;
                }
            }
            // Conversion
            if i < chars.len() {
                let conv = chars[i];
                i += 1;
                let val = if arg_idx < args.len() {
                    &args[arg_idx]
                } else {
                    ""
                };
                arg_idx += 1;
                match conv {
                    'd' | 'i' => {
                        let n = str_to_f64(val) as i64;
                        // Simple width handling
                        result.push_str(&format_with_spec_int(&spec, n));
                    }
                    'f' | 'e' | 'g' => {
                        let n = str_to_f64(val);
                        result.push_str(&format_with_spec_float(&spec, conv, n));
                    }
                    's' => {
                        result.push_str(&format_with_spec_str(&spec, val));
                    }
                    'c' => {
                        if let Some(ch) = val.chars().next() {
                            result.push(ch);
                        }
                    }
                    _ => {
                        result.push('%');
                        result.push(conv);
                    }
                }
            }
        } else if chars[i] == '\\' && i + 1 < chars.len() {
            i += 1;
            match chars[i] {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                _ => {
                    result.push('\\');
                    result.push(chars[i]);
                }
            }
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

fn format_with_spec_int(spec: &str, n: i64) -> String {
    // Parse width from spec for basic formatting
    let inner = &spec[1..]; // skip %
    let left_align = inner.starts_with('-');
    let zero_pad = inner.starts_with('0') && !left_align;
    let trimmed = inner.trim_start_matches(['-', '+', ' ', '#', '0']);
    let width: usize = trimmed.parse().unwrap_or(0);
    let s = n.to_string();
    if width == 0 {
        return s;
    }
    if left_align {
        format!("{:<width$}", s)
    } else if zero_pad {
        format!("{:0>width$}", s)
    } else {
        format!("{:>width$}", s)
    }
}

fn format_with_spec_float(spec: &str, conv: char, n: f64) -> String {
    let inner = &spec[1..];
    let left_align = inner.starts_with('-');
    let trimmed = inner.trim_start_matches(['-', '+', ' ', '#', '0']);
    let (width_str, prec_str) = if let Some(dot) = trimmed.find('.') {
        (&trimmed[..dot], &trimmed[dot + 1..])
    } else {
        (trimmed, "")
    };
    let width: usize = width_str.parse().unwrap_or(0);
    let precision: usize = if prec_str.is_empty() {
        6
    } else {
        prec_str.parse().unwrap_or(6)
    };

    let s = match conv {
        'f' => format!("{:.prec$}", n, prec = precision),
        'e' => format!("{:.prec$e}", n, prec = precision),
        'g' => {
            // %g: use shortest representation
            let f_str = format!("{:.prec$}", n, prec = precision);
            let e_str = format!("{:.prec$e}", n, prec = precision);
            if f_str.len() <= e_str.len() { f_str } else { e_str }
        }
        _ => format!("{n}"),
    };
    if width == 0 {
        return s;
    }
    if left_align {
        format!("{:<width$}", s)
    } else {
        format!("{:>width$}", s)
    }
}

fn format_with_spec_str(spec: &str, s: &str) -> String {
    let inner = &spec[1..];
    let left_align = inner.starts_with('-');
    let trimmed = inner.trim_start_matches(['-', '+', ' ', '#', '0']);
    let width: usize = trimmed.parse().unwrap_or(0);
    if width == 0 {
        return s.to_string();
    }
    if left_align {
        format!("{:<width$}", s)
    } else {
        format!("{:>width$}", s)
    }
}

fn exec_stmt(stmt: &Stmt, env: &mut Env, output: &mut dyn Write) -> ControlFlow {
    match stmt {
        Stmt::Print(args, dest) => {
            let parts: Vec<String> = if args.is_empty() {
                vec![env.get_field(0)]
            } else {
                args.iter().map(|a| eval_expr(a, env)).collect()
            };
            let line = parts.join(&env.ofs);
            let out_line = format!("{}{}", line, env.ors);

            if let Some(file_expr) = dest {
                let filename = eval_expr(file_expr, env);
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&filename)
                {
                    let _ = f.write_all(out_line.as_bytes());
                } else {
                    eprintln!("awk: can't redirect to {filename}");
                }
            } else {
                let _ = output.write_all(out_line.as_bytes());
            }
            ControlFlow::Continue
        }
        Stmt::Printf(args) => {
            if args.is_empty() {
                return ControlFlow::Continue;
            }
            let fmt = eval_expr(&args[0], env);
            let vals: Vec<String> = args[1..].iter().map(|a| eval_expr(a, env)).collect();
            let s = awk_sprintf(&fmt, &vals);
            let _ = output.write_all(s.as_bytes());
            ControlFlow::Continue
        }
        Stmt::Expression(expr) => {
            eval_expr(expr, env);
            ControlFlow::Continue
        }
        Stmt::If(cond, then_branch, else_branch) => {
            if eval_as_bool(cond, env) {
                exec_stmt(then_branch, env, output)
            } else if let Some(eb) = else_branch {
                exec_stmt(eb, env, output)
            } else {
                ControlFlow::Continue
            }
        }
        Stmt::While(cond, body) => {
            let mut iter_count = 0;
            while eval_as_bool(cond, env) {
                match exec_stmt(body, env, output) {
                    ControlFlow::Continue => {}
                    cf => return cf,
                }
                iter_count += 1;
                if iter_count > 1_000_000 {
                    eprintln!("awk: while loop exceeded 1000000 iterations, aborting");
                    break;
                }
            }
            ControlFlow::Continue
        }
        Stmt::For(init, cond, update, body) => {
            exec_stmt(init, env, output);
            let mut iter_count = 0;
            while eval_as_bool(cond, env) {
                match exec_stmt(body, env, output) {
                    ControlFlow::Continue => {}
                    cf => return cf,
                }
                exec_stmt(update, env, output);
                iter_count += 1;
                if iter_count > 1_000_000 {
                    eprintln!("awk: for loop exceeded 1000000 iterations, aborting");
                    break;
                }
            }
            ControlFlow::Continue
        }
        Stmt::Block(stmts) => {
            for s in stmts {
                match exec_stmt(s, env, output) {
                    ControlFlow::Continue => {}
                    cf => return cf,
                }
            }
            ControlFlow::Continue
        }
        Stmt::Next => ControlFlow::Next,
        Stmt::Exit(code_expr) => {
            let code = code_expr
                .as_ref()
                .map(|e| eval_as_number(e, env) as i32)
                .unwrap_or(0);
            ControlFlow::Exit(code)
        }
    }
}

fn pattern_matches(pattern: &Pattern, env: &mut Env) -> bool {
    match pattern {
        Pattern::Begin | Pattern::End => false, // handled separately
        Pattern::Expr(expr) => {
            // Check if it's a regex match against $0
            eval_as_bool(expr, env)
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn run(
    program: &str,
    config: &AwkConfig,
    input: &mut dyn BufRead,
    output: &mut dyn Write,
) -> io::Result<i32> {
    // Lex
    let mut lexer = Lexer::new(program);
    let tokens = lexer.tokenize().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // Parse
    let mut parser = Parser::new(tokens);
    let prog = parser
        .parse_program()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // Initialize environment
    let mut env = Env::new(config);

    // Execute BEGIN rules
    for rule in &prog.rules {
        if matches!(rule.pattern, Some(Pattern::Begin))
            && let ControlFlow::Exit(code) = exec_stmt(&rule.action, &mut env, output)
        {
            return Ok(code);
        }
    }

    // Process input lines
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = input.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        // Strip trailing newline
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }

        env.nr += 1;
        env.set_record(&line);

        let mut do_next = false;
        for rule in &prog.rules {
            if matches!(rule.pattern, Some(Pattern::Begin) | Some(Pattern::End)) {
                continue;
            }

            let should_run = match &rule.pattern {
                None => true,
                Some(pat) => pattern_matches(pat, &mut env),
            };

            if should_run {
                match exec_stmt(&rule.action, &mut env, output) {
                    ControlFlow::Continue => {}
                    ControlFlow::Next => {
                        do_next = true;
                        break;
                    }
                    ControlFlow::Exit(code) => {
                        // Still run END rules
                        for end_rule in &prog.rules {
                            if matches!(end_rule.pattern, Some(Pattern::End)) {
                                exec_stmt(&end_rule.action, &mut env, output);
                            }
                        }
                        return Ok(code);
                    }
                }
            }
        }
        if do_next {
            continue;
        }
    }

    // Execute END rules
    let mut exit_code = 0;
    for rule in &prog.rules {
        if matches!(rule.pattern, Some(Pattern::End))
            && let ControlFlow::Exit(code) = exec_stmt(&rule.action, &mut env, output)
        {
            exit_code = code;
            break;
        }
    }

    Ok(exit_code)
}
