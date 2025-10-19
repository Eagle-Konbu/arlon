use crate::application::dto::CommitDto;
use crate::domain::repositories::{GitRepository, GitRepositoryError};
use crate::domain::services::CommitComparisonDomainService;
use crate::domain::value_objects::{BranchName, BranchNameError};

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

        let head_commits = self.git_repository.get_commits_from_head()?;
        let branch_commits = self.git_repository.get_commits_from_branch(&branch)?;

        let commits =
            CommitComparisonDomainService::commits_not_in_branch(head_commits, branch_commits);

        Ok(commits.into_iter().map(CommitDto::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Commit;
    use crate::domain::repositories::{GitRepository, GitRepositoryError};
    use crate::domain::value_objects::{BranchName, CommitHash};
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        TestGitRepository {}

        impl GitRepository for TestGitRepository {
            fn get_commits_from_head(&self) -> Result<Vec<Commit>, GitRepositoryError>;
            fn get_commits_from_branch(&self, branch: &BranchName) -> Result<Vec<Commit>, GitRepositoryError>;

            fn get_file_changes_between_branches(
                &self,
                branch: &BranchName,
            ) -> Result<Vec<crate::domain::entities::FileChange>, GitRepositoryError>;
        }
    }

    fn create_test_commit() -> Commit {
        let hash = CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap();
        Commit::new(
            hash,
            "Test Author".to_string(),
            "test@example.com".to_string(),
            1634567890,
            "Test commit message".to_string(),
        )
    }

    #[test]
    fn test_execute_success() {
        let mut mock_repo = MockTestGitRepository::new();
        let test_commit = create_test_commit();
        let head_commits = vec![test_commit.clone()];
        let branch_commits = vec![];

        mock_repo
            .expect_get_commits_from_head()
            .times(1)
            .returning(move || Ok(head_commits.clone()));

        mock_repo
            .expect_get_commits_from_branch()
            .times(1)
            .returning(move |_| Ok(branch_commits.clone()));

        let use_case = CompareCommitsUseCase::new(&mock_repo);
        let result = use_case.execute("main".to_string());

        assert!(result.is_ok());
        let commits = result.unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].hash, "abcdef1234567890abcdef1234567890abcdef12");
        assert_eq!(commits[0].author, "Test Author");
        assert_eq!(commits[0].message, "Test commit message");
    }

    #[test]
    fn test_execute_invalid_branch_name() {
        let mock_repo = MockTestGitRepository::new();
        let use_case = CompareCommitsUseCase::new(&mock_repo);

        let result = use_case.execute("".to_string()); // Invalid empty branch name

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompareCommitsError::InvalidBranchName(_)
        ));
    }

    #[test]
    fn test_execute_repository_error() {
        let mut mock_repo = MockTestGitRepository::new();

        mock_repo.expect_get_commits_from_head().returning(|| {
            Err(GitRepositoryError::BranchNotFound {
                branch: "nonexistent".to_string(),
            })
        });

        let use_case = CompareCommitsUseCase::new(&mock_repo);
        let result = use_case.execute("nonexistent".to_string());

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompareCommitsError::RepositoryError(_)
        ));
    }

    #[test]
    fn test_execute_empty_commits() {
        let mut mock_repo = MockTestGitRepository::new();

        mock_repo
            .expect_get_commits_from_head()
            .returning(|| Ok(vec![]));

        mock_repo
            .expect_get_commits_from_branch()
            .returning(|_| Ok(vec![]));

        let use_case = CompareCommitsUseCase::new(&mock_repo);
        let result = use_case.execute("main".to_string());

        assert!(result.is_ok());
        let commits = result.unwrap();
        assert_eq!(commits.len(), 0);
    }
}
