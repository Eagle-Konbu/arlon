use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::value_objects::{BranchName, BranchNameError};
use crate::application::dto::FileDto;

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
        let file_changes = self
            .git_repository
            .get_file_changes_between_branches(&branch)?;

        Ok(file_changes.into_iter().map(FileDto::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{FileChange, FileChangeStatus};
    use crate::domain::repositories::{GitRepository, GitRepositoryError};
    use crate::domain::value_objects::{BranchName, FilePath};
    use mockall::mock;

    mock! {
        TestGitRepository {}

        impl GitRepository for TestGitRepository {
            fn get_commits_from_head(&self) -> Result<Vec<crate::domain::entities::Commit>, GitRepositoryError>;
            fn get_commits_from_branch(&self, branch: &BranchName) -> Result<Vec<crate::domain::entities::Commit>, GitRepositoryError>;

            fn get_file_changes_between_branches(
                &self,
                branch: &BranchName,
            ) -> Result<Vec<FileChange>, GitRepositoryError>;
        }
    }

    fn create_test_file_change() -> FileChange {
        let file_path = FilePath::new("src/main.rs".to_string()).unwrap();
        FileChange::new(file_path, FileChangeStatus::Modified)
    }

    #[test]
    fn test_execute_success() {
        let mut mock_repo = MockTestGitRepository::new();
        let test_file_change = create_test_file_change();
        let expected_changes = vec![test_file_change];
        
        mock_repo
            .expect_get_file_changes_between_branches()
            .times(1)
            .returning(move |_| Ok(expected_changes.clone()));
        
        let use_case = CompareFilesUseCase::new(&mock_repo);
        let result = use_case.execute("main".to_string());
        
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "src/main.rs");
        assert_eq!(files[0].status, "modified");
    }

    #[test]
    fn test_execute_invalid_branch_name() {
        let mock_repo = MockTestGitRepository::new();
        let use_case = CompareFilesUseCase::new(&mock_repo);
        
        let result = use_case.execute("".to_string()); // Invalid empty branch name
        
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompareFilesError::InvalidBranchName(_)
        ));
    }

    #[test]
    fn test_execute_repository_error() {
        let mut mock_repo = MockTestGitRepository::new();
        
        mock_repo
            .expect_get_file_changes_between_branches()
            .returning(|_| Err(GitRepositoryError::BranchNotFound {
                branch: "nonexistent".to_string(),
            }));
        
        let use_case = CompareFilesUseCase::new(&mock_repo);
        let result = use_case.execute("nonexistent".to_string());
        
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompareFilesError::RepositoryError(_)
        ));
    }

    #[test]
    fn test_execute_empty_files() {
        let mut mock_repo = MockTestGitRepository::new();
        
        mock_repo
            .expect_get_file_changes_between_branches()
            .returning(|_| Ok(vec![]));
        
        let use_case = CompareFilesUseCase::new(&mock_repo);
        let result = use_case.execute("main".to_string());
        
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 0);
    }
}
