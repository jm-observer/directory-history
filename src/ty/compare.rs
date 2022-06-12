use serde::Serialize;
use std::cmp::Ordering;
#[derive(Eq, PartialEq, PartialOrd, Ord, Serialize)]
enum FileChangeTy {
    Add,
    Delete,
    Modify,
}
#[derive(Eq, PartialEq, PartialOrd, Serialize)]
enum FileTy {
    File,
    Dir,
}

#[derive(Eq, PartialEq, PartialOrd, Serialize)]
pub struct ChangeRecord {
    path: String,
    file_ty: FileTy,
    change_ty: FileChangeTy,
}

impl ChangeRecord {
    pub fn init_add_file_record(name: String) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Add,
        }
    }
    pub fn init_delete_file_record(name: String) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Delete,
        }
    }
    pub fn init_modiry_file_record(name: String) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Modify,
        }
    }

    pub fn init_add_dir_record(name: String) -> Self {
        Self {
            path: name,
            file_ty: FileTy::Dir,
            change_ty: FileChangeTy::Add,
        }
    }
    pub fn init_delete_dir_record(name: String) -> Self {
        Self {
            path: name,
            file_ty: FileTy::Dir,
            change_ty: FileChangeTy::Delete,
        }
    }
}

impl Ord for ChangeRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.file_ty == other.file_ty {
            if self.change_ty == other.change_ty {
                self.path.cmp(&other.path)
            } else {
                self.change_ty.cmp(&other.change_ty)
            }
        } else {
            self.file_ty.cmp(&other.file_ty)
        }
    }
}

impl Ord for FileTy {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if *self == FileTy::Dir {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
