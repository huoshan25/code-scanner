use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Comment {
    pub comment_type: String,
    pub content: String,
    pub file_path: String,
    pub line_number: usize,
    pub language: String,
}

pub struct Parser {
    patterns: Vec<(String, Vec<String>)>,
}

impl Parser {
    pub fn new(patterns: Vec<(String, Vec<String>)>) -> Self {
        Parser { patterns }
    }

    pub fn parse_file(&self, file_path: &Path) -> Result<Vec<Comment>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut comments = Vec::new();

        let language = self.detect_language(file_path);

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            for (comment_type, patterns) in &self.patterns {
                for pattern in patterns {
                    if let Some(content) = self.extract_comment(&line, pattern) {
                        comments.push(Comment {
                            comment_type: comment_type.clone(),
                            content,
                            file_path: file_path.to_string_lossy().to_string(),
                            line_number: line_number + 1,
                            language: language.clone(),
                        });
                    }
                }
            }
        }

        Ok(comments)
    }

    fn detect_language(&self, file_path: &Path) -> String {
        match file_path.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                "rs" => "Rust",
                "js" => "JavaScript",
                "ts" => "TypeScript",
                "py" => "Python",
                _ => "Unknown",
            }.to_string(),
            None => "Unknown".to_string(),
        }
    }

    fn extract_comment(&self, line: &str, pattern: &str) -> Option<String> {
        if let Some(idx) = line.find(pattern) {
            Some(line[idx + pattern.len()..].trim().to_string())
        } else {
            None
        }
    }
}