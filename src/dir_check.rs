use dirs::home_dir;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::fs::File;
use std::path::PathBuf;

pub fn create_ignore_if_not_found() -> File {
    let mut path = home_dir().unwrap();
    path.push(".config/jump-kun/.jump_kun_ignore");
    match File::open(&path) {
        Ok(e) => e,
        Err(_e) => {
            File::create(&path).unwrap_or_else(|_| panic!("could not create .jump_kun_ignore file"))
        }
    }
}
pub fn create_jump_kun_ignore() -> Gitignore {
    create_ignore_if_not_found();
    let mut home_path = home_dir().unwrap();
    home_path.push(".config/jump-kun/.jump_kun_ignore");
    let mut builder = GitignoreBuilder::new(home_dir().unwrap());
    builder.add(home_path);
    builder
        .build()
        .unwrap_or_else(|_| panic!("could not create .jump_kun_ignore file"))
}

pub fn must_be_included(entry: &PathBuf, jump_kun_ignore: &Gitignore) -> bool {
    if entry.is_dir() {
        jump_kun_ignore.matched(entry, true).is_none()
    } else {
        false
    }
}
