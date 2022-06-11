use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn get_file_name(path: &PathBuf) -> Result<String> {
    Ok(path
        .file_name()
        .unwrap_or_else(|| path.as_os_str())
        .to_str()
        .ok_or(anyhow!("路径无法提取文件名：{:?}", path))?
        .to_string())
}
