use crate::infra::output::OutputFormatter;
use crate::service::dto::{CommitDto, FileDto};

pub struct SimpleFormatter;

impl OutputFormatter for SimpleFormatter {
    fn format_commits(&self, commits: &[CommitDto]) -> Result<(), String> {
        for commit in commits {
            println!("{} {} {}", commit.hash, commit.date, commit.message);
        }
        Ok(())
    }
    
    fn format_files(&self, files: &[FileDto]) -> Result<(), String> {
        for file in files {
            println!("{} {}", file.status, file.path);
        }
        Ok(())
    }
}