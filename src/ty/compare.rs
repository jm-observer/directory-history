enum FileChangeTy {
    Add,
    Delete,
    Modify,
}
enum FileTy {
    File,
    Dir,
}
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
