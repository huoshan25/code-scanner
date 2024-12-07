use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

mod scanner;
mod parser;
mod config;
mod markdown;
mod html;
mod progress;

use config::Config;
use scanner::Scanner;
use parser::Parser as CommentParser;
use markdown::MarkdownReporter;
use html::HtmlReporter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to scan
    #[arg(default_value = ".")]
    directory: PathBuf,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Output format (md/html)
    #[arg(short = 'f', long, value_name = "FORMAT")]
    format: Option<String>,
}

fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 加载配置
    let mut config = match cli.config {
        Some(ref path) => Config::from_file(path)?,
        None => Config::default(),
    };

    // 合并命令行参数到配置
    config.merge_cli_args(cli.format, cli.output);
    config.validate()?;

    // 初始化扫描器
    let scanner = Scanner::new(config.ignore.clone(), config.file_types.clone());

    // 扫描文件
    println!("Scanning directory: {}", cli.directory.display());
    let files = scanner.scan_directory(&cli.directory)?;
    println!("Found {} files to analyze", files.len());

    // 初始化解析器
    let patterns: Vec<(String, Vec<String>)> = config.comments
        .iter()
        .map(|c| (c.type_name.clone(), c.patterns.clone()))
        .collect();
    let parser = CommentParser::new(patterns);

    // 解析所有文件
    println!("Parsing files...");
    let mut all_comments = Vec::new();
    for file in files {
        match parser.parse_file(&file) {
            Ok(comments) => all_comments.extend(comments),
            Err(err) => eprintln!("Error parsing file {}: {}", file.display(), err),
        }
    }
    println!("Found {} comments", all_comments.len());

    // 生成报告
    let output_path = PathBuf::from(&config.output.path);
    match config.output.format.as_str() {
        "md" => {
            println!("Generating Markdown report...");
            MarkdownReporter::generate_report(&all_comments, &output_path)?;
        }
        "html" => {
            println!("Generating HTML report...");
            HtmlReporter::generate_report(&all_comments, &output_path)?;
        }
        _ => anyhow::bail!("Unsupported output format"),
    }

    println!("Report generated at: {}", output_path.display());
    Ok(())
}