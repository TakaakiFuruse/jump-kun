use super::enums::DirType;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::path::PathBuf;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, PartialOrd)]
pub struct Dir {
    pub path: PathBuf,
    pub cd_count: u32,
    pub dirtype: DirType,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            path: home_dir().unwrap(),
            cd_count: 0,
            dirtype: DirType::NotSure,
        }
    }
}

impl Dir {
    pub fn invalid() -> Self {
        Self {
            path: PathBuf::from(""),
            cd_count: 0,
            dirtype: DirType::Invalid,
        }
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    pub fn dirtype(mut self, dir_type: DirType) -> Self {
        self.dirtype = dir_type;
        self
    }

    pub fn new(path: PathBuf, cd_count: u32, dirtype: DirType) -> Dir {
        Dir {
            path,
            cd_count,
            dirtype,
        }
    }

    pub fn new_visited(dir: PathBuf) -> Self {
        Self {
            path: dir,
            cd_count: 1,
            dirtype: DirType::VisitedDir,
        }
    }

    pub fn add_cd_count(&mut self) {
        self.cd_count += 1;
    }

    pub fn as_dirtype(&mut self, dirtype: DirType) {
        self.dirtype = dirtype
    }
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize, Clone, Default)]
pub struct DirVec {
    pub map: Vec<Dir>,
}

impl FromIterator<Dir> for DirVec {
    fn from_iter<I: IntoIterator<Item = Dir>>(iter: I) -> Self {
        let mut dirvec = DirVec::new();

        for i in iter {
            dirvec.map.push(i);
        }
        dirvec
    }
}

impl DirVec {
    pub fn new() -> DirVec {
        DirVec { map: Vec::new() }
    }
    pub fn push(&mut self, elm: Dir) {
        self.map.push(elm)
    }
    pub fn append(&mut self, mut v: DirVec) {
        self.map.append(&mut v.map)
    }

    pub fn all_path_to_str(&self) -> String {
        let s: String = self
            .map
            .iter()
            .map(|elm| {
                format!(
                    "{}\n",
                    elm.path
                        .to_str()
                        .unwrap_or("DirVec::all_path_to_string error")
                )
            })
            .collect();
        s
    }

    pub fn sort(&mut self) {
        &self.map.sort_by(|a, b| {
            a.dirtype
                .order()
                .cmp(&b.dirtype.order())
                .then(a.cd_count.cmp(&b.cd_count).reverse())
        });
    }
}

#[cfg(test)]
mod tests_for_dir {
    use super::*;

    #[test]
    fn add_cd_count() {
        let path = PathBuf::from("/a/dir");
        let mut dir = Dir::new(path, 0, DirType::VisitedDir);
        assert_eq!(dir.cd_count, 0);
        dir.add_cd_count();
        assert_eq!(dir.cd_count, 1);
    }

    #[test]
    fn as_dirtype() {
        let path = PathBuf::from("/a/dir");
        let mut dir = Dir::new(path, 0, DirType::VisitedDir);
        dir.as_dirtype(DirType::ParentDir);
        assert_eq!(dir.dirtype, DirType::ParentDir);
    }

    #[test]
    fn push() {
        let mut dir_vec = DirVec::new();
        let path = PathBuf::from("/a/dir");
        let dir = Dir::new(path, 0, DirType::VisitedDir);
        dir_vec.push(dir);

        let path = PathBuf::from("/a/dir");
        let dir = Dir::new(path, 0, DirType::VisitedDir);
        assert_eq!(dir_vec.map[0], dir);
    }
}

#[cfg(test)]
mod tests_for_dirvec {
    use super::*;

    #[test]
    fn all_path_to_string() {
        let mut dirvec = DirVec::new();
        dirvec.push(Dir::new(PathBuf::from("/a/dir/1"), 0, DirType::VisitedDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/2"), 0, DirType::VisitedDir));
        assert_eq!(dirvec.all_path_to_str(), "/a/dir/1\n/a/dir/2\n");
    }

    #[test]
    fn sort_fun_sorts_dirtypes() {
        let mut dirvec = DirVec::new();
        dirvec.push(Dir::new(PathBuf::from("/a/dir/1"), 0, DirType::NotSure));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/2"), 0, DirType::ParentDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/3"), 0, DirType::CurrentDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/4"), 0, DirType::ChildDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/5"), 0, DirType::VisitedDir));

        let mut sorted_result = DirVec::new();
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/3"), 0, DirType::CurrentDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/5"), 0, DirType::VisitedDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/2"), 0, DirType::ParentDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/4"), 0, DirType::ChildDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/1"), 0, DirType::NotSure));

        dirvec.sort();

        assert_eq!(dirvec, sorted_result);
    }

    #[test]
    fn sort_fun_sorts_dirtypes_by_cdcount() {
        let mut dirvec = DirVec::new();
        dirvec.push(Dir::new(PathBuf::from("/a/dir/5"), 0, DirType::VisitedDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/5"), 2, DirType::VisitedDir));
        dirvec.push(Dir::new(PathBuf::from("/a/dir/5"), 3, DirType::VisitedDir));

        let mut sorted_result = DirVec::new();
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/5"), 3, DirType::VisitedDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/5"), 2, DirType::VisitedDir));
        sorted_result.push(Dir::new(PathBuf::from("/a/dir/5"), 0, DirType::VisitedDir));

        dirvec.sort();

        assert_eq!(dirvec, sorted_result);
    }
}
