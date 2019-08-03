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
            Ok(v) => {
                if v == None {
                    let res = tree.insert(
                        &item.as_bytes(),
                        serde_json::to_string(&new_visited_dir).unwrap().as_bytes(),
                    );
                    res
                } else {
                    Ok(v)
                }
            }
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
