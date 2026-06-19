use std::time::SystemTime;
use clap::Parser;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use std::io::{self, BufRead};
#[derive(Parser, Debug)]
#[command(
    author = "Axror Hasanov",
    version = "1.0.0",
    about = "File System Cleanup Tool - Helper for cleaning old files",
    long_about = "This CLI tool scans a specified folder and identifies files that have not been modified for longer than the time you specify"
)]
struct Args {
    #[arg(short, long)]
    folder: String,
    #[arg(short, long, default_value_t = 30)]
    days: u32,
}
fn main() {
    let args = Args::parse();
    let folder_path = args.folder;
    let days_limit = args.days;
    let now = SystemTime::now();
    let mut old_files_count = 0;
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                let duration = now.duration_since(modified).unwrap_or_default();
                let days_old = (duration.as_secs() / 86400) as u32;

                let datetime: DateTime<Utc> = modified.into();
                if days_old >= days_limit {
                    old_files_count += 1;
                    println!("Path: {:?}", path);
                    println!("Last modified: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
                    println!("{} day(s) ago", days_old);
                    println!("---");
                }

            }
        }
    }
    println!("Old files count: {}", old_files_count);

    println!("Click *Enter* button to exit...");
    let mut stdin = io::stdin().lock();
    let _ = stdin.fill_buf();
}
