pub mod compare;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) sha256: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Dir {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
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
impl Dir {
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

    pub fn sort(&mut self) {
        self.dirs.sort_by_key(|x| x.name.clone());
        self.files.sort_by_key(|x| x.name.clone());
    }
}
