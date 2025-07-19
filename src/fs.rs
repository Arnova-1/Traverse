use std::{fs::{self, DirEntry}, io, path::PathBuf, time::SystemTime};

use chrono::DateTime;

use crate::state::FileState;

pub struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: Option<u64>,
    modified: Option<SystemTime>,
    extension: Option<String>
}

pub enum Navigate {
    Forward(PathBuf),
    Backward,
    Refresh
}

impl FileState {
    fn process_entry(entry: DirEntry) -> io::Result<FileItem> {
        let path = fs::canonicalize(entry.path())?;
        let metadata = fs::metadata(&path)?;

        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string();

        let is_dir = metadata.is_dir();
        let size = if is_dir {
            None
        } else {
            Some(metadata.len())
        };

        let modified = metadata.modified().ok();
        let extension = path.extension().and_then(|ext| ext.to_str()).map(String::from);

        Ok(FileItem {
            name,
            path,
            is_dir,
            size,
            modified,
            extension
        })
    }

    pub fn read_dir(&mut self) -> io::Result<()> {
        let entries = fs::read_dir(&self.current_path)? 
            .map(|res| res.and_then(Self::process_entry))
            .collect::<Result<Vec<_>, io::Error>>()?;

        self.files = entries;

        Ok(())
    }

    pub fn navigate(&mut self, nav: Navigate) -> io::Result<()> {
        match nav {
            Navigate::Forward(path) => {
                self.current_path = path;
            }
            Navigate::Backward => {
                self.current_path.pop();
            }
            Navigate::Refresh => {}
        }

        self.read_dir()?;
        Ok(())
    }

    pub fn formatted_lines(&self) -> Vec<String> {
        self.files.iter().map(|f| f.format_item()).collect()
    }
}

impl FileItem {
    pub fn format_item(&self) -> String {
        let icon = if self.is_dir { " " } else { "󰈙 " };
        let size = self.size.map(|s| format!("{:.1} KB", s as f64 / 1024.0)).unwrap_or("-".into());
        let date = self.modified.map(|t| {
            let datetime: DateTime<chrono::Local> = t.into();
            datetime.format("%Y-%m-%d %H:%M").to_string()
        }).unwrap_or("-".into());

        format!("{icon} {:<20} {:<6} {:<8} {}", self.name, if self.is_dir {"<DIR>"} else {"<FILE>"}, size, date)
    }
}
