use serde::Serialize;
use std::cmp::Ordering;
use std::path::PathBuf;

#[repr(u64)]
#[derive(Eq, PartialEq, PartialOrd, Serialize, Debug, Copy, Clone)]
enum FileChangeTy {
    Add = 0,
    Delete = 1,
    Modify = 2,
}
#[derive(Eq, PartialEq, PartialOrd, Serialize, Debug)]
enum FileTy {
    File,
    Dir,
}

#[derive(Eq, PartialEq, PartialOrd, Serialize, Debug)]
pub struct ChangeRecord {
    path: PathBuf,
    file_ty: FileTy,
    change_ty: FileChangeTy,
}

impl ChangeRecord {
    pub fn init_add_file_record(name: PathBuf) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Add,
        }
    }
    pub fn init_delete_file_record(name: PathBuf) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Delete,
        }
    }
    pub fn init_modiry_file_record(name: PathBuf) -> Self {
        Self {
            path: name,
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Modify,
        }
    }

    pub fn init_add_dir_record(name: PathBuf) -> Self {
        Self {
            path: name,
            file_ty: FileTy::Dir,
            change_ty: FileChangeTy::Add,
        }
    }
    pub fn init_delete_dir_record(name: PathBuf) -> Self {
        Self {
            path: name,
            file_ty: FileTy::Dir,
            change_ty: FileChangeTy::Delete,
        }
    }
}

// impl FileChangeTy {
//     fn val_u64(&self) -> u64 {
//         *self as u64
//     }
// }

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
impl Ord for FileChangeTy {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = *self as u64;
        let other_val = *other as u64;
        Ord::cmp(&self_val, &other_val)
    }
}

#[cfg(test)]
mod test {
    use crate::ty::compare::{ChangeRecord, FileChangeTy, FileTy};

    #[test]
    fn test_file_change_ty_ord() {
        {
            assert!(FileChangeTy::Add == FileChangeTy::Add);
            assert!(FileChangeTy::Add < FileChangeTy::Delete);
            assert!(FileChangeTy::Add < FileChangeTy::Modify);
            assert!(FileChangeTy::Delete < FileChangeTy::Modify);
        }
    }
    #[test]
    fn test_ord() {
        let mut one = ChangeRecord {
            path: "1".into(),
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Add,
        };
        let two = ChangeRecord {
            path: "2".into(),
            file_ty: FileTy::File,
            change_ty: FileChangeTy::Add,
        };
        assert!(two > one);
        {
            assert!(
                ChangeRecord {
                    path: "1".into(),
                    file_ty: FileTy::File,
                    change_ty: FileChangeTy::Delete,
                } > ChangeRecord {
                    path: "2".into(),
                    file_ty: FileTy::File,
                    change_ty: FileChangeTy::Add,
                }
            );
        }
    }
}
