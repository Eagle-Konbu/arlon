use arlon::git::{CommitInfo, FileInfo, GitOperations};

// テスト内でmockallを使用
#[cfg(test)]
use mockall::mock;

// テスト用のモック定義
mock! {
    GitService {}

    impl GitOperations for GitService {
        fn get_commits_not_in_branch(&self, branch_name: &str) -> Result<Vec<CommitInfo>, Box<dyn std::error::Error>>;
        fn get_file_diff_between_branches(&self, branch_name: &str) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>>;
    }
}

// シンプルなテストヘルパー関数
fn create_test_commit(hash: &str, author: &str, message: &str) -> CommitInfo {
    CommitInfo {
        hash: hash.to_string(),
        author: author.to_string(),
        email: format!("{}@example.com", author.to_lowercase()),
        timestamp: 1729180800,
        message: message.to_string(),
    }
}

fn create_test_file(path: &str, status: &str) -> FileInfo {
    FileInfo {
        path: path.to_string(),
        status: status.to_string(),
    }
}

// データ構造の基本テスト
#[test]
fn test_commit_info_creation() {
    let commit = create_test_commit("abc123", "TestAuthor", "Test commit");
    
    assert_eq!(commit.hash, "abc123");
    assert_eq!(commit.author, "TestAuthor");
    assert_eq!(commit.email, "testauthor@example.com");
    assert_eq!(commit.message, "Test commit");
}

#[test]
fn test_file_info_creation() {
    let file = create_test_file("src/main.rs", "modified");
    
    assert_eq!(file.path, "src/main.rs");
    assert_eq!(file.status, "modified");
}

#[test]
fn test_commit_info_equality() {
    let commit1 = create_test_commit("same", "same", "same");
    let commit2 = create_test_commit("same", "same", "same");
    
    assert_eq!(commit1, commit2);
}

#[test]
fn test_file_info_equality() {
    let file1 = create_test_file("same.rs", "same");
    let file2 = create_test_file("same.rs", "same");
    
    assert_eq!(file1, file2);
}

// mockallを使用した軽量なモックテスト
#[test]
fn test_mock_get_commits_success() {
    let mut mock = MockGitService::new();
    
    let expected_commits = vec![
        create_test_commit("commit1", "Author1", "First commit"),
        create_test_commit("commit2", "Author2", "Second commit"),
    ];
    
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("main"))
        .times(1)
        .returning(move |_| Ok(expected_commits.clone()));
    
    let result = mock.get_commits_not_in_branch("main").unwrap();
    
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].hash, "commit1");
    assert_eq!(result[1].hash, "commit2");
}

#[test]
fn test_mock_get_commits_error() {
    let mut mock = MockGitService::new();
    
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("nonexistent"))
        .times(1)
        .returning(|_| Err("Branch not found".into()));
    
    let result = mock.get_commits_not_in_branch("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_mock_get_files_success() {
    let mut mock = MockGitService::new();
    
    let expected_files = vec![
        create_test_file("src/lib.rs", "modified"),
        create_test_file("src/new.rs", "added"),
        create_test_file("src/old.rs", "deleted"),
    ];
    
    mock.expect_get_file_diff_between_branches()
        .with(mockall::predicate::eq("feature"))
        .times(1)
        .returning(move |_| Ok(expected_files.clone()));
    
    let result = mock.get_file_diff_between_branches("feature").unwrap();
    
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].status, "modified");
    assert_eq!(result[1].status, "added");
    assert_eq!(result[2].status, "deleted");
}

#[test]
fn test_mock_get_files_empty() {
    let mut mock = MockGitService::new();
    
    mock.expect_get_file_diff_between_branches()
        .with(mockall::predicate::eq("empty"))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    let result = mock.get_file_diff_between_branches("empty").unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_mock_get_files_error() {
    let mut mock = MockGitService::new();
    
    mock.expect_get_file_diff_between_branches()
        .with(mockall::predicate::eq("error"))
        .times(1)
        .returning(|_| Err("Access denied".into()));
    
    let result = mock.get_file_diff_between_branches("error");
    assert!(result.is_err());
}

#[test]
fn test_mock_multiple_expectations() {
    let mut mock = MockGitService::new();
    
    // 複数の期待値を設定
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("branch1"))
        .times(1)
        .returning(|_| Ok(vec![create_test_commit("hash1", "author1", "msg1")]));
    
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("branch2"))
        .times(1)
        .returning(|_| Ok(vec![create_test_commit("hash2", "author2", "msg2")]));
    
    mock.expect_get_file_diff_between_branches()
        .with(mockall::predicate::eq("branch1"))
        .times(1)
        .returning(|_| Ok(vec![create_test_file("file1.rs", "added")]));
    
    // 実行
    let commits1 = mock.get_commits_not_in_branch("branch1").unwrap();
    let commits2 = mock.get_commits_not_in_branch("branch2").unwrap();
    let files = mock.get_file_diff_between_branches("branch1").unwrap();
    
    assert_eq!(commits1[0].hash, "hash1");
    assert_eq!(commits2[0].hash, "hash2");
    assert_eq!(files[0].path, "file1.rs");
}

// エッジケースとデータバリデーションテスト
#[test]
fn test_unicode_support() {
    let commit = CommitInfo {
        hash: "unicode123".to_string(),
        author: "テスト太郎".to_string(),
        email: "test@日本.jp".to_string(),
        timestamp: 1729180800,
        message: "日本語のコミットメッセージ".to_string(),
    };
    
    assert_eq!(commit.author, "テスト太郎");
    assert_eq!(commit.email, "test@日本.jp");
    assert_eq!(commit.message, "日本語のコミットメッセージ");
}

#[test]
fn test_special_characters_in_paths() {
    let file = create_test_file("path/with spaces & symbols.txt", "modified");
    
    assert!(file.path.contains(" "));
    assert!(file.path.contains("&"));
    assert_eq!(file.status, "modified");
}

#[test]
fn test_negative_timestamp() {
    let commit = CommitInfo {
        hash: "old".to_string(),
        author: "Ancient Author".to_string(),
        email: "ancient@example.com".to_string(),
        timestamp: -86400, // 1 day before epoch
        message: "Very old commit".to_string(),
    };
    
    assert_eq!(commit.timestamp, -86400);
}

#[test]
fn test_empty_values() {
    let commit = CommitInfo {
        hash: "".to_string(),
        author: "".to_string(),
        email: "".to_string(),
        timestamp: 0,
        message: "".to_string(),
    };
    
    assert_eq!(commit.hash, "");
    assert_eq!(commit.author, "");
    assert_eq!(commit.email, "");
    assert_eq!(commit.message, "");
}

#[test]
fn test_complex_file_paths() {
    let file = create_test_file("src/deep/nested/directory/complex_file.rs", "renamed");
    
    assert!(file.path.contains("/"));
    assert!(file.path.ends_with(".rs"));
    assert_eq!(file.status, "renamed");
}

#[test]
fn test_all_file_statuses() {
    let statuses = vec!["added", "deleted", "modified", "renamed", "copied", "untracked"];
    
    for status in statuses {
        let file = create_test_file(&format!("test_{}.rs", status), status);
        assert_eq!(file.status, status);
    }
}

// mockallの高度な機能を示すテスト
#[test]
fn test_mock_with_predicate_any() {
    let mut mock = MockGitService::new();
    
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::always()) // 任意の引数で呼び出し可能
        .times(3)
        .returning(|_| Ok(vec![create_test_commit("any", "any", "any")]));
    
    // 異なる引数で3回呼び出し
    let _result1 = mock.get_commits_not_in_branch("branch1").unwrap();
    let _result2 = mock.get_commits_not_in_branch("branch2").unwrap();
    let _result3 = mock.get_commits_not_in_branch("branch3").unwrap();
}

#[test]
fn test_mock_sequence() {
    let mut mock = MockGitService::new();
    
    // 順番に異なる結果を返す
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("test"))
        .times(1)
        .returning(|_| Ok(vec![create_test_commit("first", "first", "first")]));
        
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("test"))
        .times(1)
        .returning(|_| Ok(vec![create_test_commit("second", "second", "second")]));
    
    let result1 = mock.get_commits_not_in_branch("test").unwrap();
    let result2 = mock.get_commits_not_in_branch("test").unwrap();
    
    assert_eq!(result1[0].hash, "first");
    assert_eq!(result2[0].hash, "second");
}

#[test]
fn test_mock_with_move_data() {
    let mut mock = MockGitService::new();
    
    let commits = vec![create_test_commit("move", "move", "move")];
    
    mock.expect_get_commits_not_in_branch()
        .with(mockall::predicate::eq("move"))
        .times(1)
        .returning(move |_| Ok(commits.clone())); // データをcloneして返す
    
    let result = mock.get_commits_not_in_branch("move").unwrap();
    assert_eq!(result[0].hash, "move");
}