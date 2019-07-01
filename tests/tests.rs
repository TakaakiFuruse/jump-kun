use jump_kun::walker::walk_around;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walker_walk_around() {
        let test_path = PathBuf::from("/this/is/test/dir");
        let res_hash = walk_around(test_path);
        println!("{:?}", res_hash);
        assert!(1 != 1);
    }
}
