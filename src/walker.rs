use super::dir_check;
use super::enums::DirType;
use super::structs::{Dir, DirVec};
use anyhow::Result;
use ignore::gitignore::Gitignore;
use jwalk::WalkDir;

pub fn start_walking_around(from: Dir, jump_kun_ignore: &Gitignore) -> Result<DirVec> {
    let mut dirs = DirVec::new();

    for dir in from.path.ancestors() {
        let parent_dirs = WalkDir::new(&dir)
            .min_depth(0)
            .max_depth(0)
            .into_iter()
            .filter(|e| match &e.as_ref() {
                Ok(ee) => dir_check::must_be_included(&ee.path(), jump_kun_ignore),
                Err(_) => false,
            });

        let child_dirs = WalkDir::new(&dir)
            .min_depth(1)
            .max_depth(4)
            .into_iter()
            .filter(|e| match &e.as_ref() {
                Ok(ee) => dir_check::must_be_included(&ee.path(), jump_kun_ignore),
                Err(_) => false,
            });

        for entry in parent_dirs {
            if entry.is_ok() {
                dirs.push(
                    Dir::default()
                        .path(entry?.path())
                        .dirtype(DirType::ParentDir),
                );
            }
        }
        for entry in child_dirs {
            if entry.is_ok() {
                dirs.push(
                    Dir::default()
                        .path(entry?.path())
                        .dirtype(DirType::ChildDir),
                );
            }
        }
    }
    Ok(dirs)
}

pub fn start_walking_down(from: Dir, jump_kun_ignore: &Gitignore) -> Result<DirVec> {
    let mut dirs = DirVec::new();

    let child_dirs = WalkDir::new(from.path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter(|e| match &e.as_ref() {
            Ok(ee) => dir_check::must_be_included(&ee.path(), jump_kun_ignore),
            Err(_) => false,
        });

    for entry in child_dirs {
        if entry.is_ok() {
            dirs.push(
                Dir::default()
                    .path(entry?.path())
                    .dirtype(DirType::ChildDir),
            );
        }
    }
    Ok(dirs)
}

pub fn start_walking_up(from: Dir, jump_kun_ignore: &Gitignore) -> Result<DirVec> {
    let mut dirs = DirVec::new();

    for dir in from.path.ancestors() {
        let parent_dirs = WalkDir::new(&dir)
            .min_depth(0)
            .max_depth(0)
            .into_iter()
            .filter(|e| match &e.as_ref() {
                Ok(ee) => dir_check::must_be_included(&ee.path(), jump_kun_ignore),
                Err(_) => false,
            });

        for entry in parent_dirs {
            if entry.is_ok() {
                dirs.push(
                    Dir::default()
                        .path(entry?.path())
                        .dirtype(DirType::ParentDir),
                );
            }
        }
    }
    Ok(dirs)
}
