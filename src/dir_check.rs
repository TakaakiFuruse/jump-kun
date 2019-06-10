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
