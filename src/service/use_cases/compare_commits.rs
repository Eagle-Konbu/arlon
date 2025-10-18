use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::value_objects::{BranchName, BranchNameError};
use crate::service::dto::CommitDto;

#[derive(Debug, thiserror::Error)]
pub enum CompareCommitsError {
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(#[from] BranchNameError),
    #[error("Git repository error: {0}")]
    RepositoryError(#[from] GitRepositoryError),
}

pub struct CompareCommitsUseCase<'a, R> {
    git_repository: &'a R,
}

impl<'a, R: GitRepository> CompareCommitsUseCase<'a, R> {
    pub fn new(git_repository: &'a R) -> Self {
        Self { git_repository }
    }
    
    pub fn execute(&self, branch_name: String) -> Result<Vec<CommitDto>, CompareCommitsError> {
        let branch = BranchName::new(branch_name)?;
        let commits = self.git_repository.get_commits_not_in_branch(&branch)?;
        
        Ok(commits.into_iter().map(CommitDto::from).collect())
    }
}