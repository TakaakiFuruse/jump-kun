use super::structs::DirInfo;
use super::walker::walk_around;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub fn up_down() -> HashMap<PathBuf, DirInfo> {
    match env::var("DOWN_FROM") {
        Ok(dir) => walk_around(PathBuf::from(dir)),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => walk_around(
                PathBuf::from(dir)
                    .parent()
                    .unwrap_or(Path::new("/"))
                    .to_path_buf(),
            ),
            Err(_) => walk_around(env::current_dir().unwrap()),
        },
    }
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
