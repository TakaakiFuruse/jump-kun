use jump_kun::history_to_hash::read_history;
use jump_kun::{dir_up_down, select_item};
extern crate skim;
use dirs::home_dir;
use jump_kun::dir_check;
use jump_kun::dir_sorting::dir_string;
use jump_kun::jump_then_add_to_hist::jump_then_add_to_hist;
use jump_kun::structs::DirInfo;
use skim::SkimOptionsBuilder;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

// shell script
// function jump-kun-jump(){
//     local selected=$(jump-kun)
//         if [[ -n $selected ]]; then
//         \cd $selected
//         fi
// }

fn log_all_dirs() {
    let mut h: HashMap<PathBuf, DirInfo> = HashMap::new();
    WalkDir::new("/home/orz/")
        .into_iter()
        .filter_entry(|e| {
            !dir_check::is_git_dir(e) && dir_check::is_directory(e) && dir_check::is_not_hidden(e)
        })
        .for_each(|x| {
            if x.is_ok() {
                h.insert(
                    x.unwrap().path().to_path_buf(),
                    DirInfo {
                        cd_count: 1,
                        from_history: true,
                    },
                );
            };
        });
    WalkDir::new("/mnt/")
        .into_iter()
        .filter_entry(|e| {
            !dir_check::is_git_dir(e) && dir_check::is_directory(e) && dir_check::is_not_hidden(e)
        })
        .for_each(|x| {
            if x.is_ok() {
                h.insert(
                    x.unwrap().path().to_path_buf(),
                    DirInfo {
                        cd_count: 1,
                        from_history: true,
                    },
                );
            };
        });
    let mut history_dir: PathBuf = home_dir().unwrap();
    history_dir.push(".config/jump-kun/history.log");
    let mut log_file = BufWriter::new(File::create(history_dir).unwrap());
    let b: &str = &serde_json::to_string(&h).unwrap();
    let _ = log_file.write(&b.as_bytes());
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .bind(vec![
            "shift-tab:execute(UP_FROM={} jump-kun)+abort",
            "tab:execute(DOWN_FROM={} jump-kun)+abort",
        ])
        .layout("reverse")
        .multi(false)
        .preview(Some("ls {}"))
        .build()
        .unwrap();

    if env::var("LOG_ALL_DIRS") == std::result::Result::Ok("true".to_string()) {
        log_all_dirs();
    }
    let history_hash: HashMap<PathBuf, DirInfo> = read_history();
    let mut found_dirs: HashMap<PathBuf, DirInfo> = dir_up_down::up_down();

    found_dirs.extend(history_hash.clone());

    let all_dir_string = dir_string(found_dirs);

    let item = select_item::select(all_dir_string, &options);
    jump_then_add_to_hist(item, history_hash);
}
