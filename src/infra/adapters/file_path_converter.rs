use crate::domain::value_objects::FilePath;
use std::path::PathBuf;

pub struct FilePathConverter;

impl FilePathConverter {
    pub fn to_path_buf(file_path: &FilePath) -> PathBuf {
        PathBuf::from(file_path.as_str())
    }

    pub fn to_string_lossy(file_path: &FilePath) -> String {
        let path_buf = Self::to_path_buf(file_path);
        path_buf.to_string_lossy().to_string()
    }

    pub fn from_path_buf(path_buf: &std::path::Path) -> Result<FilePath, crate::domain::value_objects::FilePathError> {
        let path_str = path_buf.to_string_lossy().to_string();
        FilePath::new(path_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_path_buf() {
        let file_path = FilePath::new("src/main.rs".to_string()).unwrap();
        let path_buf = FilePathConverter::to_path_buf(&file_path);
        assert_eq!(path_buf.to_string_lossy(), "src/main.rs");
    }

    #[test]
    fn test_to_string_lossy() {
        let file_path = FilePath::new("src/main.rs".to_string()).unwrap();
        let lossy_string = FilePathConverter::to_string_lossy(&file_path);
        assert_eq!(lossy_string, "src/main.rs");
    }

    #[test]
    fn test_from_path_buf() {
        let path_buf = PathBuf::from("src/main.rs");
        let file_path = FilePathConverter::from_path_buf(&path_buf).unwrap();
        assert_eq!(file_path.as_str(), "src/main.rs");
    }
}