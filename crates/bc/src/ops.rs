use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct BcState {
    pub scale: u32,
    pub math_lib: bool,
}

impl BcState {
    pub fn new(math_lib: bool) -> Self {
        Self {
            scale: if math_lib { 20 } else { 0 },
            math_lib,
        }
    }
}

pub fn bc_repl(input: &mut dyn BufRead, output: &mut dyn Write, state: &mut BcState) -> std::io::Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        if input.read_line(&mut line)? == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "quit" {
            break;
        }

        // Handle scale assignment
        if let Some(rest) = trimmed.strip_prefix("scale") {
            let rest = rest.trim();
            if let Some(val) = rest.strip_prefix('=')
                && let Ok(s) = val.trim().parse::<u32>() {
                    state.scale = s;
                    continue;
            }
            if rest.is_empty() {
                writeln!(output, "{}", state.scale)?;
                output.flush()?;
                continue;
            }
        }

        match eval_expression(trimmed, state) {
            Ok(val) => {
                let formatted = format_value(val, state.scale);
                writeln!(output, "{formatted}")?;
                output.flush()?;
            }
            Err(e) => {
                writeln!(output, "error: {e}")?;
                output.flush()?;
            }
        }
    }
    Ok(())
}

pub fn eval_expression(expr: &str, state: &BcState) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let mut pos = 0;
    let result = parse_expr(&tokens, &mut pos, state)?;
    if pos < tokens.len() {
        return Err(format!("unexpected token: {:?}", tokens[pos]));
    }
    Ok(result)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    Func(String),
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => { i += 1; }
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '%' => { tokens.push(Token::Percent); i += 1; }
            '^' => { tokens.push(Token::Caret); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num = num_str.parse::<f64>().map_err(|e| e.to_string())?;
                tokens.push(Token::Number(num));
            }
            c if c.is_ascii_alphabetic() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                tokens.push(Token::Func(name));
            }
            c => return Err(format!("unexpected character: '{c}'")),
        }
    }
    Ok(tokens)
}

// Expression parser with precedence climbing
fn parse_expr(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    parse_add(tokens, pos, state)
}

fn parse_add(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    let mut left = parse_mul(tokens, pos, state)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Plus => { *pos += 1; left += parse_mul(tokens, pos, state)?; }
            Token::Minus => { *pos += 1; left -= parse_mul(tokens, pos, state)?; }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_mul(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    let mut left = parse_power(tokens, pos, state)?;
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Star => { *pos += 1; left *= parse_power(tokens, pos, state)?; }
            Token::Slash => {
                *pos += 1;
                let right = parse_power(tokens, pos, state)?;
                if right == 0.0 {
                    return Err("division by zero".to_string());
                }
                left /= right;
            }
            Token::Percent => {
                *pos += 1;
                let right = parse_power(tokens, pos, state)?;
                if right == 0.0 {
                    return Err("division by zero".to_string());
                }
                left %= right;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_power(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    let base = parse_unary(tokens, pos, state)?;
    if *pos < tokens.len() && tokens[*pos] == Token::Caret {
        *pos += 1;
        let exp = parse_power(tokens, pos, state)?; // right-associative
        Ok(base.powf(exp))
    } else {
        Ok(base)
    }
}

fn parse_unary(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    if *pos < tokens.len() && tokens[*pos] == Token::Minus {
        *pos += 1;
        let val = parse_primary(tokens, pos, state)?;
        return Ok(-val);
    }
    if *pos < tokens.len() && tokens[*pos] == Token::Plus {
        *pos += 1;
    }
    parse_primary(tokens, pos, state)
}

fn parse_primary(tokens: &[Token], pos: &mut usize, state: &BcState) -> Result<f64, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of expression".to_string());
    }

    match &tokens[*pos] {
        Token::Number(n) => {
            let val = *n;
            *pos += 1;
            Ok(val)
        }
        Token::LParen => {
            *pos += 1;
            let val = parse_expr(tokens, pos, state)?;
            if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                return Err("missing closing parenthesis".to_string());
            }
            *pos += 1;
            Ok(val)
        }
        Token::Func(name) => {
            let name = name.clone();
            *pos += 1;
            if *pos < tokens.len() && tokens[*pos] == Token::LParen {
                *pos += 1;
                let arg = parse_expr(tokens, pos, state)?;
                if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                    return Err("missing closing parenthesis".to_string());
                }
                *pos += 1;
                eval_func(&name, arg, state)
            } else {
                Err(format!("unknown variable: '{name}'"))
            }
        }
        other => Err(format!("unexpected token: {other:?}")),
    }
}

fn eval_func(name: &str, arg: f64, state: &BcState) -> Result<f64, String> {
    if !state.math_lib {
        return Err(format!("function '{name}' requires -l flag"));
    }
    match name {
        "s" => Ok(arg.sin()),
        "c" => Ok(arg.cos()),
        "a" => Ok(arg.atan()),
        "l" => {
            if arg <= 0.0 {
                return Err("logarithm of non-positive number".to_string());
            }
            Ok(arg.ln())
        }
        "e" => Ok(arg.exp()),
        "sqrt" => {
            if arg < 0.0 {
                return Err("square root of negative number".to_string());
            }
            Ok(arg.sqrt())
        }
        _ => Err(format!("unknown function: '{name}'")),
    }
}

pub fn format_value(val: f64, scale: u32) -> String {
    if scale == 0 {
        // Truncate toward zero like bc does
        let truncated = val as i64;
        truncated.to_string()
    } else {
        format!("{val:.prec$}", prec = scale as usize)
    }
}
