use crate::parser::Comment;
use chrono::Local;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct MarkdownReporter;

impl MarkdownReporter {
    pub fn generate_report(comments: &[Comment], output_path: &Path) -> Result<()> {
        let mut content = String::new();

        // 添加报告头部
        content.push_str("# Code Review Report\n");
        content.push_str(&format!("Generated at: {}\n\n",
                                  Local::now().format("%Y-%m-%d %H:%M:%S")));

        // 生成统计摘要
        let stats = self::generate_statistics(comments);
        content.push_str("## Summary\n");
        for (type_name, count) in &stats.comment_counts {
            content.push_str(&format!("- Total {}: {}\n", type_name, count));
        }
        content.push_str(&format!("- Files scanned: {}\n", stats.file_count));
        content.push_str(&format!("- Languages: {}\n\n",
                                  stats.languages.join(", ")));

        // 按注释类型分组
        let grouped = self::group_comments_by_type(comments);
        for (comment_type, type_comments) in grouped {
            content.push_str(&format!("## {}\n", comment_type));

            // 按文件分组
            let file_grouped = self::group_comments_by_file(&type_comments);
            for (file, file_comments) in file_grouped {
                content.push_str(&format!("### {}\n", file));
                for comment in file_comments {
                    content.push_str(&format!("- Line {}: {}\n",
                                              comment.line_number, comment.content));
                }
                content.push('\n');
            }
        }

        // 写入文件
        fs::write(output_path, content)?;
        Ok(())
    }
}

// 统计信息结构
struct Statistics {
    comment_counts: HashMap<String, usize>,
    file_count: usize,
    languages: Vec<String>,
}

// 生成统计信息
fn generate_statistics(comments: &[Comment]) -> Statistics {
    let mut comment_counts = HashMap::new();
    let mut files = std::collections::HashSet::new();
    let mut languages = std::collections::HashSet::new();

    for comment in comments {
        *comment_counts.entry(comment.comment_type.clone())
            .or_insert(0) += 1;
        files.insert(comment.file_path.clone());
        languages.insert(comment.language.clone());
    }

    Statistics {
        comment_counts,
        file_count: files.len(),
        languages: languages.into_iter().collect(),
    }
}

// 按注释类型分组
fn group_comments_by_type(comments: &[Comment]) -> HashMap<String, Vec<&Comment>> {
    let mut grouped = HashMap::new();
    for comment in comments {
        grouped.entry(comment.comment_type.clone())
            .or_insert_with(Vec::new)
            .push(comment);
    }
    grouped
}

// 按文件分组，添加生命周期参数 'a
fn group_comments_by_file<'a>(comments: &[&'a Comment]) -> HashMap<String, Vec<&'a Comment>> {
    let mut grouped = HashMap::new();
    for &comment in comments {
        grouped.entry(comment.file_path.clone())
            .or_insert_with(Vec::new)
            .push(comment);
    }
    grouped
}