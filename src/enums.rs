use jump_kun_macros::order;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, order, PartialOrd)]
pub enum DirType {
    CurrentDir,
    ParentDir,
    ChildDir,
    VisitedDir,
    NotSure,
    Invalid,
}

#[cfg(test)]
mod tests_for_dirtype {
    use super::*;

    #[test]
    fn enum_returns_order() {
        assert_eq!(DirType::CurrentDir.order() > -1, true);
        assert_eq!(DirType::NotSure.order() > -1, true);
    }
}
