use super::dir_check;
use super::structs::DirInfo;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn start_walking(from: PathBuf) -> HashMap<PathBuf, DirInfo> {
    let mut dirs = HashMap::new();

    for dir in from.ancestors() {
        let around_dirs = WalkDir::new(&dir)
            .min_depth(0)
            .max_depth(2)
            .into_iter()
            .filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e));

        for entry in around_dirs {
            if entry.is_ok() {
                dirs.insert(entry.unwrap().into_path(), DirInfo::new(0, false));
            }
        }
    }
    dirs
}
