use jump_kun::history_to_hash::read_history;
use jump_kun::{dir_finder, select_item};
extern crate skim;
use jump_kun::dir_sorter;
use jump_kun::jump_then_add_to_hist::jump_then_add_to_hist;
use jump_kun::structs::DirInfo;
use skim::SkimOptionsBuilder;
use std::collections::HashMap;
use std::path::PathBuf;

// shell script
// function jump-kun-jump(){
//     local selected=$(jump-kun)
//         if [[ -n $selected ]]; then
//         \cd $selected
//         fi
// }

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

    let history_hash: HashMap<PathBuf, DirInfo> = read_history();
    let mut found_dirs: HashMap<PathBuf, DirInfo> = dir_finder::find_dirs();

    found_dirs.extend(history_hash.clone());

    let all_dir_in_string = dir_sorter::to_sorted_string(found_dirs, dir_finder::current_dir());

    let item = select_item::select(all_dir_in_string, &options);
    jump_then_add_to_hist(item, history_hash);
}
