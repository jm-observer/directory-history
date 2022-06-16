use crate::common::{get_file_name, sha256};
use crate::ty::{Dir, DirBuilder, File};
use anyhow::{anyhow, bail, Result};
use async_recursion::async_recursion;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::read_dir;
use tokio::task::JoinHandle;

#[async_recursion]
pub async fn record_dir(
    path: PathBuf,
    excludes: Option<Arc<HashMap<String, u8>>>,
) -> Result<Option<Dir>> {
    if let Ok(mut read_dir) = read_dir(path.as_path()).await {
        let name = get_file_name(&path)?;
        if let Some(excludes_tmp) = excludes.as_ref() {
            if excludes_tmp.contains_key(&name) {
                return Ok(None);
            }
        }
        let mut dir = DirBuilder::new(name, path.clone());
        let mut dir_res = Vec::default();
        let mut sub_files = Vec::default();
        while let Ok(Some(sub_dir)) = read_dir.next_entry().await {
            if let Ok(metadata) = sub_dir.metadata().await {
                let file_path = sub_dir.path();
                if metadata.is_file() {
                    sub_files.push(tokio::spawn(record_file(file_path)));
                } else if metadata.is_dir() {
                    let tmp = excludes.clone();
                    dir_res.push(tokio::spawn(async move {
                        record_dir(sub_dir.path(), tmp).await
                    }));
                }
            } else {
                warn!("读取文件[{:?}]metadata失败", sub_dir.path());
            }
        }
        let dirs_handle = tokio::spawn(collect_sub_dirs(path.clone(), dir_res));
        let files_handle = tokio::spawn(collect_sub_files(path.clone(), sub_files));
        let files = files_handle.await?;
        dir.update_sub_files(files);

        let dirs = dirs_handle.await?;
        dir.update_sub_dirs(dirs);
        Ok(Some(dir.build()))
    } else {
        bail!("文件夹{:?}读取失败", path)
    }
}

async fn collect_sub_files(
    path: PathBuf,
    sub_file_handles: Vec<JoinHandle<Result<File>>>,
) -> Vec<File> {
    let mut sub_files = Vec::with_capacity(sub_file_handles.len());
    for sub_file_handle in sub_file_handles.into_iter() {
        match sub_file_handle.await {
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
    sub_files
}

async fn collect_sub_dirs(
    path: PathBuf,
    dir_res: Vec<JoinHandle<Result<Option<Dir>>>>,
) -> Vec<Dir> {
    let mut sub_dirs = Vec::with_capacity(dir_res.len());
    for sub_res in dir_res.into_iter() {
        match sub_res.await {
            Ok(res) => match res {
                Ok(Some(file)) => {
                    sub_dirs.push(file);
                }
                Err(e) => {
                    warn!("{:?}", e);
                }
                _ => {}
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
    sub_dirs
}

async fn record_file(file_path: PathBuf) -> Result<File> {
    match record_file_detail(&file_path).await {
        Ok(file) => Ok(file),
        Err(e) => {
            bail!("文件{:?}读取失败:{:?}", file_path, e);
        }
    }
}
async fn record_file_detail(file_path: &PathBuf) -> Result<File> {
    if let Ok(data) = tokio::fs::read(file_path.as_path()).await {
        let name = get_file_name(&file_path)?;
        let sha256 = sha256(&data);
        Ok(File::new(name, file_path, sha256))
    } else {
        Err(anyhow!("读取文件{:?}报错", file_path))
    }
}
