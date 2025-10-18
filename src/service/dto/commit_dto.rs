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
