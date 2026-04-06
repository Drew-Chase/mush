use std::io::{self, BufWriter};

use ls::cli::ResolvedConfig;
use ls::color::ColorScheme;
use ls::{format, read, sort};

fn main() {
    let config = ResolvedConfig {
        human_readable: true,
        ..Default::default()
    };
    let colors = ColorScheme::new(&config);

    let mut entries = read::read_entries(".".as_ref(), &config).expect("failed to read directory");
    sort::sort_entries(&mut entries, &config);

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    format::write_output(&entries, &config, &colors, &mut out).expect("failed to write output");
}
