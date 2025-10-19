use crate::infra::cli::{Commands, OutputFormat};
use crate::infra::output::{JsonFormatter, OutputFormatter, SimpleFormatter};
use crate::domain::repositories::GitRepository;
use crate::infra::repositories::GitRepositoryImpl;
use crate::application::use_cases::{CompareCommitsUseCase, CompareFilesUseCase};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Compare commits failed: {0}")]
    CompareCommitsError(#[from] crate::application::use_cases::compare_commits::CompareCommitsError),
    #[error("Compare files failed: {0}")]
    CompareFilesError(#[from] crate::application::use_cases::compare_files::CompareFilesError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::domain::repositories::GitRepositoryError),
    #[error("Output error: {0}")]
    OutputError(String),
}

pub struct CommandController<R> {
    git_repository: R,
}

impl<R: GitRepository> CommandController<R> {
    pub fn new(git_repository: R) -> Self {
        Self { git_repository }
    }

    pub fn execute(&self, command: Commands) -> Result<(), CommandError> {
        match command {
            Commands::Commits { branch, format } => self.handle_compare_commits(branch, format),
            Commands::Files { branch, format } => self.handle_compare_files(branch, format),
        }
    }

    fn handle_compare_commits(
        &self,
        branch: String,
        format: OutputFormat,
    ) -> Result<(), CommandError> {
        let use_case = CompareCommitsUseCase::new(&self.git_repository);
        let commits = use_case.execute(branch)?;

        match format {
            OutputFormat::Simple => {
                let formatter = SimpleFormatter;
                formatter
                    .format_commits(&commits)
                    .map_err(CommandError::OutputError)?;
            }
            OutputFormat::Json => {
                let formatter = JsonFormatter;
                formatter
                    .format_commits(&commits)
                    .map_err(CommandError::OutputError)?;
            }
        }

        Ok(())
    }

    fn handle_compare_files(
        &self,
        branch: String,
        format: OutputFormat,
    ) -> Result<(), CommandError> {
        let use_case = CompareFilesUseCase::new(&self.git_repository);
        let files = use_case.execute(branch)?;

        match format {
            OutputFormat::Simple => {
                let formatter = SimpleFormatter;
                formatter
                    .format_files(&files)
                    .map_err(CommandError::OutputError)?;
            }
            OutputFormat::Json => {
                let formatter = JsonFormatter;
                formatter
                    .format_files(&files)
                    .map_err(CommandError::OutputError)?;
            }
        }

        Ok(())
    }
}

impl CommandController<GitRepositoryImpl> {
    pub fn new_with_current_dir() -> Result<Self, CommandError> {
        let git_repository = GitRepositoryImpl::open_current_dir()?;
        Ok(Self::new(git_repository))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::{GitRepository, GitRepositoryError};
    use crate::domain::entities::{Commit, FileChange};
    use crate::domain::value_objects::BranchName;
    use crate::infra::cli::OutputFormat;
    use mockall::mock;

    mock! {
        TestGitRepository {}
        impl GitRepository for TestGitRepository {
            fn get_commits_from_head(&self) -> Result<Vec<Commit>, GitRepositoryError>;
            fn get_commits_from_branch(&self, branch: &BranchName) -> Result<Vec<Commit>, GitRepositoryError>;
            fn get_file_changes_between_branches(&self, branch: &BranchName) -> Result<Vec<FileChange>, GitRepositoryError>;
        }
    }

    #[test]
    fn test_command_controller_with_mock_repository() {
        let mut mock_repo = MockTestGitRepository::new();
        
        // Mock the repository behavior
        mock_repo
            .expect_get_commits_from_head()
            .returning(|| Ok(vec![]));
        
        mock_repo
            .expect_get_commits_from_branch()
            .returning(|_| Ok(vec![]));
        
        let controller = CommandController::new(mock_repo);
        let commands = Commands::Commits {
            branch: "main".to_string(),
            format: OutputFormat::Simple,
        };
        
        // This should not panic - demonstrates improved testability
        let result = controller.execute(commands);
        assert!(result.is_ok());
    }
}
