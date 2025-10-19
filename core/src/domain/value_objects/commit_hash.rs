use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitHash(String);

#[derive(Debug, thiserror::Error)]
pub enum CommitHashError {
    #[error("Commit hash cannot be empty")]
    Empty,
    #[error("Invalid commit hash format: {0}")]
    InvalidFormat(String),
}

impl CommitHash {
    pub fn new(hash: String) -> Result<Self, CommitHashError> {
        if hash.is_empty() {
            return Err(CommitHashError::Empty);
        }

        if hash.len() != 40 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(CommitHashError::InvalidFormat(hash));
        }

        Ok(Self(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn short(&self) -> &str {
        &self.0[..7]
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CommitHash> for String {
    fn from(commit_hash: CommitHash) -> Self {
        commit_hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_commit_hash() {
        let hash = "abcdef1234567890abcdef1234567890abcdef12".to_string();
        let commit_hash = CommitHash::new(hash.clone());
        assert!(commit_hash.is_ok());
        assert_eq!(commit_hash.unwrap().as_str(), hash);
    }

    #[test]
    fn test_empty_commit_hash() {
        let result = CommitHash::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommitHashError::Empty));
    }

    #[test]
    fn test_invalid_length_commit_hash() {
        let result = CommitHash::new("short".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommitHashError::InvalidFormat(_)));
    }

    #[test]
    fn test_invalid_characters_commit_hash() {
        let result = CommitHash::new("ghijklmnopqrstuvwxyzghijklmnopqrstuvwxy".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommitHashError::InvalidFormat(_)));
    }

    #[test]
    fn test_commit_hash_short() {
        let hash = "abcdef1234567890abcdef1234567890abcdef12".to_string();
        let commit_hash = CommitHash::new(hash).unwrap();
        assert_eq!(commit_hash.short(), "abcdef1");
    }

    #[test]
    fn test_commit_hash_display() {
        let hash = "abcdef1234567890abcdef1234567890abcdef12".to_string();
        let commit_hash = CommitHash::new(hash.clone()).unwrap();
        assert_eq!(format!("{}", commit_hash), hash);
    }
}
