use std::{fs::{self, DirEntry}, io, path::PathBuf, time::SystemTime};

use crate::state::FileState;

pub struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: Option<u64>,
    modified: Option<SystemTime>,
    extension: Option<String>
}

pub enum {

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
        let mut entries = fs::read_dir(&self.path)? 
            .map(|res| res.and_then(Self::process_entry))
            .collect::<Result<Vec<_>, io::Error>>()?;

        self.files = entries;

        Ok(())
    }
}
