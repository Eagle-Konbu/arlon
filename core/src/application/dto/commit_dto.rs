use crate::domain::entities::Commit;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommitDto {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
}

impl From<Commit> for CommitDto {
    fn from(commit: Commit) -> Self {
        Self {
            hash: commit.hash().to_string(),
            author: commit.author().to_string(),
            email: commit.email().to_string(),
            date: commit.formatted_date(),
            message: commit.message().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::CommitHash;

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
    fn test_commit_dto_from_commit() {
        let commit = create_test_commit();
        let dto = CommitDto::from(commit);

        assert_eq!(dto.hash, "abcdef1234567890abcdef1234567890abcdef12");
        assert_eq!(dto.author, "Test Author");
        assert_eq!(dto.email, "test@example.com");
        assert!(dto.date.contains("2021-10-18"));
        assert_eq!(dto.message, "Test commit message");
    }

    #[test]
    fn test_commit_dto_serialization() {
        let dto = CommitDto {
            hash: "abcdef12".to_string(),
            author: "Test Author".to_string(),
            email: "test@example.com".to_string(),
            date: "2021-10-18 12:31:30".to_string(),
            message: "Test commit message".to_string(),
        };

        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("abcdef12"));
        assert!(json.contains("Test Author"));
        assert!(json.contains("test@example.com"));
    }
}
