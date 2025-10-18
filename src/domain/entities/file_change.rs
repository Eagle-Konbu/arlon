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
