use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentPattern {
    pub type_name: String,
    pub patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,  // "md" 或 "html"
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_ignore")]
    pub ignore: Vec<String>,

    #[serde(default = "default_file_types")]
    pub file_types: Vec<String>,

    #[serde(default = "default_comments")]
    pub comments: Vec<CommentPattern>,

    #[serde(default = "default_output")]
    pub output: OutputConfig,
}

// 默认配置
fn default_ignore() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "target".to_string(),
        "dist".to_string(),
    ]
}

fn default_file_types() -> Vec<String> {
    vec![
        ".rs".to_string(),
        ".js".to_string(),
        ".ts".to_string(),
        ".py".to_string(),
    ]
}

fn default_comments() -> Vec<CommentPattern> {
    vec![
        CommentPattern {
            type_name: "TODO".to_string(),
            patterns: vec!["TODO:".to_string(), "TODO(".to_string()],
        },
        CommentPattern {
            type_name: "FIXME".to_string(),
            patterns: vec!["FIXME:".to_string(), "FIX:".to_string()],
        },
        CommentPattern {
            type_name: "NOTE".to_string(),
            patterns: vec!["NOTE:".to_string()],
        },
        CommentPattern {
            type_name: "HACK".to_string(),
            patterns: vec!["HACK:".to_string()],
        },
    ]
}

fn default_output() -> OutputConfig {
    OutputConfig {
        format: "html".to_string(),
        path: "./report".to_string(),
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ignore: default_ignore(),
            file_types: default_file_types(),
            comments: default_comments(),
            output: default_output(),
        }
    }
}

impl Config {
    // 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    // 合并命令行参数
    pub fn merge_cli_args(&mut self, cli_output_format: Option<String>, cli_output_path: Option<String>) {
        if let Some(format) = cli_output_format {
            self.output.format = format;
        }
        if let Some(path) = cli_output_path {
            self.output.path = path;
        }
    }

    // 验证配置
    pub fn validate(&self) -> Result<()> {
        // 确保输出格式是有效的
        if !["md", "html"].contains(&self.output.format.as_str()) {
            anyhow::bail!("Invalid output format. Must be 'md' or 'html'");
        }

        // 确保至少有一个文件类型
        if self.file_types.is_empty() {
            anyhow::bail!("At least one file type must be specified");
        }

        // 确保至少有一个注释类型
        if self.comments.is_empty() {
            anyhow::bail!("At least one comment pattern must be specified");
        }

        Ok(())
    }
}