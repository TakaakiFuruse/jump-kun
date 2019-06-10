use jump_kun::dir_check;
use std::env;
use walkdir::WalkDir;

fn main() {
    let cwd = env::current_dir();

    let walker = WalkDir::new(cwd.unwrap())
        .min_depth(0)
        .max_depth(1)
        .into_iter();
    for entry in walker.filter_entry(|e| !dir_check::is_git_dir(e) && dir_check::is_directory(e)) {
        println!("{}", entry.unwrap().path().display());
    }
}
