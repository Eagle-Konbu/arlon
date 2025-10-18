use crate::infra::output::OutputFormatter;
use crate::service::dto::{CommitDto, FileDto};

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
