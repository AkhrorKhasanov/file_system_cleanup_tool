use chrono::{DateTime, Utc};
use clap::Parser;
use std::io::{self, Write};
use std::time::SystemTime;
use walkdir::WalkDir;
#[derive(Parser, Debug)]
#[command(
    author = "Axror Hasanov",
    version = "1.0.0",
    about = "File System Cleanup Tool - Helper for cleaning old files",
    long_about = "This CLI tool scans a specified folder and identifies files that have not been modified for longer than the time you specify"
)]
struct Args {
    #[arg(short, long)]
    folder: Option<String>,
    #[arg(short, long, default_value_t = 30)]
    days: u32,
}
fn main() {
    let args = Args::parse();

    let folder_path = match args.folder {
        Some(f) => f,
        None => {
            print!("Papka yo'lini kiriting: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Xatolik");
            input.trim().to_string()
        }
    };

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

    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
