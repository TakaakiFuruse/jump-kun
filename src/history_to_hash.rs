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

fn history_hash(history: String) -> HashMap<PathBuf, DirInfo> {
    let mut hm: HashMap<PathBuf, DirInfo> =
        serde_json::from_str(&history).unwrap_or(HashMap::new());
    hm.retain(|k, _| k.exists());
    hm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_hash_returns_empty_hashmap_if_dir_not_exist() {
        let mut history: HashMap<PathBuf, DirInfo> = HashMap::new();
        history.insert(PathBuf::from(r"\this\doesnt\exists"), DirInfo::new(1, true));
        let result = history_hash(serde_json::to_string(&history).unwrap());
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_history_hash_returns_empty_hashmap_if_dir_exists() {
        let mut history: HashMap<PathBuf, DirInfo> = HashMap::new();
        history.insert(dirs::home_dir().unwrap(), DirInfo::new(1, true));
        let result = history_hash(serde_json::to_string(&history).unwrap());
        assert_eq!(result.len(), 1);
    }
}
