use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use rand::seq::SliceRandom;
use rand::rng;
use crate::SplitStrategy; 

fn collect_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

fn process_files(files: &[PathBuf], dest_dir: &Path, label: &str, strategy: SplitStrategy) -> io::Result<()> {
    for (i, file_path) in files.iter().enumerate() {
        if let Some(file_name) = file_path.file_name() {
            let dest_path = dest_dir.join(file_name);
            match strategy {
                SplitStrategy::Copy => {
                    fs::copy(file_path, &dest_path)?;
                },
                SplitStrategy::HardLink => {
                    match fs::hard_link(file_path, &dest_path) {
                        Ok(e) => e,
                        Err(e) => {
                            println!("Failed to hardlink {}", e);
                            fs::copy(file_path, &dest_path)?;
                        }
                    }
                },
                SplitStrategy::Move => {
                    fs::rename(file_path, &dest_path)?;
                }
            }

            if i % 10 == 0 && i > 0 {
                let icon = match strategy {
                    SplitStrategy::Copy => "📑",
                    SplitStrategy::HardLink => "🔗",
                    SplitStrategy::Move => "🚚",
                };
                println!("   [{}] {} Processed {} files...", label, icon, i);
            }
        }
    }

    Ok(())
}

pub fn run(src_dir: &str, dst_dir: &str, ratio: f32, strategy: SplitStrategy) -> io::Result<()> {
    let src_path = Path::new(src_dir);

    if !src_path.exists() || !src_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source folder not found or is not a directory"));
    }

    if ratio <= 0.0 || ratio >= 1.0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ratio must be between 0.0 and 1.0"));
    }

    println!("Reading files from: {:?}", src_path);

    let mut files = collect_files(src_path)?;
    let total_files = files.len();

    if total_files == 0 {
        println!("⚠️  No files found to split.");
        return Ok(());
    }

    println!("Found {} files. Shuffling...", total_files);

    let mut rng = rng();
    files.shuffle(&mut rng);

    let split_idx = (total_files as f32 * ratio).round() as usize;
    let (train_files, test_files) = files.split_at(split_idx);

    println!("Splitting: {} Train / {} Test", train_files.len(), test_files.len());

    let output_root = Path::new(dst_dir);
    let train_dir = output_root.join("train");
    let test_dir = output_root.join("test");

    fs::create_dir_all(&train_dir)?;
    fs::create_dir_all(&test_dir)?;

    println!("Processing files ...");
    process_files(train_files, &train_dir, "Train", strategy)?;
    process_files(test_files, &test_dir, "Test", strategy)?;

    println!("Done! Check folder {:?}", dst_dir);
    Ok(())
}