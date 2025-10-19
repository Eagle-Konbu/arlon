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
    /// 指定されたブランチに存在しないコミットを取得する
    fn get_commits_not_in_branch(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<Commit>, GitRepositoryError>;

    /// 指定されたブランチとの間でファイルの差分を取得する
    fn get_file_changes_between_branches(
        &self,
        branch: &BranchName,
    ) -> Result<Vec<FileChange>, GitRepositoryError>;
}
