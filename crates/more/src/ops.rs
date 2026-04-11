use std::io::{self, Write};

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, ClearType};
use crossterm::{execute, queue};

use crate::cli::MoreConfig;

const HELP_SCREEN: &str = "\
Interactive commands:
  SPACE   next page
  ENTER   next line
  q       quit
  h       this help screen

Press any key to continue...";

fn squeeze_blank_lines(lines: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(lines.len());
    let mut prev_blank = false;
    for line in lines {
        let is_blank = line.trim().is_empty();
        if is_blank && prev_blank {
            continue;
        }
        prev_blank = is_blank;
        result.push(line.clone());
    }
    result
}

pub fn more(lines: &[String], config: &MoreConfig) -> io::Result<()> {
    let lines = if config.squeeze {
        squeeze_blank_lines(lines)
    } else {
        lines.to_vec()
    };

    let (_, term_height) = terminal::size()?;
    let page_size = config
        .lines_per_screen
        .unwrap_or(term_height.saturating_sub(1) as usize)
        .max(1);

    let start = config
        .start_line
        .map(|n| n.saturating_sub(1))
        .unwrap_or(0)
        .min(lines.len());

    let mut offset = start;
    let total = lines.len();

    if total == 0 {
        return Ok(());
    }

    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    let result = run_pager(&mut stdout, &lines, &mut offset, page_size, total);
    terminal::disable_raw_mode()?;

    // Print a newline after exiting so the shell prompt starts on a fresh line
    println!();

    result
}

fn run_pager(
    stdout: &mut io::Stdout,
    lines: &[String],
    offset: &mut usize,
    page_size: usize,
    total: usize,
) -> io::Result<()> {
    // Display the first screenful
    let end = (*offset + page_size).min(total);
    for line in &lines[*offset..end] {
        queue!(stdout, crossterm::style::Print(line), crossterm::style::Print("\r\n"))?;
    }
    *offset = end;

    if *offset >= total {
        return Ok(());
    }

    show_status(stdout, *offset, total)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => {
                    clear_status(stdout)?;
                    return Ok(());
                }
                KeyCode::Char(' ') => {
                    clear_status(stdout)?;
                    let end = (*offset + page_size).min(total);
                    for line in &lines[*offset..end] {
                        queue!(
                            stdout,
                            crossterm::style::Print(line),
                            crossterm::style::Print("\r\n")
                        )?;
                    }
                    *offset = end;
                    if *offset >= total {
                        return Ok(());
                    }
                    show_status(stdout, *offset, total)?;
                }
                KeyCode::Enter => {
                    clear_status(stdout)?;
                    if *offset < total {
                        queue!(
                            stdout,
                            crossterm::style::Print(&lines[*offset]),
                            crossterm::style::Print("\r\n")
                        )?;
                        *offset += 1;
                    }
                    if *offset >= total {
                        return Ok(());
                    }
                    show_status(stdout, *offset, total)?;
                }
                KeyCode::Char('h') => {
                    clear_status(stdout)?;
                    queue!(
                        stdout,
                        crossterm::style::Print(HELP_SCREEN.replace('\n', "\r\n"))
                    )?;
                    stdout.flush()?;
                    // Wait for any key
                    loop {
                        if let Event::Key(_) = event::read()? {
                            break;
                        }
                    }
                    // Redraw status
                    queue!(stdout, crossterm::style::Print("\r\n"))?;
                    show_status(stdout, *offset, total)?;
                }
                _ => {}
            }
        }
    }
}

fn show_status(stdout: &mut io::Stdout, offset: usize, total: usize) -> io::Result<()> {
    let pct = if total == 0 {
        100
    } else {
        (offset * 100) / total
    };
    queue!(
        stdout,
        crossterm::style::SetAttribute(crossterm::style::Attribute::Reverse),
        crossterm::style::Print(format!("--More--({pct}%)")),
        crossterm::style::SetAttribute(crossterm::style::Attribute::Reset)
    )?;
    stdout.flush()
}

fn clear_status(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        crossterm::style::Print("\r"),
        terminal::Clear(ClearType::CurrentLine)
    )
}
