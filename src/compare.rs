use crate::ty::{Dir, File};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn compare(
    before_data_path: impl AsRef<Path>,
    data_path: impl AsRef<Path>,
) -> Result<()> {
    Ok(())
}

pub async fn compare_dir(mut dir_before: Dir, mut dir: Dir) -> Result<()> {
    dir_before.sort();
    dir.sort();

    let mut index_before = 0;
    let mut index = 0;
    let file_num = dir.files.len();
    let file_before_num = dir_before.files.len();
    let mut file: &File;
    let mut file_before: &File;
    loop {
        file = if index >= file_num {
            // 剩下的旧文件都为删除文件
            break;
        } else {
            &dir.files[index]
        };
        file_before = if index_before >= file_before_num {
            // 剩下的新文件都为新增文件
            break;
        } else {
            &dir_before.files[index_before]
        };
        if file.name == file_before.name {
            if file.sha256 == file.sha256 {
                // 该文件未发生变化
                index += 1;
                index_before += 1;
            } else {
                todo!("该文件被修改");
            }
        } else if file.name < file_before.name {
            todo!("当前文件为新增文件");
            index += 1;
        } else {
            todo!("该历史文件为删除文件");
            index_before += 1;
        }
    }
    //todo
    Ok(())
}

pub async fn init_dir(data_path: impl AsRef<Path>) -> Result<Dir> {
    let data = tokio::fs::read(data_path).await?;
    Ok(serde_json::from_slice(data.as_slice())?)
}
