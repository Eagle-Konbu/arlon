use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath(PathBuf);

#[derive(Debug, thiserror::Error)]
pub enum FilePathError {
    #[error("File path cannot be empty")]
    Empty,
    #[error("Invalid file path: {0}")]
    Invalid(String),
}

impl FilePath {
    pub fn new(path: String) -> Result<Self, FilePathError> {
        if path.is_empty() {
            return Err(FilePathError::Empty);
        }
        
        let path_buf = PathBuf::from(path);
        Ok(Self(path_buf))
    }
    
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
    
    pub fn as_str(&self) -> Option<&str> {
        self.0.to_str()
    }
    
    pub fn to_string_lossy(&self) -> String {
        self.0.to_string_lossy().to_string()
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<FilePath> for PathBuf {
    fn from(file_path: FilePath) -> Self {
        file_path.0
    }
}