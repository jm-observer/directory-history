use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};

use std::path::PathBuf;

pub fn get_file_name(path: &PathBuf) -> Result<String> {
    Ok(path
        .file_name()
        .unwrap_or_else(|| path.as_os_str())
        .to_str()
        .ok_or(anyhow!("路径无法提取文件名：{:?}", path))?
        .to_string())
}

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result: Vec<u8> = hasher.finalize().to_vec();
    hex::encode(result)
}
