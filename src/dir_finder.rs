use super::structs::DirInfo;
use super::walker::start_walking;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub fn find_dirs() -> HashMap<PathBuf, DirInfo> {
    match env::var("DOWN_FROM") {
        Ok(dir) => start_walking(from_specific_directory(dir)),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => start_walking(from_parent_directory_or_root(dir)),
            Err(_) => start_walking(from_current_directory()),
        },
    }
}

fn from_specific_directory(dir: String) -> PathBuf {
    PathBuf::from(dir)
}

fn from_current_directory() -> PathBuf {
    env::current_dir().unwrap()
}

fn from_parent_directory_or_root(dir: String) -> PathBuf {
    PathBuf::from(dir)
        .parent()
        .unwrap_or(Path::new("/"))
        .to_path_buf()
}

pub fn current_dir() -> PathBuf {
    match env::var("DOWN_FROM") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => PathBuf::from(dir).parent().unwrap().to_path_buf(),
            Err(_) => env::current_dir().unwrap(),
        },
    }
}
