pub mod compare;

use crate::common::sha256;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone)]
pub struct File {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) sha256: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Dir {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) sha256: String,
    pub(crate) files: Vec<File>,
    pub(crate) dirs: Vec<Dir>,
}

impl File {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>, sha256: String) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            sha256,
        }
    }
}

pub struct DirBuilder {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) files: Vec<File>,
    pub(crate) dirs: Vec<Dir>,
}

impl DirBuilder {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            files: Vec::default(),
            dirs: Vec::default(),
        }
    }
    pub fn update_sub_dirs(&mut self, dirs: Vec<Dir>) {
        self.dirs = dirs;
    }
    pub fn update_sub_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    // fn sort(&mut self) {
    //     self.dirs.sort_by(|x, y| x.name.cmp(&y.name));
    //     self.files.sort_by(|x, y| x.name.cmp(&y.name));
    // }

    pub fn build(self) -> Dir {
        let DirBuilder {
            name,
            path,
            mut files,
            mut dirs,
        } = self;
        files.sort_by(|x, y| x.name.cmp(&y.name));
        dirs.sort_by(|x, y| x.name.cmp(&y.name));

        let res = files.iter().fold(String::new(), |mut x, item| {
            x.push_str(item.sha256.as_str());
            x
        });
        let res = dirs.iter().fold(res, |mut x, item| {
            x.push_str(item.sha256.as_str());
            x
        });
        let sha256 = sha256(res.as_bytes());
        Dir {
            name,
            path,
            files,
            dirs,
            sha256,
        }
    }
}

impl Dir {
    pub fn find_dir(&self, name: &str) -> Option<&Dir> {
        if self.name.as_str() == name {
            return Some(&self);
        } else {
            for sub in self.dirs.iter() {
                let res = sub.find_dir(name);
                if res.is_some() {
                    return res;
                }
            }
        }
        None
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "path: {:?}, sha256: {}", self.path, self.sha256)
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "path: {:?}, sha256: {}, files: {:?}, sub_dirs: {:?}",
            self.path, self.sha256, self.files, self.dirs
        )
    }
}
