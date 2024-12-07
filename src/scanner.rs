use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use anyhow::Result;
use rayon::prelude::*;
use crate::progress::ProgressBar;
use std::sync::Mutex;

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
        // 首先收集所有符合条件的文件条目
        let entries: Vec<_> = WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && self.is_target_file(e))
            .collect();

        let total_files = entries.len();
        let progress = Mutex::new(ProgressBar::new(total_files));

        // 并行处理文件
        let files: Vec<PathBuf> = entries.par_iter()
            .map(|entry| {
                let path = entry.path().to_owned();
                if let Ok(mut progress_guard) = progress.lock() {
                    progress_guard.increment();
                }
                path
            })
            .collect();

        Ok(files)
    }
}