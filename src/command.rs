use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Record(Record),
    Compare(Compare),
}
#[derive(Args, Debug)]
pub struct Record {
    pub target_path: PathBuf,
}
#[derive(Args, Debug)]
pub struct Compare {
    pub before_record: PathBuf,
    pub record: PathBuf,
}
