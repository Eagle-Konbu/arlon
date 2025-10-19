use crate::application::dto::{CommitDto, FileDto};
use crate::infra::output::OutputFormatter;

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format_commits(&self, commits: &[CommitDto]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(commits)
            .map_err(|e| format!("Failed to serialize commits to JSON: {}", e))?;
        println!("{}", json);
        Ok(())
    }

    fn format_files(&self, files: &[FileDto]) -> Result<(), String> {
        let json = serde_json::to_string_pretty(files)
            .map_err(|e| format!("Failed to serialize files to JSON: {}", e))?;
        println!("{}", json);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{CommitDto, FileDto};

    #[test]
    fn test_format_commits() {
        let formatter = JsonFormatter;
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
        let formatter = JsonFormatter;
        let files = vec![FileDto {
            path: "src/main.rs".to_string(),
            status: "modified".to_string(),
        }];

        let result = formatter.format_files(&files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_empty_commits() {
        let formatter = JsonFormatter;
        let result = formatter.format_commits(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_empty_files() {
        let formatter = JsonFormatter;
        let result = formatter.format_files(&[]);
        assert!(result.is_ok());
    }
}
