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
            path: file_change.path().to_string(),
            status: file_change.status().as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FileChangeStatus;
    use crate::domain::value_objects::FilePath;

    fn create_test_file_change() -> FileChange {
        let file_path = FilePath::new("src/main.rs".to_string()).unwrap();
        FileChange::new(file_path, FileChangeStatus::Modified)
    }

    #[test]
    fn test_file_dto_from_file_change() {
        let file_change = create_test_file_change();
        let dto = FileDto::from(file_change);

        assert_eq!(dto.path, "src/main.rs");
        assert_eq!(dto.status, "modified");
    }

    #[test]
    fn test_file_dto_serialization() {
        let dto = FileDto {
            path: "src/main.rs".to_string(),
            status: "modified".to_string(),
        };

        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("src/main.rs"));
        assert!(json.contains("modified"));
    }

    #[test]
    fn test_file_dto_different_statuses() {
        let test_cases = vec![
            (FileChangeStatus::Added, "added"),
            (FileChangeStatus::Deleted, "deleted"),
            (FileChangeStatus::Modified, "modified"),
        ];

        for (status, expected_str) in test_cases {
            let file_path = FilePath::new("test.txt".to_string()).unwrap();
            let file_change = FileChange::new(file_path, status);
            let dto = FileDto::from(file_change);

            assert_eq!(dto.status, expected_str);
            assert_eq!(dto.path, "test.txt");
        }
    }
}
