use std::io::{self, BufRead, Write};

use regex::Regex;

use crate::cli::SedConfig;

#[derive(Debug)]
pub enum Address {
    Line(usize),
    Last,
    Regex(Regex),
}

#[derive(Debug)]
pub enum AddressRange {
    None,
    Single(Address),
    Range(Address, Address),
}

#[derive(Debug)]
pub enum SedCommand {
    Substitute {
        pattern: Regex,
        replacement: String,
        global: bool,
        print: bool,
    },
    Delete,
    Print,
    Quit,
    Transliterate {
        from: Vec<char>,
        to: Vec<char>,
    },
    AppendText(String),
    InsertText(String),
    ReplaceText(String),
}

#[derive(Debug)]
pub struct CompiledCommand {
    pub address: AddressRange,
    pub command: SedCommand,
}

fn parse_address(s: &str, extended: bool) -> Result<(Address, &str), String> {
    let s = s.trim_start();
    if s.is_empty() {
        return Err("expected address".to_string());
    }

    if let Some(rest) = s.strip_prefix('$') {
        return Ok((Address::Last, rest));
    }

    if s.as_bytes()[0].is_ascii_digit() {
        let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
        let num: usize = s[..end]
            .parse()
            .map_err(|e| format!("invalid line number: {e}"))?;
        return Ok((Address::Line(num), &s[end..]));
    }

    if let Some(rest) = s.strip_prefix('/') {
        let mut escaped = false;
        let mut end = None;
        for (i, c) in rest.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c == '/' {
                end = Some(i);
                break;
            }
        }
        let end = end.ok_or("unterminated address regex")?;
        let pattern = &rest[..end];
        let remaining = &rest[end + 1..];
        let re = build_regex(pattern, extended, false)?;
        return Ok((Address::Regex(re), remaining));
    }

    Err(format!("unexpected address character: {}", &s[..1]))
}

fn build_regex(pattern: &str, _extended: bool, ignore_case: bool) -> Result<Regex, String> {
    let mut re_pattern = String::new();
    if ignore_case {
        re_pattern.push_str("(?i)");
    }
    re_pattern.push_str(pattern);
    Regex::new(&re_pattern).map_err(|e| format!("invalid regex: {e}"))
}

fn parse_substitute(s: &str, extended: bool) -> Result<(SedCommand, &str), String> {
    if s.is_empty() {
        return Err("empty substitute command".to_string());
    }
    let delim = s.chars().next().unwrap();
    let rest = &s[delim.len_utf8()..];

    // Find pattern end
    let (pattern_str, after_pattern) = find_delimited(rest, delim)?;
    let (replacement, after_replacement) = find_delimited(after_pattern, delim)?;

    // Parse flags
    let mut global = false;
    let mut print = false;
    let mut ignore_case = false;
    let flag_end = after_replacement
        .find([';', '\n', '}'])
        .unwrap_or(after_replacement.len());
    let flags_str = &after_replacement[..flag_end];
    let remaining = &after_replacement[flag_end..];

    for c in flags_str.chars() {
        match c {
            'g' => global = true,
            'p' => print = true,
            'i' | 'I' => ignore_case = true,
            ' ' | '\t' => {}
            _ => return Err(format!("unknown flag: '{c}'")),
        }
    }

    let re = build_regex(&pattern_str, extended, ignore_case)?;

    Ok((
        SedCommand::Substitute {
            pattern: re,
            replacement,
            global,
            print,
        },
        remaining,
    ))
}

fn find_delimited(s: &str, delim: char) -> Result<(String, &str), String> {
    let mut result = String::new();
    let mut escaped = false;
    let mut end_pos = None;
    let mut byte_pos = 0;

    for (i, c) in s.char_indices() {
        if escaped {
            if c != delim {
                result.push('\\');
            }
            result.push(c);
            escaped = false;
            byte_pos = i + c.len_utf8();
            continue;
        }
        if c == '\\' {
            escaped = true;
            byte_pos = i + c.len_utf8();
            continue;
        }
        if c == delim {
            end_pos = Some(i);
            break;
        }
        result.push(c);
        byte_pos = i + c.len_utf8();
    }

    match end_pos {
        Some(pos) => Ok((result, &s[pos + delim.len_utf8()..])),
        None => {
            // Allow unterminated final delimiter
            let _ = byte_pos;
            Ok((result, ""))
        }
    }
}

fn parse_transliterate(s: &str) -> Result<(SedCommand, &str), String> {
    if s.is_empty() {
        return Err("empty transliterate command".to_string());
    }
    let delim = s.chars().next().unwrap();
    let rest = &s[delim.len_utf8()..];

    let (from_str, after_from) = find_delimited(rest, delim)?;
    let (to_str, remaining) = find_delimited(after_from, delim)?;

    let from: Vec<char> = from_str.chars().collect();
    let to: Vec<char> = to_str.chars().collect();

    if from.len() != to.len() {
        return Err(format!(
            "transliterate strings have different lengths: {} vs {}",
            from.len(),
            to.len()
        ));
    }

    Ok((SedCommand::Transliterate { from, to }, remaining))
}

fn parse_text_arg(s: &str) -> (String, &str) {
    // a\text or a text - consume rest of command until ; or newline
    let s = s
        .strip_prefix('\\')
        .unwrap_or_else(|| s.trim_start());
    let end = s
        .find([';', '\n'])
        .unwrap_or(s.len());
    let text = s[..end].to_string();
    let remaining = if end < s.len() {
        &s[end + 1..]
    } else {
        ""
    };
    (text, remaining)
}

pub fn parse_script(script: &str, extended: bool) -> Result<Vec<CompiledCommand>, String> {
    let mut commands = Vec::new();
    let mut s = script.trim();

    while !s.is_empty() {
        s = s.trim_start();
        if s.is_empty() {
            break;
        }

        // Skip semicolons and newlines
        if s.starts_with(';') || s.starts_with('\n') {
            s = &s[1..];
            continue;
        }

        // Parse optional address
        let address;
        let first = s.chars().next().unwrap();

        if first.is_ascii_digit() || first == '$' || first == '/' {
            let (addr1, rest) = parse_address(s, extended)?;
            let rest = rest.trim_start();
            if let Some(comma_rest) = rest.strip_prefix(',') {
                let rest = comma_rest.trim_start();
                let (addr2, rest2) = parse_address(rest, extended)?;
                address = AddressRange::Range(addr1, addr2);
                s = rest2.trim_start();
            } else {
                address = AddressRange::Single(addr1);
                s = rest;
            }
        } else {
            address = AddressRange::None;
        }

        s = s.trim_start();
        if s.is_empty() {
            break;
        }

        let cmd_char = s.chars().next().unwrap();
        s = &s[cmd_char.len_utf8()..];

        let command = match cmd_char {
            's' => {
                let (cmd, rest) = parse_substitute(s, extended)?;
                s = rest;
                cmd
            }
            'd' => SedCommand::Delete,
            'p' => SedCommand::Print,
            'q' => SedCommand::Quit,
            'y' => {
                let (cmd, rest) = parse_transliterate(s)?;
                s = rest;
                cmd
            }
            'a' => {
                let (text, rest) = parse_text_arg(s);
                s = rest;
                SedCommand::AppendText(text)
            }
            'i' => {
                let (text, rest) = parse_text_arg(s);
                s = rest;
                SedCommand::InsertText(text)
            }
            'c' => {
                let (text, rest) = parse_text_arg(s);
                s = rest;
                SedCommand::ReplaceText(text)
            }
            ';' | '\n' => continue,
            _ => return Err(format!("unknown command: '{cmd_char}'")),
        };

        commands.push(CompiledCommand { address, command });
    }

    Ok(commands)
}

fn address_matches(addr: &Address, line_num: usize, line: &str, is_last: bool) -> bool {
    match addr {
        Address::Line(n) => line_num == *n,
        Address::Last => is_last,
        Address::Regex(re) => re.is_match(line),
    }
}

fn range_matches(
    range: &AddressRange,
    line_num: usize,
    line: &str,
    is_last: bool,
    in_range: &mut bool,
) -> bool {
    match range {
        AddressRange::None => true,
        AddressRange::Single(addr) => address_matches(addr, line_num, line, is_last),
        AddressRange::Range(start, end) => {
            if *in_range {
                if address_matches(end, line_num, line, is_last) {
                    *in_range = false;
                }
                true
            } else if address_matches(start, line_num, line, is_last) {
                *in_range = true;
                true
            } else {
                false
            }
        }
    }
}

fn apply_substitute(
    line: &str,
    pattern: &Regex,
    replacement: &str,
    global: bool,
) -> String {
    if global {
        pattern.replace_all(line, replacement).into_owned()
    } else {
        pattern.replace(line, replacement).into_owned()
    }
}

fn apply_transliterate(line: &str, from: &[char], to: &[char]) -> String {
    line.chars()
        .map(|c| {
            if let Some(pos) = from.iter().position(|&f| f == c) {
                to[pos]
            } else {
                c
            }
        })
        .collect()
}

pub fn sed_process(
    input: &mut dyn BufRead,
    commands: &[CompiledCommand],
    config: &SedConfig,
    writer: &mut dyn Write,
) -> io::Result<()> {
    // Read all lines to know which is last
    let lines: Vec<String> = input.lines().collect::<io::Result<Vec<_>>>()?;
    let total = lines.len();

    // Track range state per command
    let mut range_states: Vec<bool> = vec![false; commands.len()];

    for (idx, original_line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let is_last = line_num == total;
        let mut line = original_line.clone();
        let mut deleted = false;
        let mut extra_print = false;
        let mut append_text: Vec<String> = Vec::new();
        let mut insert_text: Vec<String> = Vec::new();
        let mut quit = false;
        for (cmd_idx, cmd) in commands.iter().enumerate() {
            if !range_matches(
                &cmd.address,
                line_num,
                &line,
                is_last,
                &mut range_states[cmd_idx],
            ) {
                continue;
            }

            match &cmd.command {
                SedCommand::Substitute {
                    pattern,
                    replacement,
                    global,
                    print,
                } => {
                    let new_line = apply_substitute(&line, pattern, replacement, *global);
                    let changed = new_line != line;
                    line = new_line;
                    if changed && *print {
                        extra_print = true;
                    }
                }
                SedCommand::Delete => {
                    deleted = true;
                    break;
                }
                SedCommand::Print => {
                    extra_print = true;
                }
                SedCommand::Quit => {
                    quit = true;
                }
                SedCommand::Transliterate { from, to } => {
                    line = apply_transliterate(&line, from, to);
                }
                SedCommand::AppendText(text) => {
                    append_text.push(text.clone());
                }
                SedCommand::InsertText(text) => {
                    insert_text.push(text.clone());
                }
                SedCommand::ReplaceText(text) => {
                    line = text.clone();
                }
            }
        }

        if deleted {
            continue;
        }

        for text in &insert_text {
            writeln!(writer, "{text}")?;
        }

        if extra_print {
            writeln!(writer, "{line}")?;
        }

        if !config.quiet {
            writeln!(writer, "{line}")?;
        }

        for text in &append_text {
            writeln!(writer, "{text}")?;
        }

        if quit {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn run_sed(input: &str, script: &str, quiet: bool) -> String {
        let commands = parse_script(script, false).unwrap();
        let config = SedConfig {
            quiet,
            ..Default::default()
        };
        let mut output = Vec::new();
        let mut reader = BufReader::new(input.as_bytes());
        sed_process(&mut reader, &commands, &config, &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_simple_substitute() {
        assert_eq!(run_sed("hello world\n", "s/hello/goodbye/", false), "goodbye world\n");
    }

    #[test]
    fn test_global_substitute() {
        assert_eq!(
            run_sed("aaa\n", "s/a/b/g", false),
            "bbb\n"
        );
    }

    #[test]
    fn test_delete() {
        assert_eq!(run_sed("line1\nline2\nline3\n", "2d", false), "line1\nline3\n");
    }

    #[test]
    fn test_print_with_quiet() {
        assert_eq!(run_sed("line1\nline2\n", "2p", true), "line2\n");
    }

    #[test]
    fn test_quit() {
        assert_eq!(run_sed("line1\nline2\nline3\n", "2q", false), "line1\nline2\n");
    }

    #[test]
    fn test_transliterate() {
        assert_eq!(run_sed("abc\n", "y/abc/xyz/", false), "xyz\n");
    }

    #[test]
    fn test_multiple_commands() {
        assert_eq!(
            run_sed("hello world\n", "s/hello/goodbye/; s/world/earth/", false),
            "goodbye earth\n"
        );
    }

    #[test]
    fn test_regex_address() {
        assert_eq!(
            run_sed("apple\nbanana\ncherry\n", "/banana/d", false),
            "apple\ncherry\n"
        );
    }

    #[test]
    fn test_line_range() {
        assert_eq!(
            run_sed("line1\nline2\nline3\nline4\n", "2,3d", false),
            "line1\nline4\n"
        );
    }

    #[test]
    fn test_last_line_address() {
        assert_eq!(
            run_sed("line1\nline2\nline3\n", "$d", false),
            "line1\nline2\n"
        );
    }

    #[test]
    fn test_substitute_with_print_flag() {
        assert_eq!(
            run_sed("hello\n", "s/hello/world/p", true),
            "world\n"
        );
    }

    #[test]
    fn test_append_text() {
        assert_eq!(
            run_sed("line1\nline2\n", "1a\\added", false),
            "line1\nadded\nline2\n"
        );
    }

    #[test]
    fn test_insert_text() {
        assert_eq!(
            run_sed("line1\nline2\n", "2i\\inserted", false),
            "line1\ninserted\nline2\n"
        );
    }
}
