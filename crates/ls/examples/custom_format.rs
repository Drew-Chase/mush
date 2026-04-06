use ls::cli::ResolvedConfig;
use ls::entry::FileType;
use ls::{read, sort};

fn main() {
    let config = ResolvedConfig::default();
    let mut entries = read::read_entries(".".as_ref(), &config).expect("failed to read directory");
    sort::sort_entries(&mut entries, &config);

    println!("{:<30} {:>10} {}", "NAME", "SIZE", "TYPE");
    println!("{}", "-".repeat(50));

    for entry in &entries {
        let type_label = match entry.file_type {
            FileType::Directory => "dir",
            FileType::Symlink => "link",
            FileType::Regular => "file",
            _ => "other",
        };
        println!("{:<30} {:>10} {}", entry.name, entry.size, type_label);
    }
}
