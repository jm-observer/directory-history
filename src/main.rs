use anyhow::{anyhow, bail, Context, Result};
use async_recursion::async_recursion;
use hex_literal::hex;
use log::warn;
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::path::PathBuf;
use tokio::fs::read_dir;

fn main() {
    let data = std::fs::read("./data").unwrap();
    // create a Sha256 object
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(&data);
    // read hash digest and consume hasher
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("8b81b41df79474f2d6de976921c455186d8a4e3213f6fd6e79d8021faccb7cd0")[..]
    );
}

#[async_recursion]
async fn init_dir_v2(path: PathBuf) -> Result<Dir> {
    if path.is_dir() {
        let mut read_dir = read_dir(path.as_path())
            .await
            .context(anyhow!("读取文件夹[{:?}]失败", path.as_path()))?;
        let name = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_os_string();
        let mut dir = Dir::new(name, path.clone());
        let mut dir_res = Vec::default();
        while let Ok(Some(sub_dir)) = read_dir.next_entry().await {
            if let Ok(metadata) = sub_dir.metadata().await {
                if metadata.is_file() {
                } else if metadata.is_dir() {
                    dir_res.push(tokio::spawn(
                        async move { init_dir_v2(sub_dir.path()).await },
                    ));
                }
            } else {
                warn!("读取文件[{:?}]metadata失败", sub_dir.path());
            }
        }
        for sub_res in dir_res.into_iter() {
            match sub_res.await.context(anyhow!("等待异常"))? {
                Ok(res) => {
                    todo!()
                }
                Err(e) => {
                    warn!("{:?}", e);
                }
            }
        }
        Ok(dir)
    } else {
        bail!("文件属性有误或无权限: {:?}", path);
    }
}

#[derive(Debug)]
struct File {
    name: OsString,
    path: PathBuf,
    sha256: String,
}
#[derive(Debug)]
struct Dir {
    name: OsString,
    path: PathBuf,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl File {
    pub fn new(name: impl Into<OsString>, path: impl Into<PathBuf>, size: u64) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}
impl Dir {
    pub fn new(name: impl Into<OsString>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            files: Vec::default(),
            dirs: Vec::default(),
        }
    }
}
