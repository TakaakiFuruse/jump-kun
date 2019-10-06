use super::enums::DirType;
use super::structs::{Dir, DirBuilder, DirVec};
use super::walker::{start_walking_around, start_walking_down, start_walking_up};
use ignore::gitignore::Gitignore;

use std::env;
use std::path::{Path, PathBuf};

pub fn find_dirs(jump_kun_ignore: Gitignore) -> DirVec {
    match env::var("DOWN_FROM") {
        Ok(dir) => start_walking_down(from_specific_directory(dir), &jump_kun_ignore),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => start_walking_up(from_parent_directory_or_root(dir), &jump_kun_ignore),
            Err(_) => start_walking_around(from_current_directory(), &jump_kun_ignore),
        },
    }
}

fn from_specific_directory(dir: String) -> Dir {
    DirBuilder::default()
        .path(PathBuf::from(dir))
        .dirtype(DirType::CurrentDir)
        .build()
        .unwrap()
}

fn from_current_directory() -> Dir {
    DirBuilder::default()
        .path(env::current_dir().unwrap())
        .dirtype(DirType::CurrentDir)
        .build()
        .unwrap()
}

fn from_parent_directory_or_root(dir: String) -> Dir {
    DirBuilder::default()
        .path(
            PathBuf::from(dir)
                .parent()
                .unwrap_or_else(|| Path::new("/"))
                .to_path_buf(),
        )
        .dirtype(DirType::CurrentDir)
        .build()
        .unwrap()
}

pub fn current_dir() -> Dir {
    let pathbuf = match env::var("DOWN_FROM") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => match env::var("UP_FROM") {
            Ok(dir) => PathBuf::from(dir).parent().unwrap().to_path_buf(),
            Err(_) => env::current_dir().unwrap(),
        },
    };
    DirBuilder::default()
        .path(pathbuf)
        .dirtype(DirType::CurrentDir)
        .build()
        .unwrap()
}
