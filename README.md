# 🧹 File System Cleanup Tool

A lightweight and efficient Rust-based CLI tool designed to identify and clean up "digital clutter" on your system.

## 🚀 Description
This tool recursively scans a specified directory to identify files that haven't been modified for a set period (default is 30 days). It helps you maintain disk space and organize your file system by pinpointing stale files.

## 🛠 Tech Stack
* **Rust**: The core programming language.
* **Clap**: For robust command-line argument parsing.
* **WalkDir**: For efficient recursive directory traversal.
* **Chrono**: For accurate time calculations and date formatting.

## 📦 Installation
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository:
   ```bash
   git clone https://github.com/AkhrorKhasanov/file_system_cleanup_tool.git
   cd file_system_cleanup_tool

1. Verify the installation by checking the help menu:

```bash
cargo run -- --help
```

⌨️ Usage
You can run the tool using the following command structure:

General Format:

```bash
cargo run -- --folder "YOUR_FOLDER_PATH" --days NUMBER_OF_DAYS
```
Examples:

1. Run with the default 30-day limit:

```bash
cargo run -- --folder "C:\Users\Documents\Downloads"
```

2. Find files not modified in the last 60 days:

```bash
cargo run -- -f "C:\Users\Documents\Downloads" -d 60
```

📋 Arguments
-f, --folder <FOLDER> : The path to the directory you want to scan (Required).

-d, --days <DAYS> : The age threshold for files (in days). Default is 30.

-h, --help : Display the help menu.

🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page or submit a Pull Request.

This project was created to explore Rust systems programming and improve system organization.
