use arlon::output::format_timestamp;
use arlon::cli::OutputFormat;
use arlon::git::{CommitInfo, FileInfo};
use arlon::output::{print_commits, print_files};

// Test format_timestamp function
#[test]
fn test_format_timestamp() {
    let timestamp = 1729180800;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "2024-10-17 16:00:00");
}

#[test]
fn test_format_timestamp_zero() {
    let timestamp = 0;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "1970-01-01 00:00:00");
}

#[test]
fn test_format_timestamp_negative() {
    let timestamp = -86400;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "1969-12-31 00:00:00");
}

#[test]
fn test_format_timestamp_recent() {
    let timestamp = 1697500000;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "2023-10-16 23:46:40");
}

// Test output functions with mock data
#[test]
fn test_print_commits_simple_format() {
    let commits = vec![
        CommitInfo {
            hash: "abc123".to_string(),
            author: "Test Author".to_string(),
            email: "test@example.com".to_string(),
            timestamp: 1729180800,
            message: "Test commit".to_string(),
        },
        CommitInfo {
            hash: "def456".to_string(),
            author: "Another Author".to_string(),
            email: "another@example.com".to_string(),
            timestamp: 1729267200,
            message: "Another commit".to_string(),
        },
    ];
    
    let result = print_commits(&commits, &OutputFormat::Simple);
    assert!(result.is_ok());
}

#[test]
fn test_print_commits_json_format() {
    let commits = vec![
        CommitInfo {
            hash: "abc123".to_string(),
            author: "Test Author".to_string(),
            email: "test@example.com".to_string(),
            timestamp: 1729180800,
            message: "Test commit".to_string(),
        },
    ];
    
    let result = print_commits(&commits, &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_print_commits_empty_list() {
    let commits = vec![];
    
    let result = print_commits(&commits, &OutputFormat::Simple);
    assert!(result.is_ok());
    
    let result = print_commits(&commits, &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_print_files_simple_format() {
    let files = vec![
        FileInfo {
            path: "src/main.rs".to_string(),
            status: "modified".to_string(),
        },
        FileInfo {
            path: "src/lib.rs".to_string(),
            status: "added".to_string(),
        },
    ];
    
    let result = print_files(&files, &OutputFormat::Simple);
    assert!(result.is_ok());
}

#[test]
fn test_print_files_json_format() {
    let files = vec![
        FileInfo {
            path: "src/test.rs".to_string(),
            status: "deleted".to_string(),
        },
    ];
    
    let result = print_files(&files, &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_print_files_empty_list() {
    let files = vec![];
    
    let result = print_files(&files, &OutputFormat::Simple);
    assert!(result.is_ok());
    
    let result = print_files(&files, &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_commit_info_with_special_characters() {
    let commit = CommitInfo {
        hash: "special123".to_string(),
        author: "Author with émojis 🚀".to_string(),
        email: "test@exämple.com".to_string(),
        timestamp: 1729180800,
        message: "Fix: handle special chars & symbols".to_string(),
    };
    
    let result = print_commits(&vec![commit], &OutputFormat::Json);
    assert!(result.is_ok());
}

#[test]
fn test_file_info_with_complex_paths() {
    let files = vec![
        FileInfo {
            path: "src/deep/nested/directory/file.rs".to_string(),
            status: "modified".to_string(),
        },
        FileInfo {
            path: "file with spaces.txt".to_string(),
            status: "added".to_string(),
        },
    ];
    
    let result = print_files(&files, &OutputFormat::Json);
    assert!(result.is_ok());
}
