use std::fs;
use std::path::Path;
use std::io;
use glob::Pattern;

/// Entry point for the find command
/// - pattern_str: The glob pattern to search for (e.g., "*.rs")
/// - root_dir: Where to start searching (default is ".")
pub fn run(pattern_str: &str, root_dir: &str) -> io::Result<()> {
    // Compile the pattern once for performance
    let pattern = match Pattern::new(pattern_str) {
        Ok(p) => p,
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid pattern: {}", e)));
        }
    };

    let root_path = Path::new(root_dir);
    if !root_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"));
    }

    // Start recursive search
    search_recursive(root_path, &pattern)
}

/// Recursive function to traverse directories
fn search_recursive(dir: &Path, pattern: &Pattern) -> io::Result<()> {
    // Read directory entries
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            // Handle permission errors gracefully (skip protected folders)
            if e.kind() == io::ErrorKind::PermissionDenied {
                eprintln!("⚠️  Permission denied: {:?}", dir);
                return Ok(());
            }
            return Err(e);
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // 1. If it's a FILE: Check if it matches the pattern
        if path.is_file() {
            if pattern.matches(&name) {
                // Found a match! Print the full relative path
                println!("{}", path.display());
            }
        }
        // 2. If it's a DIRECTORY: Recursively search inside
        else if path.is_dir() {
            // Optimization: Skip hidden folders like .git to save time
            if !name.starts_with('.') {
                search_recursive(&path, pattern)?;
            }
        }
    }

    Ok(())
}