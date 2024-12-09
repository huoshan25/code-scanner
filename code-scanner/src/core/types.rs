use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub comment_type: String,
    pub content: String,
    pub file_path: String,
    pub line_number: usize,
    pub language: String,
}