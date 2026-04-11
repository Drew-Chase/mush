use std::io::{self, BufWriter, Write};

use crate::cli::SeqConfig;

pub fn run_seq(config: &SeqConfig) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let width = if config.equal_width {
        equal_width(config.first, config.increment, config.last)
    } else {
        0
    };

    let mut current = config.first;
    let mut first_output = true;

    let going_up = config.increment > 0.0;

    loop {
        if going_up && current > config.last + 1e-10 {
            break;
        }
        if !going_up && current < config.last - 1e-10 {
            break;
        }
        if config.increment == 0.0 {
            break;
        }

        if !first_output {
            write!(out, "{}", config.separator)?;
        }

        if let Some(ref fmt) = config.format {
            let formatted = apply_format(fmt, current);
            write!(out, "{formatted}")?;
        } else if width > 0 {
            let s = format_number(current);
            if current >= 0.0 {
                write!(out, "{s:>0width$}")?;
            } else {
                let abs_s = format_number(current.abs());
                let padded = format!("{abs_s:>0w$}", w = width - 1);
                write!(out, "-{padded}")?;
            }
        } else {
            let s = format_number(current);
            write!(out, "{s}")?;
        }

        first_output = false;
        current += config.increment;
    }

    if !first_output {
        writeln!(out)?;
    }

    out.flush()
}

fn format_number(n: f64) -> String {
    if n == n.trunc() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

fn apply_format(fmt: &str, value: f64) -> String {
    // Support basic %g, %f, %e formats
    if let Some(pos) = fmt.find('%') {
        let spec = &fmt[pos..];
        // Find the conversion character
        for (i, c) in spec.char_indices() {
            if i > 0 && matches!(c, 'f' | 'e' | 'g' | 'F' | 'E' | 'G') {
                let before = &fmt[..pos];
                let after = &fmt[pos + i + 1..];
                let spec_str = &fmt[pos..pos + i + 1];
                let formatted = c_style_format(spec_str, value);
                return format!("{before}{formatted}{after}");
            }
        }
    }
    format_number(value)
}

fn c_style_format(spec: &str, value: f64) -> String {
    // Parse %[width][.precision]type
    let inner = &spec[1..];
    let conv = inner.chars().last().unwrap();
    let params = &inner[..inner.len() - 1];

    let (width, precision) = if let Some(dot_pos) = params.find('.') {
        let w: usize = if dot_pos > 0 {
            params[..dot_pos].parse().unwrap_or(0)
        } else {
            0
        };
        let p: usize = params[dot_pos + 1..].parse().unwrap_or(6);
        (w, Some(p))
    } else {
        let w: usize = if params.is_empty() {
            0
        } else {
            params.parse().unwrap_or(0)
        };
        (w, None)
    };

    let prec = precision.unwrap_or(6);

    let s = match conv {
        'f' | 'F' => format!("{value:.prec$}"),
        'e' => format!("{value:.prec$e}"),
        'E' => format!("{value:.prec$E}"),
        'g' | 'G' => {
            if let Some(p) = precision {
                format!("{value:.p$}")
            } else {
                format!("{value}")
            }
        }
        _ => format!("{value}"),
    };

    if width > 0 {
        format!("{s:>width$}")
    } else {
        s
    }
}

fn equal_width(first: f64, increment: f64, last: f64) -> usize {
    let first_s = format_number(first);
    let last_s = format_number(last);

    // Also check the final value in the sequence
    let mut current = first;
    let mut max_len = first_s.len().max(last_s.len());

    while (increment > 0.0 && current <= last + 1e-10)
        || (increment < 0.0 && current >= last - 1e-10)
    {
        let s = format_number(current);
        max_len = max_len.max(s.len());
        current += increment;
    }

    max_len
}
