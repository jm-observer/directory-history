pub mod collect;
pub mod command;
pub mod common;
pub mod compare;
pub mod ty;

use crate::collect::init_dir;
use anyhow::Result;
use log::{debug, warn};

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();
    let dir = std::env::args().nth(1).unwrap_or("c:".to_string());
    debug!("start to collect directory: {}", dir);
    match init_dir(dir.parse()?).await {
        Ok(dir_res) => {
            let dir_str = serde_json::to_string(&dir_res)?;
            tokio::fs::write(".dir.json", dir_str).await?;
            Ok(())
        }
        Err(e) => {
            warn!("{:?}", e);
            Err(e)
        }
    }
}
