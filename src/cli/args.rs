// src/cli/arts.rs

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 要扫描的目录
    #[arg(default_value = ".")]
    pub directory: PathBuf,

    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// 输出文件路径
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// 输出格式(md/html)
    #[arg(short = 'f', long, value_name = "FORMAT")]
    pub format: Option<String>,
}