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
