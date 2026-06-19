# file_system_cleanup_tool
A Rust-based CLI tool that scans directories to identify stale files (e.g., unmodified for >30 days). Utilizing std::fs for metadata analysis and clap for configuration, it provides a safe, efficient way to manage disk space by automating the identification, deletion, or archival of "digital clutter."
