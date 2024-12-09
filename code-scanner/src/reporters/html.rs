use crate::core::Comment;
use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult};
use serde_json::json;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct HtmlReporter;

impl HtmlReporter {
    pub fn generate_report(comments: &[Comment], output_path: &Path) -> Result<()> {
        let mut handlebars = Handlebars::new();

        // 注册 json helper
        handlebars.register_helper("json", Box::new(json_helper));

        // 注册模板
        handlebars.register_template_string("report", include_str!("../../templates/report.html"))?;

        // 准备数据
        let data = json!({
            "generated_at": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            "comments": comments,
            "statistics": {
                "total": comments.len(),
                "by_type": generate_type_statistics(comments),
                "by_language": generate_language_statistics(comments),
                "files": get_unique_files(comments).len(),
            }
        });

        // 渲染 HTML
        let html = handlebars.render("report", &data)?;

        // 写入文件
        fs::write(output_path, html)?;
        Ok(())
    }
}

// JSON helper 函数
fn json_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(&serde_json::to_string(param.value()).unwrap_or_default())?;
    Ok(())
}

fn generate_type_statistics(comments: &[Comment]) -> Vec<serde_json::Value> {
    let mut stats = std::collections::HashMap::new();
    for comment in comments {
        *stats.entry(&comment.comment_type).or_insert(0) += 1;
    }

    stats.iter()
        .map(|(k, v)| json!({
            "type": k,
            "count": v
        }))
        .collect()
}

fn generate_language_statistics(comments: &[Comment]) -> Vec<serde_json::Value> {
    let mut stats = std::collections::HashMap::new();
    for comment in comments {
        *stats.entry(&comment.language).or_insert(0) += 1;
    }

    stats.iter()
        .map(|(k, v)| json!({
            "language": k,
            "count": v
        }))
        .collect()
}

fn get_unique_files(comments: &[Comment]) -> Vec<String> {
    let mut files: std::collections::HashSet<String> = std::collections::HashSet::new();
    for comment in comments {
        files.insert(comment.file_path.clone());
    }
    files.into_iter().collect()
}