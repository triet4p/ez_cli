use std::fs;
use std::path::{ Path, PathBuf };
use std::io;
use glob::Pattern;

// UI Config
// Mid branch
const EDGE: &str = "├── "; 
// Vertical line
const LINE: &str = "│   ";
// Last branch
const CORNER: &str = "└── ";
// Blank when end branch
const BLANK: &str= "    ";

#[derive(Debug)]
struct Node {
    name: String,
    path: PathBuf,
    is_dir: bool,
    children: Vec<Node>, 
}

struct TreeFilter {
    includes: Vec<Pattern>,
    excludes: Vec<Pattern>,
}

impl TreeFilter {
    fn is_excluded(&self, name: &str) -> bool {
        self.excludes.iter().any(|p| p.matches(name))
    }

    fn is_included(&self, name: &str) -> bool {
        if self.includes.is_empty() {
            return true; 
        }
        self.includes.iter().any(|p| p.matches(name))
    }
}

fn build_tree(path: &Path, filter: &TreeFilter) -> Option<Node> {
    let name = path.file_name()
        .unwrap_or(path.as_os_str()) 
        .to_string_lossy()
        .to_string();

    let is_dir = path.is_dir();

    if filter.is_excluded(&name) {
        return None;
    }

    if !is_dir {
        if filter.is_included(&name) {
            return Some(Node {
                name,
                path: path.to_path_buf(),
                is_dir: false,
                children: Vec::new(),
            });
        } else {
            return None; 
        }
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                eprintln!("⚠️  Warning: Permission denied accessing {:?}", path);
            }
            return None;
        }
    };
    
    let mut children: Vec<Node> = Vec::new();

    for entry in entries.flatten() { 
        if let Some(child_node) = build_tree(&entry.path(), filter) {
            children.push(child_node);
        }
    }

    if children.is_empty() && is_dir {
        return None;
    }

    // Sắp xếp con cho đẹp
    children.sort_by(|a, b| a.name.cmp(&b.name));

    Some(Node {
        name,
        path: path.to_path_buf(),
        is_dir: is_dir,
        children,
    })
}

fn print_tree(nodes: &[Node], prefix: &str) {
    let count = nodes.len();
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { CORNER } else { EDGE };

        println!("{}{}{}", prefix, connector, node.name);

        if node.is_dir {
            let child_prefix = if is_last { BLANK } else { LINE };
            let new_prefix = format!("{}{}", prefix, child_prefix);
            print_tree(&node.children, &new_prefix);
        }
    }
}

/// Draw a tree
pub fn draw(path_str: &str, includes: &[String], excludes: &[String]) -> io::Result<()> {
    let root_path = Path::new(path_str);
    if !root_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "❌ Path not found"));
    }

    let compile_patterns = |raw_lst: &[String]| -> Vec<Pattern> {
        raw_lst.iter()
            .filter_map(|s| Pattern::new(s).ok())
            .collect()
    };

    let filter = TreeFilter {
        includes: compile_patterns(includes),
        excludes: compile_patterns(excludes),
    };

    println!("{}", root_path.file_name().unwrap_or(root_path.as_os_str()).to_string_lossy());

    println!("Building tree for: {}", root_path.display());

    if let Some(root_node) = build_tree(root_path, &filter) {
        println!("{}", root_node.name); 
        print_tree(&root_node.children, ""); 
    } else {
        println!("(No files found matching patterns)");
    }

    Ok(())
}