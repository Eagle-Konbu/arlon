use crate::domain::entities::{Commit, FileChange};
use crate::domain::value_objects::BranchName;

#[derive(Debug, thiserror::Error)]
pub enum GitRepositoryError {
    #[error("Repository not found or invalid")]
    RepositoryNotFound,
    #[error("Branch not found: {branch}")]
    BranchNotFound { branch: String },
    #[error("Git operation failed: {message}")]
    GitOperationFailed { message: String },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg_attr(test, mockall::automock)]
pub trait GitRepository {
    fn get_commits_from_head(&self) -> Result<Vec<Commit>, GitRepositoryError>;

    fn get_commits_from_branch(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<Commit>, GitRepositoryError>;

    fn get_file_changes_between_branches(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<FileChange>, GitRepositoryError>;
}
