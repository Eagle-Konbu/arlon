use crate::domain::value_objects::CommitHash;
use chrono::DateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    hash: CommitHash,
    author: String,
    email: String,
    timestamp: i64,
    message: String,
}

impl Commit {
    pub fn new(
        hash: CommitHash,
        author: String,
        email: String,
        timestamp: i64,
        message: String,
    ) -> Self {
        Self {
            hash,
            author,
            email,
            timestamp,
            message,
        }
    }

    pub fn hash(&self) -> &CommitHash {
        &self.hash
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn formatted_date(&self) -> String {
        let datetime = DateTime::from_timestamp(self.timestamp, 0)
            .unwrap_or_default();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_commit() -> Commit {
        let hash = CommitHash::new("abcdef1234567890abcdef1234567890abcdef12".to_string()).unwrap();
        Commit::new(
            hash,
            "Test Author".to_string(),
            "test@example.com".to_string(),
            1634567890, // 2021-10-18 12:31:30 UTC
            "Test commit message".to_string(),
        )
    }

    #[test]
    fn test_commit_creation() {
        let commit = create_test_commit();
        assert_eq!(commit.author(), "Test Author");
        assert_eq!(commit.email(), "test@example.com");
        assert_eq!(commit.timestamp(), 1634567890);
        assert_eq!(commit.message(), "Test commit message");
    }

    #[test]
    fn test_commit_hash() {
        let commit = create_test_commit();
        assert_eq!(commit.hash().as_str(), "abcdef1234567890abcdef1234567890abcdef12");
    }

    #[test]
    fn test_formatted_date() {
        let commit = create_test_commit();
        let formatted = commit.formatted_date();
        // 具体的な時間ではなく、フォーマットが正しいことをテスト
        assert!(formatted.contains("2021-10-18"));
        assert!(formatted.contains(":"));
        assert_eq!(formatted.len(), 19); // "YYYY-MM-DD HH:MM:SS" format
    }

    #[test]
    fn test_commit_equality() {
        let commit1 = create_test_commit();
        let commit2 = create_test_commit();
        assert_eq!(commit1, commit2);
    }

    #[test]
    fn test_commit_clone() {
        let commit1 = create_test_commit();
        let commit2 = commit1.clone();
        assert_eq!(commit1, commit2);
    }
}
