use super::dir_check;
use super::enums::DirType;
use super::structs::{Dir, DirBuilder, DirVec};
use walkdir::WalkDir;

pub fn start_walking_around(from: Dir) -> DirVec {
    let mut dirs = DirVec::new();

    for dir in from.path.ancestors() {
        let parent_dirs = WalkDir::new(&dir)
            .min_depth(0)
            .max_depth(0)
            .into_iter()
            .filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e));

        let child_dirs = WalkDir::new(&dir)
            .min_depth(1)
            .max_depth(3)
            .into_iter()
            .filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e));

        for entry in parent_dirs {
            if entry.is_ok() {
                dirs.push(
                    DirBuilder::default()
                        .path(entry.unwrap().into_path())
                        .dirtype(DirType::ParentDir)
                        .build()
                        .unwrap(),
                );
            }
        }
        for entry in child_dirs {
            if entry.is_ok() {
                dirs.push(
                    DirBuilder::default()
                        .path(entry.unwrap().into_path())
                        .dirtype(DirType::ChildDir)
                        .build()
                        .unwrap(),
                );
            }
        }
    }
    dirs
}

pub fn start_walking_down(from: Dir) -> DirVec {
    let mut dirs = DirVec::new();

    let child_dirs = WalkDir::new(from.path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e));

    for entry in child_dirs {
        if entry.is_ok() {
            dirs.push(
                DirBuilder::default()
                    .path(entry.unwrap().into_path())
                    .dirtype(DirType::ChildDir)
                    .build()
                    .unwrap(),
            );
        }
    }
    dirs
}

pub fn start_walking_up(from: Dir) -> DirVec {
    let mut dirs = DirVec::new();

    for dir in from.path.ancestors() {
        let parent_dirs = WalkDir::new(&dir)
            .min_depth(0)
            .max_depth(0)
            .into_iter()
            .filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e));

        for entry in parent_dirs {
            if entry.is_ok() {
                dirs.push(
                    DirBuilder::default()
                        .path(entry.unwrap().into_path())
                        .dirtype(DirType::ParentDir)
                        .build()
                        .unwrap(),
                );
            }
        }
    }
    dirs
}
