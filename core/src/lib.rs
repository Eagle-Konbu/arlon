pub mod application;
pub mod domain;
pub mod infra;

// Re-export commonly used types
pub use application::dto::{CommitDto, FileDto};
pub use application::use_cases::{CompareCommitsUseCase, CompareFilesUseCase};
pub use domain::entities::{Commit, FileChange};
pub use domain::repositories::{GitRepository, GitRepositoryError};
pub use domain::value_objects::{BranchName, CommitHash, FilePath};
pub use infra::output::{JsonFormatter, OutputFormatter, SimpleFormatter};
pub use infra::repositories::GitRepositoryImpl;