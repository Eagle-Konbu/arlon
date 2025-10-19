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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file_path() {
        let path = "src/main.rs".to_string();
        let file_path = FilePath::new(path.clone());
        assert!(file_path.is_ok());
        assert_eq!(file_path.unwrap().to_string_lossy(), path);
    }

    #[test]
    fn test_empty_file_path() {
        let result = FilePath::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FilePathError::Empty));
    }

    #[test]
    fn test_absolute_file_path() {
        let path = "/home/user/project/src/main.rs".to_string();
        let file_path = FilePath::new(path.clone());
        assert!(file_path.is_ok());
        assert_eq!(file_path.unwrap().to_string_lossy(), path);
    }

    #[test]
    fn test_file_path_display() {
        let path = "src/main.rs".to_string();
        let file_path = FilePath::new(path.clone()).unwrap();
        assert_eq!(format!("{}", file_path), path);
    }

    #[test]
    fn test_file_path_as_str() {
        let path = "src/main.rs".to_string();
        let file_path = FilePath::new(path.clone()).unwrap();
        assert_eq!(file_path.as_str(), Some(path.as_str()));
    }
}
