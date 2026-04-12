use std::io::{BufRead, Write};

/// Simple XorShift64 PRNG seeded from system time.
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn from_time() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        // Ensure non-zero seed
        Self { state: seed | 1 }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self { state: seed | 1 }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Returns a value in [0, bound)
    pub fn next_bounded(&mut self, bound: u64) -> u64 {
        if bound == 0 { return 0; }
        self.next_u64() % bound
    }
}

/// Fisher-Yates shuffle
pub fn shuffle(items: &mut [String], rng: &mut XorShift64) {
    let n = items.len();
    for i in (1..n).rev() {
        let j = rng.next_bounded(i as u64 + 1) as usize;
        items.swap(i, j);
    }
}

pub fn shuf_lines(
    lines: &[String],
    head_count: Option<usize>,
    repeat: bool,
    output: &mut dyn Write,
    rng: &mut XorShift64,
) -> std::io::Result<()> {
    if lines.is_empty() {
        return Ok(());
    }

    if repeat {
        let count = head_count.unwrap_or(lines.len());
        for _ in 0..count {
            let idx = rng.next_bounded(lines.len() as u64) as usize;
            writeln!(output, "{}", lines[idx])?;
        }
    } else {
        let mut items: Vec<String> = lines.to_vec();
        shuffle(&mut items, rng);
        let count = head_count.unwrap_or(items.len()).min(items.len());
        for item in &items[..count] {
            writeln!(output, "{item}")?;
        }
    }
    output.flush()
}

pub fn read_lines(input: &mut dyn BufRead) -> std::io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        if input.read_line(&mut line)? == 0 {
            break;
        }
        let trimmed = line.strip_suffix('\n').unwrap_or(&line);
        let trimmed = trimmed.strip_suffix('\r').unwrap_or(trimmed);
        lines.push(trimmed.to_string());
    }
    Ok(lines)
}

pub fn range_to_lines(lo: u64, hi: u64) -> Vec<String> {
    (lo..=hi).map(|n| n.to_string()).collect()
}
