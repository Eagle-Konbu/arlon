use crate::infra::cli::{Commands, OutputFormat};
use crate::infra::output::{JsonFormatter, OutputFormatter, SimpleFormatter};
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

pub struct CommandController {
    git_repository: GitRepositoryImpl,
}

impl CommandController {
    pub fn new() -> Result<Self, CommandError> {
        let git_repository = GitRepositoryImpl::open_current_dir()?;
        Ok(Self { git_repository })
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
