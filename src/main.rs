use clap::Parser;
use anyhow::Result;

mod progress;
mod cli;
mod core;
mod config;
mod reporters;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::execute_scan(&cli)?;
    Ok(())
}