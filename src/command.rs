use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Commands {
    #[structopt(name = "record")]
    Record(Record),
    #[structopt(name = "cmp")]
    Compare(Compare),
}
#[derive(Debug, StructOpt)]
pub struct Record {
    #[structopt(short, help = "请输入需要统计的目录")]
    pub target_path: PathBuf,
    #[structopt(short, help = "请输入存储结果的文件")]
    pub record_name: String,
    #[structopt(short, help = "请输入需要排除统计的目录")]
    pub excludes: Option<String>,
}
#[derive(Debug, StructOpt)]
pub struct Compare {
    #[structopt(short, long, help = "请输入前统计结果")]
    pub before_record: PathBuf,
    #[structopt(short, long, help = "请输入后统计结果")]
    pub record: PathBuf,
}
