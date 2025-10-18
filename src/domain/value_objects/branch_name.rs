use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchName(String);

#[derive(Debug, thiserror::Error)]
pub enum BranchNameError {
    #[error("Branch name cannot be empty")]
    Empty,
    #[error("Branch name contains invalid characters: {0}")]
    InvalidCharacters(String),
}

impl BranchName {
    pub fn new(name: String) -> Result<Self, BranchNameError> {
        if name.is_empty() {
            return Err(BranchNameError::Empty);
        }
        
        // 基本的なブランチ名の検証（Git のルールに基づく）
        if name.contains("..") || name.starts_with('.') || name.ends_with('.') {
            return Err(BranchNameError::InvalidCharacters(name));
        }
        
        Ok(Self(name))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BranchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BranchName> for String {
    fn from(branch_name: BranchName) -> Self {
        branch_name.0
    }
}