use std::error::Error;

pub mod cmd;

#[derive(Debug, Clone, Copy)]
pub enum SplitStrategy {
    Move,       // Default: rename/move file (destructive)
    Copy,       // Flag: --copy
    HardLink,   // Flag: --hard-link
}

// Define supported command
#[derive(Debug)]
pub enum Command {
    Tree { path: String, include: Vec<String>, exclude: Vec<String> },
    Find { pattern: String, root_dir: String },
    Clean { pattern: String, root_dir: String },
    Split { src: String, dst: String, ratio: f32, strategy: SplitStrategy },
    Env { key: String, val: String, group: String },
    Help,
}

// Struct contain parsed cmd
pub struct Config {
    pub cmd: Command,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Ok(Config { cmd: Command::Help });
        }

        // args[0] is exe file
        // args[1] is command (tree, find, ...)
        let command_str = &args[1];

        let cmd = match command_str.as_str() {
            "tree" => {
                // Default '.' if user hasn't input
                let path = args.get(2)
                    .cloned().unwrap_or_else(|| String::from("."));

                let include_patterns = find_flag_value(args, "--include");
                let include = convert_list_patterns(&include_patterns, ",");
                println!("{:?}", &include);

                let exclude_patterns = find_flag_value(args, "--exclude");
                let exclude = convert_list_patterns(&exclude_patterns, ",");

                Command::Tree { path, include, exclude }
            },
            "find" => {
                let pattern = args.get(2)
                    .cloned().ok_or("❌Missing pattern (ex *.pyc)");

                let root_dir = args.get(3)
                    .cloned().unwrap_or_else(|| String::from("."));
                Command::Find { pattern: pattern?, root_dir: root_dir }
            },
            "clean" => {
                let pattern = args.get(2)
                    .cloned().ok_or("❌Missing pattern (ex *.pyc)");

                let root_dir = args.get(3)
                    .cloned().unwrap_or_else(|| String::from("."));
                Command::Clean { pattern: pattern?, root_dir }
            },
            "split" => {
                let src = args.get(2)
                    .cloned().ok_or("❌Missing source folder");
                let dst = args.get(3)
                    .cloned().unwrap_or_else(|| String::from("."));
                // Parse ratio
                let ratio = args.get(3)
                    .map(|s| s.parse().unwrap_or(0.8))
                    .unwrap_or(0.8);

                let is_hard_link_strategy = is_flag_exist(args, "--hard-link");
                let is_copy_strategy = is_flag_exist(args, "--copy");
                
                let mut strategy = SplitStrategy::Move;
                if is_hard_link_strategy && is_copy_strategy {
                    return Err("❌ You must select each `--hard-link` or `--copy` flag, not both.");
                } else if is_hard_link_strategy {
                    strategy = SplitStrategy::HardLink;
                } else if is_copy_strategy {
                    strategy = SplitStrategy::Copy;
                } else {
                    strategy = SplitStrategy::Move;
                }

                Command::Split { src: src?, dst: dst, ratio: ratio, strategy }
            },
            "env" => {
                let key = args.get(2)
                    .cloned().ok_or("❌Missing key");
                let val = args.get(3)
                    .cloned().ok_or("❌Missing value");
                
                let group = find_flag_value(args, "--group") 
                    .unwrap_or_else(|| String::from("default"));

                Command::Env { key: key?, val: val?, group: group }
            },
            "help" | "--help" | "-h" => Command::Help,
            _ => return Err("❌ Unknown command! Type 'ez_cli help' for instructions."),
        };

        Ok(Config { cmd })
    }
}

// Helper func: Check flag in args
fn is_flag_exist(args: &[String], flag: &str) -> bool {
    args.iter().any(|s| s == flag)
}

// Helper func: Convert list string
fn convert_list_patterns(pattern_opt: &Option<String>, delimiter: &str) -> Vec<String> {
    match pattern_opt {
        Some(s) => s.split(delimiter)
                    .map(|p| p.trim().to_string())
                    .collect(),
        None => Vec::new(), // Trả về vector rỗng nếu không có
    }
}

// Helper func: Find value after a flag
fn find_flag_value(args: &[String], flag: &str) -> Option<String> {
    if let Some(index) = args.iter().position(|x| x == flag) {
        args.get(index + 1).cloned()
    } else {
        None
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.cmd {
        Command::Tree { path, include, exclude } => {
            println!("🌳 Drawing directory tree at: '{}'...", path);
            cmd::tree::draw(&path, &include, &exclude)?;
        },
        Command::Find { pattern, root_dir } => {
            println!("🔍 Searching for files matching: '{}'", pattern);
            cmd::find::run(&pattern, &root_dir)?;
        },
        Command::Clean { pattern, root_dir } => {
            println!("🗑️ Cleaning up files matching: '{}'", pattern);
            cmd::clean::run(&pattern, &root_dir)?;
        },
        Command::Split { src, dst, ratio, strategy } => {
            println!("✂️ Splitting dataset '{}' (Train: {}%)", src, ratio * 100.0);
            cmd::split::run(&src, &dst, ratio, strategy)?;
        },
        Command::Env { key, val, group } => {
            println!("📝 Adding Env: {}={} (Group: 🏷️ {})", key, val, group);
        },
        Command::Help => {
            print_help();
        }
    }
    Ok(())
}

fn print_help() {
    println!("🚀 EZ_CLI - The AI Engineer's Swiss Army Knife");
    println!("----------------------------------------------");
    println!("Usage:");
    println!("  tree [path] --include [includes] --exclude [excludes]  : 🌳 Show directory tree");
    println!("  find <pattern> [root_dir]                              : 🔍 Find files");
    println!("  clean <pattern> [root_dir]                             : 🗑️ Clean junk files");
    println!("  split <src> [ratio] [flags]                            : ✂️ Split Train/Val");
    println!("      Flags: --copy (Copy files), --hard-link (Hard Link)");
    println!("      Default: Move files (Rename)");
    println!("  env <key> <val> --group <g>                            : 📝 Manage environment variables");
}