use super::structs::DirInfo;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

pub fn jump_then_add_to_hist(item: String, mut history_hash: HashMap<PathBuf, DirInfo>) {
    if item.len() > 0 {
        print!("{}", item);

        let path_buf = PathBuf::from(&item);
        match history_hash.get_mut(&path_buf) {
            Some(e) => e.cd_count += 1,
            None => {
                history_hash.insert(path_buf, DirInfo::new(1, true));
                ()
            }
        }
        let mut history_dir: PathBuf = home_dir().unwrap();
        history_dir.push(".config/jump-kun/history.log");
        let mut h = BufWriter::new(File::create(history_dir).unwrap());
        let b: &str = &serde_json::to_string(&history_hash).unwrap();
        let _ = h.write(&b.as_bytes());
    }
}
