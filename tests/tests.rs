use dirs::home_dir;
use jump_kun::dir_sorter::to_sorted_string;
use jump_kun::history_to_hash::history_hash;
use jump_kun::structs::DirInfo;
use jump_kun::walker::start_walking;
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

    #[test]
    fn test_start_walking_returns_hashmap() {
        let result_hash = start_walking(home_dir().unwrap());
        assert_eq!(result_hash.len() > 1, true);
        for (_, v) in result_hash.iter() {
            assert_eq!(v.cd_count, 0);
            assert_eq!(v.from_history, false);
        }
    }

    #[test]
    fn test_start_walking_return_empty_hashmap_if_dir_not_exist() {
        let result_hash = start_walking(PathBuf::from(r"\this\doesnt\exists"));
        assert_eq!(result_hash.len(), 0);
    }

    #[test]
    fn test_to_sorted_string_sort_order() {
        // Current directory is on top.
        // Sub directory of current will be next.
        // Then sort by cd counts
        let mut found_dirs = HashMap::new();
        found_dirs.insert(
            PathBuf::from(r"/test/1"),
            DirInfo {
                cd_count: 1,
                from_history: false,
            },
        );
        found_dirs.insert(
            PathBuf::from(r"/test/2"),
            DirInfo {
                cd_count: 2,
                from_history: false,
            },
        );
        found_dirs.insert(
            PathBuf::from(r"/test/3"),
            DirInfo {
                cd_count: 0,
                from_history: false,
            },
        );
        found_dirs.insert(
            PathBuf::from(r"/sample/1"),
            DirInfo {
                cd_count: 0,
                from_history: false,
            },
        );
        found_dirs.insert(
            PathBuf::from(r"/sample/2"),
            DirInfo {
                cd_count: 1,
                from_history: false,
            },
        );
        let current_directory = PathBuf::from("/test/3");
        let result = to_sorted_string(found_dirs, current_directory);
        assert_eq!(result, "/test/3\n/test/2\n/test/1\n/sample/2\n/sample/1\n");
    }
}
