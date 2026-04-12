use std::io::{BufRead, Read, Write};

use crate::cli::XxdConfig;

pub fn xxd_hex_dump(input: &mut dyn Read, output: &mut dyn Write, config: &XxdConfig) -> std::io::Result<()> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;

    let start = config.seek.min(buf.len());
    let end = if let Some(len) = config.length {
        (start + len).min(buf.len())
    } else {
        buf.len()
    };
    let data = &buf[start..end];

    if config.plain {
        return dump_plain(data, output, config);
    }
    if config.include {
        return dump_include(data, output, config);
    }
    if config.bits {
        return dump_bits(data, output, config);
    }

    dump_normal(data, output, config, start)
}

fn dump_normal(data: &[u8], output: &mut dyn Write, config: &XxdConfig, base_offset: usize) -> std::io::Result<()> {
    let fmt = if config.upper { 'X' } else { 'x' };
    for (chunk_idx, chunk) in data.chunks(config.cols).enumerate() {
        let offset = base_offset + chunk_idx * config.cols;
        write!(output, "{offset:08x}: ")?;

        let mut hex_parts = Vec::new();
        for group in chunk.chunks(config.group_size.max(1)) {
            let group_hex: String = group.iter().map(|b| {
                if fmt == 'X' { format!("{b:02X}") } else { format!("{b:02x}") }
            }).collect();
            hex_parts.push(group_hex);
        }
        let hex_str = hex_parts.join(" ");

        let groups_per_line = config.cols.div_ceil(config.group_size.max(1));
        let max_hex_width = groups_per_line * (config.group_size.max(1) * 2) + groups_per_line - 1;
        write!(output, "{hex_str:<width$}  ", width = max_hex_width)?;

        for &b in chunk {
            let c = if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' };
            write!(output, "{c}")?;
        }
        writeln!(output)?;
    }
    output.flush()
}

fn dump_plain(data: &[u8], output: &mut dyn Write, config: &XxdConfig) -> std::io::Result<()> {
    let fmt = if config.upper { 'X' } else { 'x' };
    for (i, b) in data.iter().enumerate() {
        if fmt == 'X' {
            write!(output, "{b:02X}")?;
        } else {
            write!(output, "{b:02x}")?;
        }
        if (i + 1) % config.cols == 0 {
            writeln!(output)?;
        }
    }
    if !data.len().is_multiple_of(config.cols) {
        writeln!(output)?;
    }
    output.flush()
}

fn dump_include(data: &[u8], output: &mut dyn Write, config: &XxdConfig) -> std::io::Result<()> {
    let name = config.file.as_deref().unwrap_or("stdin")
        .replace(['.', '/', '\\', '-'], "_");
    writeln!(output, "unsigned char {name}[] = {{")?;
    for (i, chunk) in data.chunks(12).enumerate() {
        write!(output, " ")?;
        for (j, b) in chunk.iter().enumerate() {
            write!(output, " 0x{b:02x}")?;
            if i * 12 + j + 1 < data.len() {
                write!(output, ",")?;
            }
        }
        writeln!(output)?;
    }
    writeln!(output, "}};")?;
    writeln!(output, "unsigned int {name}_len = {};", data.len())?;
    output.flush()
}

fn dump_bits(data: &[u8], output: &mut dyn Write, config: &XxdConfig) -> std::io::Result<()> {
    let cols = config.cols.min(6);
    for (chunk_idx, chunk) in data.chunks(cols).enumerate() {
        let offset = chunk_idx * cols;
        write!(output, "{offset:08x}: ")?;
        for (j, b) in chunk.iter().enumerate() {
            if j > 0 { write!(output, " ")?; }
            write!(output, "{b:08b}")?;
        }
        let padding = cols - chunk.len();
        for _ in 0..padding {
            write!(output, "         ")?;
        }
        write!(output, "  ")?;
        for &b in chunk {
            let c = if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' };
            write!(output, "{c}")?;
        }
        writeln!(output)?;
    }
    output.flush()
}

pub fn xxd_reverse(input: &mut dyn BufRead, output: &mut dyn Write) -> std::io::Result<()> {
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

        // Try plain hex (no offset prefix)
        let hex_part = if let Some((_offset, rest)) = trimmed.split_once(": ") {
            // Normal xxd format: strip offset and ascii
            let hex_only = rest.split("  ").next().unwrap_or(rest);
            hex_only.to_string()
        } else {
            trimmed.to_string()
        };

        let hex_chars: String = hex_part.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        for pair in hex_chars.as_bytes().chunks(2) {
            if pair.len() == 2 {
                let s = std::str::from_utf8(pair).unwrap();
                if let Ok(b) = u8::from_str_radix(s, 16) {
                    output.write_all(&[b])?;
                }
            }
        }
    }
    output.flush()
}
