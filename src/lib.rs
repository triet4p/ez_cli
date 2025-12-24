use std::error::Error;

pub mod cmd;

// Define supported command
#[derive(Debug)]
pub enum Command {
    Tree { path: String },
    Find { pattern: String },
    Clean { pattern: String },
    Split { src: String, ratio: f32 },
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
                Command::Tree { path }
            },
            "find" => {
                let pattern = args.get(2)
                    .cloned().ok_or("❌Missing pattern (ex *.pyc)");
                Command::Find { pattern: pattern? }
            },
            "clean" => {
                let pattern = args.get(2)
                    .cloned().ok_or("❌Missing pattern (ex *.pyc)");
                Command::Clean { pattern: pattern? }
            },
            "split" => {
                let src = args.get(2)
                    .cloned().ok_or("❌Missing source folder");
                // Parse ratio
                let ratio = args.get(3)
                    .map(|s| s.parse().unwrap_or(0.8))
                    .unwrap_or(0.8);

                Command::Split { src: src?, ratio: ratio }
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
        Command::Tree { path } => {
            println!("🌳 Drawing directory tree at: '{}'...", path);
            cmd::tree::draw(&path)?;
        },
        Command::Find { pattern } => {
            println!("🔍 Searching for files matching: '{}'", pattern);
        },
        Command::Clean { pattern } => {
            println!("🗑️ Cleaning up files matching: '{}'", pattern);
        },
        Command::Split { src, ratio } => {
            println!("✂️ Splitting dataset '{}' (Train: {}%)", src, ratio * 100.0);
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
    println!("  ez_cli tree [path]                : 🌳 Show directory tree");
    println!("  ez_cli find <pattern>             : 🔍 Find files");
    println!("  ez_cli clean <pattern>            : 🗑️ Clean junk files");
    println!("  ez_cli split <src> [ratio]        : ✂️ Split Train/Val (default ratio 0.8)");
    println!("  ez_cli env <key> <val> --group <g>: 📝 Manage environment variables");
}