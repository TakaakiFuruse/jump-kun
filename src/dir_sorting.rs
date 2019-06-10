use super::dir_up_down::current_dir;
use super::structs::DirInfo;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn dir_string(found_dirs: HashMap<PathBuf, DirInfo>) -> String {
    let mut all_dirs = String::new();
    let mut sorted_dirs: Vec<(PathBuf, DirInfo)> = vec![];

    for (k, v) in found_dirs.iter() {
        sorted_dirs.push((k.to_path_buf(), v.to_owned()))
    }

    let current_dir = current_dir().to_str().unwrap().to_owned();
    sorted_dirs.sort_by(|a, b| {
        (b.0.starts_with(&current_dir) as i32)
            .cmp(&(a.0.starts_with(&current_dir) as i32))
            .then(b.1.cd_count.cmp(&a.1.cd_count))
            .then(a.0.cmp(&b.0))
    });

    for d in sorted_dirs {
        all_dirs.push_str(&format!("{}\n", d.0.to_str().unwrap()))
    }
    all_dirs
}
