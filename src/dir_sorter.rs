use super::structs::DirInfo;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn to_sorted_string(found_dirs: HashMap<PathBuf, DirInfo>, top_dir: PathBuf) -> String {
    let mut all_dirs = String::new();
    let mut dir_vec: Vec<(PathBuf, DirInfo)> = vec![];

    for (k, v) in found_dirs.iter() {
        dir_vec.push((k.to_path_buf(), v.to_owned()))
    }

    let current_dir = top_dir.to_str().unwrap().to_owned();

    for d in sort_dirs(dir_vec, &current_dir) {
        all_dirs.push_str(&format!("{}\n", d.0.to_str().unwrap()))
    }
    all_dirs
}

fn sort_dirs(
    mut dir_vec: Vec<(PathBuf, DirInfo)>,
    current_dir: &String,
) -> Vec<(PathBuf, DirInfo)> {
    dir_vec.sort_by(|a, b| {
        ((b.0 == PathBuf::from(&current_dir)) as i32)
            .cmp(&((a.0 == PathBuf::from(&current_dir)) as i32))
            .then(
                (b.0.starts_with(&current_dir) as i32).cmp(&(a.0.starts_with(&current_dir) as i32)),
            )
            .then(b.1.cd_count.cmp(&a.1.cd_count))
            .then(b.0.cmp(&a.0))
    });
    dir_vec
}
