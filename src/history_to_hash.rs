use super::structs::DirInfo;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

pub fn read_history() -> HashMap<PathBuf, DirInfo> {
    let mut history_dir: PathBuf = home_dir().unwrap();
    history_dir.push(".config/jump-kun/history.log");
    let f = File::open(history_dir).expect("history.log not found");

    let mut history = String::new();
    let _buf = match BufReader::new(f).read_to_string(&mut history) {
        Ok(e) => e,
        Err(_) => 0,
    };
    let history_hash: HashMap<PathBuf, DirInfo> = history_hash(history);
    history_hash
}

pub fn history_hash(history: String) -> HashMap<PathBuf, DirInfo> {
    let mut hm: HashMap<PathBuf, DirInfo> =
        serde_json::from_str(&history).unwrap_or(HashMap::new());
    hm.retain(|k, _| k.exists());
    hm
}
