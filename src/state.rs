use std::path::PathBuf;

pub struct App {
    pub file: FileState,
    pub ui: AppState
}

pub struct FileState {
    pub path: PathBuf,
    pub files: Vec<String>
}

impl Default for FileState {
    fn default() -> Self {
        Self { path: PathBuf::from("."), files: vec![] }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub selected: usize,
    pub exit: bool
}
