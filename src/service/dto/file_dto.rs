use crate::domain::entities::FileChange;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FileDto {
    pub path: String,
    pub status: String,
}

impl From<FileChange> for FileDto {
    fn from(file_change: FileChange) -> Self {
        Self {
            path: file_change.path().to_string_lossy(),
            status: file_change.status().as_str().to_string(),
        }
    }
}