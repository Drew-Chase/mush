use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let total_steps: u32 = 100;
    let step_duration = Duration::from_millis(600);
    let bar_width: u32 = 40;
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for step in 0..=total_steps {
        let filled = (step * bar_width) / total_steps;
        let empty = bar_width - filled;
        let pct = step;
        let elapsed = (step as u64 * 600) / 1000;

        let _ = write!(
            out,
            "\r[{}{}] {}% ({}s / 60s)",
            "#".repeat(filled as usize),
            " ".repeat(empty as usize),
            pct,
            elapsed,
        );
        let _ = out.flush();

        if step < total_steps {
            thread::sleep(step_duration);
        }
    }

    let _ = writeln!(out);
    let _ = out.flush();
}
