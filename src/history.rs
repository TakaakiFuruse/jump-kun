use super::structs::{Dir, DirVec};
use crate::enums::DirType;
use anyhow::Result;
use serde_json;
use std::path::PathBuf;
use std::str;

pub fn read(path: &PathBuf) -> Result<DirVec> {
    let tree = sled::open(path)?;
    let dirvec: DirVec = tree
        .iter()
        .filter_map(|k| match k {
            Ok((k, v)) => {
                if PathBuf::from(str::from_utf8(&k).unwrap_or("str from error")).exists() {
                    let d: Dir =
                        serde_json::from_str(&str::from_utf8(&v).unwrap_or("str from error"))
                            .unwrap_or(Dir::invalid());

                    Some(d)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .filter(|d| {
            if let DirType::Invalid = d.dirtype {
                false
            } else {
                true
            }
        })
        .collect();
    Ok(dirvec)
}
