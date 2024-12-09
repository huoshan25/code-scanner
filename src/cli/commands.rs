use anyhow::Result;
use std::path::PathBuf;
use std::fs;

use crate::config::Config;
use crate::core::scanner::Scanner;
use crate::core::parser::Parser as CommentParser;
use crate::reporters::{MarkdownReporter, HtmlReporter};
use crate::core::Comment;

const DEFAULT_CONFIG: &str = include_str!("../../templates/default-config.yaml");

pub fn execute_scan(cli: &super::args::Cli) -> Result<()> {
    match &cli.command {
        super::args::Commands::Scan { directory, config, format, output } => {
            let mut config = match config {
                Some(path) => Config::from_file(path)?,
                None => {
                    // 尝试从默认路径加载
                    let default_path = PathBuf::from("code-scan.yaml");
                    if default_path.exists() {
                        Config::from_file(default_path)?
                    } else {
                        Config::default()
                    }
                }
            };

            // 合并命令行参数到配置
            config.merge_cli_args(format.clone(), output.clone());
            config.validate()?;

            // 执行扫描
            let comments = scan_files(&config, directory)?;

            // 生成报告
            generate_report(&config, &comments)?;
        },
        super::args::Commands::Init => {
            // 初始化配置文件
            initialize_config()?;
        }
    }

    Ok(())
}

fn initialize_config() -> Result<()> {
    let config_path = PathBuf::from("code-scan.yaml");
    if config_path.exists() {
        println!("Configuration file already exists at: {}", config_path.display());
        return Ok(());
    }

    fs::write(&config_path, DEFAULT_CONFIG)?;
    println!("Created default configuration file at: {}", config_path.display());
    Ok(())
}

fn scan_files(config: &Config, directory: &PathBuf) -> Result<Vec<Comment>> {
    // 初始化扫描器
    let scanner = Scanner::new(config.ignore.clone(), config.file_types.clone());

    // 扫描文件
    println!("Scanning directory: {}", directory.display());
    let files = scanner.scan_directory(directory)?;
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

    Ok(all_comments)
}

fn generate_report(config: &Config, comments: &[Comment]) -> Result<()> {
    let output_path = PathBuf::from(&config.output.path);
    match config.output.format.as_str() {
        "md" => {
            println!("Generating Markdown report...");
            MarkdownReporter::generate_report(comments, &output_path)?;
        }
        "html" => {
            println!("Generating HTML report...");
            HtmlReporter::generate_report(comments, &output_path)?;
        }
        _ => anyhow::bail!("Unsupported output format"),
    }

    println!("Report generated at: {}", output_path.display());
    Ok(())
}