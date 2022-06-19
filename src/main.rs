#![allow(unused_imports)]
use anyhow::Result;
use directory_history::command::Commands;
use directory_history::{compare, record};
use log::error;
use log::LevelFilter::{Debug, Info};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout(Info);

    let cli = Commands::from_args();
    match cli {
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
