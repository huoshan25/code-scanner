#![deny(clippy::all)]

use clap::Parser;
use napi_derive::napi;

mod progress;
mod cli;
mod core;
mod config;
mod reporters;

#[napi]
pub fn scan_code() -> napi::Result<()> {
  // 使用原来的命令行解析逻辑
  let cli = cli::Cli::parse();

  // 执行扫描，将 anyhow::Result 转换为 napi::Result
  cli::execute_scan(&cli)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

  Ok(())
}