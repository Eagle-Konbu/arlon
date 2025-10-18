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

        // Git ハッシュの基本的な検証（40文字の16進数）
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
