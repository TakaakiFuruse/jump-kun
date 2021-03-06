use dirs::home_dir;
use jump_kun::dir_check::create_jump_kun_ignore;
use jump_kun::enums::DirType;
use jump_kun::structs::{Dir, DirVec};
use jump_kun::walker::{start_walking_around, start_walking_down, start_walking_up};
use std::path::PathBuf;

#[cfg(test)]
#[allow(unused_must_use)]
mod tests_for_history {
    use super::*;
    use dirs::home_dir;
    use jump_kun::history::read;
    use serde_json;

    #[test]
    fn read_returns_dirvec_with_existing_dirs() {
        // Treeをスコープ内で殺さないとロックが外れない
        {
            let tree = sled::open("./tests/test_db").unwrap();
            tree.clear();
            tree.insert(
                home_dir().unwrap().to_str().unwrap().as_bytes(),
                serde_json::to_string(&Dir::default()).unwrap().as_bytes(),
            )
            .expect("could not insert test data");

            tree.insert(
                "/not/exists/dir".as_bytes(),
                serde_json::to_string(
                    &Dir::default()
                        .path(PathBuf::from("/not/exists/dir"))
                        .dirtype(DirType::NotSure),
                )
                .unwrap()
                .as_bytes(),
            )
            .expect("could not insert test data");
            tree.flush().expect("flush faild");
        }
        let dirvec = read(&PathBuf::from("./tests/test_db"));
        let result_vec = DirVec {
            map: vec![Dir::default()],
        };
        assert_eq!(dirvec.unwrap(), result_vec);
    }
}

#[cfg(test)]
mod tests_for_walking {
    use super::*;

    #[test]
    fn start_walking_down_returns_dirvec() {
        let dir = Dir::default().path(home_dir().unwrap());
        let result = start_walking_down(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result.map.len() > 1, true);
        for dir in result.map {
            assert_eq!(dir.cd_count, 0);
            assert_eq!(dir.dirtype, DirType::ChildDir);
        }
    }

    #[test]
    fn start_walking_up_returns_dirvec() {
        let dir = Dir::default().path(home_dir().unwrap());
        let result = start_walking_up(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result.map.len() > 1, true);
        for dir in result.map {
            assert_eq!(dir.cd_count, 0);
            assert_eq!(dir.dirtype, DirType::ParentDir);
        }
    }

    #[test]
    fn start_walking_around_returns_dirvec() {
        let dir = Dir::default().path(home_dir().unwrap());
        let result = start_walking_around(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result.map.len() > 1, true);
        for dir in result.map {
            assert_eq!(dir.cd_count, 0);
        }
    }

    #[test]
    fn start_walking_around_return_empty_dirvec_if_dir_not_exist() {
        let dir = Dir::default().path(PathBuf::from(r"\this\doesnt\exists"));
        let result_hash = start_walking_around(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result_hash.map.len(), 0);
    }
    #[test]
    fn start_walking_down_return_empty_dirvec_if_dir_not_exist() {
        let dir = Dir::default().path(PathBuf::from(r"\this\doesnt\exists"));
        let result_hash = start_walking_down(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result_hash.map.len(), 0);
    }
    #[test]
    fn start_walking_up_return_empty_dirvec_if_dir_not_exist() {
        let dir = Dir::default().path(PathBuf::from(r"\this\doesnt\exists"));
        let result_hash = start_walking_up(dir, &create_jump_kun_ignore()).unwrap();
        assert_eq!(result_hash.map.len(), 0);
    }
}
