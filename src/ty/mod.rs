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
    pub(crate) success_dirs: usize,
    pub(crate) fail_dirs: usize,
    pub(crate) success_files: usize,
    pub(crate) fail_files: usize,
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
    pub(crate) success_dirs: usize,
    pub(crate) fail_dirs: usize,
    pub(crate) success_files: usize,
    pub(crate) fail_files: usize,
}

impl DirBuilder {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            files: Vec::default(),
            dirs: Vec::default(),
            success_dirs: usize::default(),
            fail_dirs: usize::default(),
            success_files: usize::default(),
            fail_files: usize::default(),
        }
    }
    pub fn update_sub_dirs(&mut self, dirs: Vec<Dir>) {
        for a in dirs.iter() {
            self.update_sub_dir_count(a);
        }
        self.dirs = dirs;
        // self.success_dirs += self.dirs.len();
    }
    pub fn update_sub_files(&mut self, files: Vec<File>) {
        self.files = files;
    }
    pub fn update_sub_dir_count(&mut self, dir: &Dir) {
        self.success_dirs += dir.success_dirs;
        self.fail_dirs += dir.fail_dirs;
        self.success_files += dir.success_files;
        self.fail_files += dir.fail_files;
    }

    pub fn update_files_count(&mut self, success: usize, fail: usize) {
        self.success_files += success;
        self.fail_files += fail;
    }
    pub fn update_dirs_count(&mut self, success: usize, fail: usize) {
        self.success_dirs += success;
        self.fail_dirs += fail;
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
            success_dirs,
            fail_dirs,
            success_files,
            fail_files,
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
            success_dirs,
            fail_dirs,
            success_files,
            fail_files,
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
