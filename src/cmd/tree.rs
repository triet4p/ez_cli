use std::fs;
use std::path::Path;
use std::io;

// UI Config
// Mid branch
const EDGE: &str = "├── "; 
// Vertical line
const LINE: &str = "│   ";
// Last branch
const CORNER: &str = "└── ";
// Blank when end branch
const BLANK: &str= "    ";

/// Recursive func
/// - dir: Path to current directory.
/// - prefix: Prefix chain to align (ex: "│   ├── ")
fn visit_dirs(dir: &Path, prefix: &str) -> io::Result<()> {
    // Read dir content
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                eprintln!("{}⚠️  Warning: Permission denied accessing {:?}", prefix, dir);
                return Ok(());
            }
            return Err(e);
        }
    };

    // Filter and add to vector
    let mut entries_vec: Vec<_> = entries.filter_map(|e| e.ok()).collect();

    entries_vec.sort_by_key(|e| e.file_name());

    let count = entries_vec.len();
    for (i, entry) in entries_vec.iter().enumerate() {
        let is_last = i == count - 1;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        let connector = if is_last { CORNER } else { EDGE };

        println!("{}{}{}", prefix, connector, name_str);

        // Recursive
        if entry.path().is_dir() {
            // Prepare prefix
            let child_prefix = if is_last { BLANK } else { LINE };
            let new_prefix = format!("{}{}", prefix, child_prefix);

            visit_dirs(&entry.path(), &new_prefix)?;
        }
    }
    Ok(())
}

/// Draw a tree
pub fn draw(path_str: &str) -> io::Result<()> {
    let root_path = Path::new(path_str);
    if !root_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "❌ Path not found"));
    }

    println!("{}", root_path.file_name().unwrap_or(root_path.as_os_str()).to_string_lossy());

    visit_dirs(root_path, "")
}