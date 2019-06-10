use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirInfo {
    pub cd_count: u32,
    pub from_history: bool,
}

impl DirInfo {
    pub fn new(cd_count: u32, from_history: bool) -> DirInfo {
        DirInfo {
            cd_count: cd_count,
            from_history: from_history,
        }
    }
}
