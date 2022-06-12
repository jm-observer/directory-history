use crate::common::get_file_name;
use crate::ty::{Dir, File};
use anyhow::{anyhow, bail, Context, Result};
use async_recursion::async_recursion;
use log::{debug, warn};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs::read_dir;
use tokio::task::JoinHandle;

#[async_recursion]
pub async fn record_dir(path: PathBuf) -> Result<Dir> {
    if path.is_dir() {
        let mut read_dir = read_dir(path.as_path())
            .await
            .context(anyhow!("读取文件夹[{:?}]失败", path.as_path()))?;
        let name = get_file_name(&path)?;
        let mut dir = Dir::new(name, path.clone());
        let mut dir_res = Vec::default();
        let mut sub_files = Vec::default();
        while let Ok(Some(sub_dir)) = read_dir.next_entry().await {
            if let Ok(metadata) = sub_dir.metadata().await {
                let file_path = sub_dir.path();
                if metadata.is_file() {
                    sub_files.push(tokio::spawn(record_file(file_path)));
                } else if metadata.is_dir() {
                    dir_res.push(tokio::spawn(
                        async move { record_dir(sub_dir.path()).await },
                    ));
                }
            } else {
                warn!("读取文件[{:?}]metadata失败", sub_dir.path());
            }
        }
        let dirs_handle = tokio::spawn(collect_sub_dirs(path.clone(), dir_res));
        let files_handle = tokio::spawn(collect_sub_files(path.clone(), sub_files));
        let files = files_handle.await??;
        dir.update_sub_files(files);

        let dirs = dirs_handle.await??;
        dir.update_sub_dirs(dirs);

        Ok(dir)
    } else {
        warn!("文件属性有误或无权限: {:?}", path,);
        bail!("文件属性有误或无权限: {:?}", path);
    }
}

pub async fn collect_sub_files(
    path: PathBuf,
    sub_file_handles: Vec<JoinHandle<Result<File>>>,
) -> Result<Vec<File>> {
    let mut sub_files = Vec::with_capacity(sub_file_handles.len());
    for sub_file_handle in sub_file_handles.into_iter() {
        match sub_file_handle.await.context(anyhow!("等待异常")) {
            Ok(res) => match res {
                Ok(file) => {
                    sub_files.push(file);
                }
                Err(e) => {
                    warn!("{:?}", e);
                }
            },
            Err(e) => {
                warn!("{:?}", e);
            }
        }
    }
    debug!(
        "collect_sub_files {:?} {}/{} files",
        path,
        sub_files.len(),
        sub_files.capacity()
    );
    Ok(sub_files)
}

pub async fn collect_sub_dirs(
    path: PathBuf,
    dir_res: Vec<JoinHandle<Result<Dir>>>,
) -> Result<Vec<Dir>> {
    let mut sub_dirs = Vec::with_capacity(dir_res.len());
    for sub_res in dir_res.into_iter() {
        match sub_res.await.context(anyhow!("等待异常")) {
            Ok(res) => match res {
                Ok(file) => {
                    sub_dirs.push(file);
                }
                Err(e) => {
                    warn!("{:?}", e);
                }
            },
            Err(e) => {
                warn!("{:?}", e);
            }
        }
    }
    debug!(
        "collect_sub_dirs {:?} {}/{} dirs",
        path,
        sub_dirs.len(),
        sub_dirs.capacity()
    );
    Ok(sub_dirs)
}

pub async fn record_file(file_path: PathBuf) -> Result<File> {
    let data = tokio::fs::read(file_path.as_path()).await?;
    let name = get_file_name(&file_path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result: Vec<u8> = hasher.finalize().to_vec();
    let sha256 = hex::encode(result);
    Ok(File::new(name, file_path, sha256))
}
