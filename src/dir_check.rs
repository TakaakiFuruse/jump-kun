use dirs::home_dir;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::fs::File;
use walkdir::DirEntry;

fn is_directory(entry: &DirEntry) -> bool {
    entry.path().is_dir()
}

pub fn create_ignore_if_not_found() -> File {
    let mut path = home_dir().unwrap();
    path.push(".config/jump-kun/.jump_kun_ignore");
    match File::open(&path) {
        Ok(e) => e,
        Err(_e) => File::create(&path).unwrap(),
    }
}
pub fn create_jump_kun_ignore() -> Gitignore {
    create_ignore_if_not_found();
    let mut home_path = home_dir().unwrap();
    home_path.push(".config/jump-kun/.jump_kun_ignore");
    let mut builder = GitignoreBuilder::new(home_dir().unwrap());
    builder.add(home_path);
    builder.build().unwrap()
}

pub fn must_be_included(entry: &DirEntry, jump_kun_ignore: &Gitignore) -> bool {
    if is_directory(entry) {
        jump_kun_ignore.matched(entry.path(), true).is_none()
    } else {
        false
    }
}
