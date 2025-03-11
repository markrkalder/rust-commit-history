use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_dir<P: AsRef<Path>>(start_path: P, excluded_names: &[&str]) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    let mut walker = WalkDir::new(start_path).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let dir_name = e.file_name().to_string_lossy();

            !excluded_names.contains(&dir_name.as_ref())
        } else {
            return false;
        }
    });

    while let Some(entry_result) = walker.next() {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => {
                println!("Error in the iterator");
                continue;
            }
        };

        let path = entry.path();
        if path.join(".git").is_dir() {
            dirs.push(entry.into_path());
            walker.skip_current_dir();
        }
    }
    return dirs;
}
