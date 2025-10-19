use crate::application::dto::{CommitDto, FileDto};
use crate::infra::output::OutputFormatter;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{CommitDto, FileDto};

    #[test]
    fn test_format_commits() {
        let formatter = SimpleFormatter;
        let commits = vec![CommitDto {
            hash: "abcdef12".to_string(),
            author: "Test Author".to_string(),
            email: "test@example.com".to_string(),
            date: "2021-10-18 12:31:30".to_string(),
            message: "Test commit message".to_string(),
        }];

        let result = formatter.format_commits(&commits);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_files() {
        let formatter = SimpleFormatter;
        let files = vec![FileDto {
            path: "src/main.rs".to_string(),
            status: "modified".to_string(),
        }];

        let result = formatter.format_files(&files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_empty_commits() {
        let formatter = SimpleFormatter;
        let result = formatter.format_commits(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_empty_files() {
        let formatter = SimpleFormatter;
        let result = formatter.format_files(&[]);
        assert!(result.is_ok());
    }
}
