use crate::cli::NprocConfig;

pub fn nproc(config: &NprocConfig) -> usize {
    let count = if config.all {
        // --all: report all installed processors
        // We use available_parallelism as the best cross-platform approximation
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    } else {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    };

    count.saturating_sub(config.ignore).max(1)
}
