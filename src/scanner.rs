use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use anyhow::Result;
use crate::progress::ProgressBar;

pub struct Scanner {
    ignored_dirs: Vec<String>,
    file_types: Vec<String>,
}

impl Scanner {
    pub fn new(ignored_dirs: Vec<String>, file_types: Vec<String>) -> Self {
        Scanner {
            ignored_dirs,
            file_types,
        }
    }

    fn is_ignored(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        if path.is_dir() {
            self.ignored_dirs.iter().any(|dir| {
                path.to_string_lossy()
                    .to_string()
                    .contains(dir)
            })
        } else {
            false
        }
    }

    fn is_target_file(&self, entry: &DirEntry) -> bool {
        if let Some(ext) = entry.path().extension() {
            self.file_types.iter().any(|t| {
                t.trim_start_matches(".") == ext.to_string_lossy()
            })
        } else {
            false
        }
    }

    pub fn scan_directory(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        // 首先计算文件总数
        let total_files = WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e))
            .filter(|e| e.is_ok())
            .filter(|e| e.as_ref().unwrap().file_type().is_file())
            .filter(|e| self.is_target_file(e.as_ref().unwrap()))
            .count();

        let mut progress = ProgressBar::new(total_files);

        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e))
        {
            let entry = entry?;
            if entry.file_type().is_file() && self.is_target_file(&entry) {
                files.push(entry.path().to_owned());
                progress.increment();
            }
        }

        Ok(files)
    }
}