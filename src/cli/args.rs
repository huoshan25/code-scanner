use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Scan code and generate report
    Scan {
        /// Directory to scan
        #[arg(default_value = ".")]
        directory: PathBuf,

        /// Configuration file path
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,

        /// Output format(md/html)
        #[arg(short = 'f', long, value_name = "FORMAT")]
        format: Option<String>,
    },

    /// Initialize default configuration file
    Init,
}