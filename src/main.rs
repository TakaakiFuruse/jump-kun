use jump_kun::history;
use jump_kun::{dir_check, dir_finder, select_item};
extern crate skim;
use anyhow::Result;
use dirs::home_dir;
use jump_kun::jump_then_add_to_hist::jump_then_add_to_hist;
use jump_kun::structs::{Dir, DirVec};
use skim::SkimOptions;
use skim::prelude::SkimOptionsBuilder;

fn runner(options: SkimOptions) -> Result<()> {
    let mut default_db_path = match home_dir() {
        Some(e) => e,
        None => panic!("could not found home directory"),
    };
    default_db_path.push(".config/jump-kun/history");

    let jump_kun_ignore = dir_check::create_jump_kun_ignore();

    let history_dirs: DirVec = history::read(&default_db_path)?;
    let mut found_dirs: DirVec = dir_finder::find_dirs(jump_kun_ignore)?;
    let current_dir: Dir = dir_finder::current_dir()?;

    found_dirs.append(history_dirs);
    found_dirs.push(current_dir);
    found_dirs.sort();
    let item = select_item::select(found_dirs.all_path_to_str(), &options);
    jump_then_add_to_hist(item, default_db_path.to_str().unwrap_or(""));
    Ok(())
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .bind(vec![
            "shift-tab:execute(UP_FROM={} jump-kun)+abort",
            "tab:execute(DOWN_FROM={} jump-kun)+abort",
        ])
        .layout("reverse")
        .multi(false)
        .build()
        .unwrap();
    match runner(options) {
        Ok(_e) => (),
        Err(_e) => panic!("runner error"),
    }
}
