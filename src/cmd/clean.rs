use std::fs;
use std::path::Path;
use std::io;
use glob::Pattern;

fn clean_recursive(dir: &Path, pattern: &Pattern) -> io::Result<()> {
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

        if path.is_file() {
            if pattern.matches(&name) {
                match fs::remove_file(&path) {
                    Ok(_) => println!("Deleted file: {:?}", path),
                    Err(e) => eprintln!("Failed to delete file: {:?}, {}", path, e),
                }
            }
        } else if path.is_dir() {
            if pattern.matches(&name) {
                match fs::remove_dir_all(&path) {
                    Ok(_) => println!("Deleted folder: {:?}", path),
                    Err(e) => eprintln!("Failed to delete folder: {:?}, {}", path, e),
                }
            } else {
                clean_recursive(&path, pattern)?;
            }
        }
    }

    Ok(())
}   

pub fn run(pattern_str: &str, root_dir: &str) -> io::Result<()> {
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

    clean_recursive(root_path, &pattern)
}