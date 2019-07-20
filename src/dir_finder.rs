use super::structs::DirInfo;
use super::walker::{start_walking_down, start_walking_up};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub fn find_dirs() -> HashMap<PathBuf, DirInfo> {
    match env::var("DOWN_FROM") {
        Ok(dir) => start_walking_down(specific_directory(dir)),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => start_walking_up(parent_directory_or_root(dir)),
            Err(_) => start_walking_down(current_directory()),
        },
    }
}

pub fn current_dir() -> PathBuf {
    match env::var("DOWN_FROM") {
        Ok(dir) => specific_directory(dir),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => parent_directory_or_root(dir),
            Err(_) => current_directory(),
        },
    }
}

fn specific_directory(dir: String) -> PathBuf {
    PathBuf::from(dir)
}

fn current_directory() -> PathBuf {
    env::current_dir().unwrap()
}

fn parent_directory_or_root(dir: String) -> PathBuf {
    PathBuf::from(dir)
        .parent()
        .unwrap_or(Path::new("/"))
        .to_path_buf()
}
