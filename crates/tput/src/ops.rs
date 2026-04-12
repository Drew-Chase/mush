use std::io::Write;

use crate::cli::TputCapability;

pub fn execute_capability(cap: &TputCapability, output: &mut dyn Write) -> std::io::Result<()> {
    match cap {
        TputCapability::Cols => {
            let (cols, _) = terminal_size();
            writeln!(output, "{cols}")?;
        }
        TputCapability::Lines => {
            let (_, lines) = terminal_size();
            writeln!(output, "{lines}")?;
        }
        TputCapability::Colors => {
            writeln!(output, "256")?;
        }
        TputCapability::Bold => {
            write!(output, "\x1b[1m")?;
        }
        TputCapability::Sgr0 => {
            write!(output, "\x1b[0m")?;
        }
        TputCapability::Setaf(color) => {
            let code = ansi_color_code(*color);
            write!(output, "\x1b[{code}m")?;
        }
        TputCapability::Clear => {
            write!(output, "\x1b[2J\x1b[H")?;
        }
        TputCapability::Cup(row, col) => {
            // ANSI escape is 1-based, tput is 0-based
            write!(output, "\x1b[{};{}H", row + 1, col + 1)?;
        }
    }
    output.flush()
}

fn terminal_size() -> (u16, u16) {
    crossterm::terminal::size().unwrap_or((80, 24))
}

fn ansi_color_code(color: u8) -> String {
    match color {
        0 => "30".to_string(),   // black
        1 => "31".to_string(),   // red
        2 => "32".to_string(),   // green
        3 => "33".to_string(),   // yellow
        4 => "34".to_string(),   // blue
        5 => "35".to_string(),   // magenta
        6 => "36".to_string(),   // cyan
        7 => "37".to_string(),   // white
        8..=15 => format!("{}", 90 + (color - 8)),  // bright colors
        _ => format!("38;5;{color}"),  // 256-color
    }
}
