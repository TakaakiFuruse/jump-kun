use super::structs::{Dir, DirVec};
use serde_json;
use sled::Db;
use std::str;

pub fn read(path: &str) -> DirVec {
    let tree = Db::open(path).unwrap();
    let dirvec: DirVec = tree
        .iter()
        .values()
        .filter_map(|elm| {
            let s: String = str::from_utf8(&elm.unwrap()).unwrap().to_owned();
            let dir: Dir = serde_json::from_str(&s).unwrap();
            if dir.path.exists() {
                Some(dir)
            } else {
                None
            }
        })
        .collect();
    dirvec
}
