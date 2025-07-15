use std::path::PathBuf;

use crate::fs::FileItem;

pub struct App {
    pub file: FileState,
    pub ui: AppState
}

pub struct FileState {
    pub current_path: PathBuf,
    pub files: Vec<FileItem>
}

impl Default for FileState {
    fn default() -> Self {
        let path = std::fs::canonicalize(".").unwrap_or_else(|_| PathBuf::from("/"));
        Self { current_path: path, files: vec![] }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub selected: usize,
    pub exit: bool
}
