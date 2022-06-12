pub mod collect;
pub mod command;
pub mod common;
pub mod compare;
pub mod ty;

use crate::collect::record_dir;
use crate::command::{Cli, Commands, Compare, Record};
use crate::compare::compare_dir;
use crate::ty::Dir;
use anyhow::Result;
use clap::Parser;
use log::error;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();

    let cli = Cli::parse();
    match cli.command {
        Commands::Record(re) => {
            if let Err(e) = record(re).await {
                error!("record error: {:?}", e);
            }
        }
        Commands::Compare(cp) => {
            if let Err(e) = compare(cp).await {
                error!("record error: {:?}", e);
            }
        }
    }
    Ok(())
}

async fn record(record: Record) -> Result<()> {
    let record_res = record_dir(record.target_path).await?;
    let data = serde_json::to_vec(&record_res)?;
    tokio::fs::write(".dir.json", data).await?;
    Ok(())
}
async fn compare(compare: Compare) -> Result<()> {
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
