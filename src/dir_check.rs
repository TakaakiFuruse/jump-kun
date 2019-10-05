use dirs::home_dir;
use ignore::gitignore::GitignoreBuilder;
use std::fs::File;
use std::path::Path;
use walkdir::DirEntry;

pub fn is_git_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".git"))
        .unwrap_or(false)
}

pub fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn create_ignore_if_not_found() -> File {
    let mut path = home_dir().unwrap();
    path.push(".config/jump-kun/.jump_kun_ignore");
    match File::open(&path) {
        Ok(e) => e,
        Err(_e) => File::create(&path).unwrap(),
    }
}

pub fn must_be_ignored(entry: &DirEntry) -> bool {
    create_ignore_if_not_found();
    let mut home_path = home_dir().unwrap();
    home_path.push(".config/jump-kun/");
    let mut builder = GitignoreBuilder::new(home_path);
    builder.add(".jump_kun_ignore");
    builder
        .build()
        .unwrap()
        .matched(Path::new(entry.file_name().to_str().unwrap()), true)
        .is_ignore()
}
