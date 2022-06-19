pub mod collect;
pub mod command;
pub mod common;
pub mod compare;
pub mod ty;

use crate::collect::record_dir;
use crate::command::{Compare, Record};
use crate::compare::compare_dir;
use crate::ty::Dir;
use anyhow::Result;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn record(record: Record) -> Result<()> {
    info!("record: {:?}", record);
    let excludes: Option<Arc<HashMap<String, u8>>> = if let Some(excludes) = record.excludes {
        let tmp: Vec<&str> = excludes.split(",").collect();
        debug!("excludes: {:?}", tmp);
        let tmp: HashMap<String, u8> = tmp.into_iter().map(|x| (x.to_string(), 1)).collect();
        Some(Arc::new(tmp))
    } else {
        None
    };
    let target_path = record.target_path.canonicalize()?;
    if let Some(record_res) = record_dir(target_path, excludes).await? {
        info!(
            "success dirs(包含忽略): {}, fail dirs: {}",
            record_res.success_dirs, record_res.fail_dirs
        );
        info!(
            "success files: {}, fail files: {}",
            record_res.success_files, record_res.fail_files
        );
        let data = serde_json::to_vec(&record_res)?;
        tokio::fs::write(record.record_name, data).await?;
    } else {
        warn!("record_dir none");
    }
    Ok(())
}
pub async fn compare(compare: Compare) -> Result<()> {
    let data_before = tokio::fs::read(compare.before_record).await?;
    let data = tokio::fs::read(compare.record).await?;

    let dir_before: Dir = serde_json::from_slice(&data_before)?;
    let dir: Dir = serde_json::from_slice(&data)?;
    let mut compare = compare_dir(dir_before, dir).await;
    compare.sort();

    debug!("{:?}", compare);
    let data = serde_json::to_vec(&compare)?;
    tokio::fs::write("compare-result", data).await?;
    Ok(())
}
