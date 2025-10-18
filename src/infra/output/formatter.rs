use crate::service::dto::{CommitDto, FileDto};

pub trait OutputFormatter {
    fn format_commits(&self, commits: &[CommitDto]) -> Result<(), String>;
    fn format_files(&self, files: &[FileDto]) -> Result<(), String>;
}