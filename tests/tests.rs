use dirs::home_dir;
use jump_kun::history_to_hash::history_hash;
use jump_kun::structs::DirInfo;
use std::collections::HashMap;
use std::path::PathBuf;

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
        history.insert(home_dir().unwrap(), DirInfo::new(1, true));
        let result = history_hash(serde_json::to_string(&history).unwrap());
        assert_eq!(result.len(), 1);
    }
}
