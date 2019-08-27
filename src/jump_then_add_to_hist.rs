use super::structs::Dir;
use serde_json;
use sled::Db;
use std::path::PathBuf;
use std::str;

pub fn jump_then_add_to_hist(item: String, path: &str) {
    if !item.is_empty() {
        let tree = Db::open(path).unwrap();
        print!("{}", &item);
        let new_visited_dir = Dir::new_visited(PathBuf::from(&item));
        let insertion = match tree.update_and_fetch(&item.as_bytes(), update) {
            Ok(v) => match v {
                None => tree.insert(
                    &item.as_bytes(),
                    serde_json::to_string(&new_visited_dir).unwrap().as_bytes(),
                ),
                _ => Ok(v),
            },
            Err(_v) => Err(_v),
        };
        insertion.expect("New item insertion failed!!");
        tree.flush().expect("DB flush failed");
    }
}

fn update(old: Option<&[u8]>) -> Option<Vec<u8>> {
    match old {
        Some(bytes) => {
            let mut dir: Dir = serde_json::from_str(str::from_utf8(&bytes).unwrap()).unwrap();
            dir.add_cd_count();
            Some(serde_json::to_string(&dir).unwrap().as_bytes().to_vec())
        }
        None => None,
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test_for_jump_then_add_to_hist {
    use super::*;

    #[test]
    fn create_a_new_dir_entry_and_correctly_increases_cd_count() {
        let path_str = "./tests/test_db";
        {
            let tree = Db::open(&path_str).unwrap();
            tree.clear();
        }

        // first visit
        {
            jump_then_add_to_hist("/first/visit/dir".to_string(), &path_str);
        }

        // creates entry with cd_count 1
        {
            let tree = Db::open(&path_str).unwrap();

            let item = "/first/visit/dir";
            assert_eq!(tree.get(&item).is_ok(), true);

            let s: String = str::from_utf8(&tree.get(&item).unwrap().unwrap())
                .unwrap()
                .to_owned();
            let dir: Dir = serde_json::from_str(&s).unwrap();

            assert_eq!(dir.cd_count, 1);
        }

        // second visit
        {
            jump_then_add_to_hist("/first/visit/dir".to_string(), &path_str);
        }

        // cd_count will be 2
        {
            let tree = Db::open(&path_str).unwrap();

            let item = "/first/visit/dir";
            assert_eq!(tree.get(&item).is_ok(), true);

            let s: String = str::from_utf8(&tree.get(&item).unwrap().unwrap())
                .unwrap()
                .to_owned();
            let dir: Dir = serde_json::from_str(&s).unwrap();

            assert_eq!(dir.cd_count, 2);
        }
    }

}
