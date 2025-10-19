use crate::domain::value_objects::FilePath;

#[derive(Debug, Clone, PartialEq)]
pub struct FileChange {
    path: FilePath,
    status: FileChangeStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileChangeStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
    Ignored,
    Untracked,
    Typechange,
    Unreadable,
    Conflicted,
    Unmodified,
}

impl FileChange {
    pub fn new(path: FilePath, status: FileChangeStatus) -> Self {
        Self { path, status }
    }

    pub fn path(&self) -> &FilePath {
        &self.path
    }

    pub fn status(&self) -> &FileChangeStatus {
        &self.status
    }
}

impl FileChangeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Modified => "modified",
            Self::Renamed => "renamed",
            Self::Copied => "copied",
            Self::Ignored => "ignored",
            Self::Untracked => "untracked",
            Self::Typechange => "typechange",
            Self::Unreadable => "unreadable",
            Self::Conflicted => "conflicted",
            Self::Unmodified => "unmodified",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::FilePath;

    fn create_test_file_change() -> FileChange {
        let file_path = FilePath::new("src/main.rs".to_string()).unwrap();
        FileChange::new(file_path, FileChangeStatus::Modified)
    }

    #[test]
    fn test_file_change_creation() {
        let file_change = create_test_file_change();
        assert_eq!(file_change.path().to_string_lossy(), "src/main.rs");
        assert_eq!(file_change.status().as_str(), "modified");
    }

    #[test]
    fn test_file_change_status_variants() {
        let test_cases = vec![
            (FileChangeStatus::Added, "added"),
            (FileChangeStatus::Deleted, "deleted"),
            (FileChangeStatus::Modified, "modified"),
            (FileChangeStatus::Renamed, "renamed"),
            (FileChangeStatus::Copied, "copied"),
            (FileChangeStatus::Ignored, "ignored"),
            (FileChangeStatus::Untracked, "untracked"),
            (FileChangeStatus::Typechange, "typechange"),
            (FileChangeStatus::Unreadable, "unreadable"),
            (FileChangeStatus::Conflicted, "conflicted"),
            (FileChangeStatus::Unmodified, "unmodified"),
        ];

        for (status, expected_str) in test_cases {
            assert_eq!(status.as_str(), expected_str);
        }
    }

    #[test]
    fn test_file_change_equality() {
        let file_change1 = create_test_file_change();
        let file_change2 = create_test_file_change();
        assert_eq!(file_change1, file_change2);
    }

    #[test]
    fn test_file_change_clone() {
        let file_change1 = create_test_file_change();
        let file_change2 = file_change1.clone();
        assert_eq!(file_change1, file_change2);
    }
}
