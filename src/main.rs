use jump_kun::history;
use jump_kun::{dir_finder, select_item};
extern crate skim;
use dirs::home_dir;
use jump_kun::jump_then_add_to_hist::jump_then_add_to_hist;
use jump_kun::structs::{Dir, DirVec};
use skim::SkimOptionsBuilder;

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .bind(vec![
            "shift-tab:execute(UP_FROM={} jump-kun)+abort",
            "tab:execute(DOWN_FROM={} jump-kun)+abort",
        ])
        .layout("reverse")
        .multi(false)
        .preview(Some("ls {}"))
        .preview_window(Some("right:30%:wrap"))
        .build()
        .unwrap();

    let mut default_db_path = home_dir().unwrap();
    default_db_path.push(".config/jump-kun/history");

    let history_dirs: DirVec = history::read(default_db_path.to_str().unwrap());
    let mut found_dirs: DirVec = dir_finder::find_dirs();
    let current_dir: Dir = dir_finder::current_dir();

    found_dirs.append(history_dirs);
    found_dirs.push(current_dir);
    found_dirs.sort();
    let item = select_item::select(found_dirs.all_path_to_string(), &options);
    jump_then_add_to_hist(item, default_db_path.to_str().unwrap());
}
