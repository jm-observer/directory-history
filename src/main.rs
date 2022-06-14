use anyhow::Result;
use clap::Parser;
use directory_history::command::{Cli, Commands};
use directory_history::{compare, record};
use log::error;
use log::LevelFilter::Info;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout(Info);

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
