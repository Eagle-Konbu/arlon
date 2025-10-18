use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::value_objects::{BranchName, BranchNameError};
use crate::service::dto::FileDto;

#[derive(Debug, thiserror::Error)]
pub enum CompareFilesError {
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(#[from] BranchNameError),
    #[error("Git repository error: {0}")]
    RepositoryError(#[from] GitRepositoryError),
}

pub struct CompareFilesUseCase<'a, R> {
    git_repository: &'a R,
}

impl<'a, R: GitRepository> CompareFilesUseCase<'a, R> {
    pub fn new(git_repository: &'a R) -> Self {
        Self { git_repository }
    }
    
    pub fn execute(&self, branch_name: String) -> Result<Vec<FileDto>, CompareFilesError> {
        let branch = BranchName::new(branch_name)?;
        let file_changes = self.git_repository.get_file_changes_between_branches(&branch)?;
        
        Ok(file_changes.into_iter().map(FileDto::from).collect())
    }
}