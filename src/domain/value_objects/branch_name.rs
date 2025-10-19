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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_branch_name() {
        let branch_name = BranchName::new("main".to_string());
        assert!(branch_name.is_ok());
        assert_eq!(branch_name.unwrap().as_str(), "main");
    }

    #[test]
    fn test_valid_branch_name_with_slash() {
        let branch_name = BranchName::new("feature/new-feature".to_string());
        assert!(branch_name.is_ok());
        assert_eq!(branch_name.unwrap().as_str(), "feature/new-feature");
    }

    #[test]
    fn test_empty_branch_name() {
        let result = BranchName::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BranchNameError::Empty));
    }

    #[test]
    fn test_branch_name_with_double_dot() {
        let result = BranchName::new("feature..bad".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BranchNameError::InvalidCharacters(_)));
    }

    #[test]
    fn test_branch_name_starting_with_dot() {
        let result = BranchName::new(".hidden".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BranchNameError::InvalidCharacters(_)));
    }

    #[test]
    fn test_branch_name_ending_with_dot() {
        let result = BranchName::new("feature.".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BranchNameError::InvalidCharacters(_)));
    }

    #[test]
    fn test_branch_name_display() {
        let branch_name = BranchName::new("main".to_string()).unwrap();
        assert_eq!(format!("{}", branch_name), "main");
    }

    #[test]
    fn test_branch_name_conversion_to_string() {
        let branch_name = BranchName::new("main".to_string()).unwrap();
        let string: String = branch_name.into();
        assert_eq!(string, "main");
    }
}
