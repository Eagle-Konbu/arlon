use crate::cli::OutputFormat;
use crate::git::{CommitInfo, FileInfo};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct JsonCommitInfo {
    hash: String,
    author: String,
    email: String,
    date: String,
    message: String,
}

#[derive(Serialize)]
struct JsonFileInfo {
    path: String,
    status: String,
}

pub fn print_commits(commits: &[CommitInfo], format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Simple => print_simple(commits),
        OutputFormat::Json => print_json(commits)?,
    }
    Ok(())
}

pub fn print_files(files: &[FileInfo], format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Simple => print_files_simple(files),
        OutputFormat::Json => print_files_json(files)?,
    }
    Ok(())
}

fn print_simple(commits: &[CommitInfo]) {
    for commit in commits {
        let date = format_timestamp(commit.timestamp);
        println!("{} {} {}", commit.hash, date, commit.message);
    }
}

fn print_json(commits: &[CommitInfo]) -> Result<(), Box<dyn std::error::Error>> {
    let json_commits: Vec<JsonCommitInfo> = commits
        .iter()
        .map(|c| JsonCommitInfo {
            hash: c.hash.clone(),
            author: c.author.clone(),
            email: c.email.clone(),
            date: format_timestamp(c.timestamp),
            message: c.message.clone(),
        })
        .collect();
    
    let json = serde_json::to_string_pretty(&json_commits)?;
    println!("{}", json);
    Ok(())
}

pub fn format_timestamp(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| DateTime::<Utc>::default());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn print_files_simple(files: &[FileInfo]) {
    for file in files {
        println!("{} {}", file.status, file.path);
    }
}

fn print_files_json(files: &[FileInfo]) -> Result<(), Box<dyn std::error::Error>> {
    let json_files: Vec<JsonFileInfo> = files
        .iter()
        .map(|f| JsonFileInfo {
            path: f.path.clone(),
            status: f.status.clone(),
        })
        .collect();
    
    let json = serde_json::to_string_pretty(&json_files)?;
    println!("{}", json);
    Ok(())
}
