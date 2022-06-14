// pub mod buffer;
pub mod collect;
pub mod command;
pub mod common;
pub mod compare;
// pub mod compare_v2;
pub mod ty;

use crate::collect::record_dir;
use crate::command::{Compare, Record};
use crate::compare::compare_dir;
use crate::ty::Dir;
use anyhow::Result;
use log::info;

pub async fn record(record: Record) -> Result<()> {
    info!("record: {:?}", record);
    let record_res = record_dir(record.target_path).await?;
    let data = serde_json::to_vec(&record_res)?;
    tokio::fs::write(record.record_name, data).await?;
    Ok(())
}
pub async fn compare(compare: Compare) -> Result<()> {
    let data_before = tokio::fs::read(compare.before_record).await?;
    let data = tokio::fs::read(compare.record).await?;

    let dir_before: Dir = serde_json::from_slice(&data_before)?;
    let dir: Dir = serde_json::from_slice(&data)?;
    let mut compare = compare_dir(dir_before, dir).await;
    compare.sort();

    let data = serde_json::to_vec(&compare)?;
    tokio::fs::write("compare-result", data).await?;
    Ok(())
}
