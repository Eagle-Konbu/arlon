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
        let datetime = DateTime::from_timestamp(self.timestamp, 0).unwrap_or_default();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
